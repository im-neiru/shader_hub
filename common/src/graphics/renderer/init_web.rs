use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::{
    HtmlCanvasElement,
    console::{info_1, warn_1},
};
use wgpu::{
    Adapter, BackendOptions, Backends, CompositeAlphaMode, Instance, InstanceDescriptor,
    InstanceFlags, MemoryBudgetThresholds, PresentMode, SurfaceColorSpace, SurfaceConfiguration,
    SurfaceTarget, TextureFormat, TextureFormatFeatureFlags, TextureUsages,
    util::is_browser_webgpu_supported,
};

use super::{camera::Camera, world::World};
use crate::{errors::GpuInitError, graphics::GpuManager};

thread_local! {
    static GPU_MANAGER: RefCell<Option<Rc<GpuManager>>> =
        const { RefCell::new(None) };
}

impl super::Renderer {
    pub async fn from_canvas(canvas: HtmlCanvasElement, wgsl: &str) -> Result<Self, GpuInitError> {
        let width = canvas.width().max(1);
        let height = canvas.height().max(1);

        let manager = match GPU_MANAGER.with(|cell| cell.borrow().clone()) {
            Some(manager) => manager,

            None => {
                let manager = if is_browser_webgpu_supported().await {
                    match Self::create_gpu_manager(&canvas, Backends::BROWSER_WEBGPU).await {
                        Ok(manager) => manager,
                        Err(error) => {
                            warn_1(&JsValue::from_str(&format!(
                                "WebGPU initialization failed; trying WebGL2: {error}"
                            )));
                            Self::create_gpu_manager(&canvas, Backends::GL).await?
                        }
                    }
                } else {
                    Self::create_gpu_manager(&canvas, Backends::GL).await?
                };

                GPU_MANAGER.with(|cell| {
                    *cell.borrow_mut() = Some(manager.clone());
                });

                manager
            }
        };

        let surface = manager
            .get_instance()
            .create_surface(SurfaceTarget::Canvas(canvas))
            .map_err(GpuInitError::Surface)?;

        let capabilities = surface.get_capabilities(manager.get_adapter());

        let srgb = SurfaceColorSpace::Srgb.to_color_spaces().unwrap();

        let preferred_format = [TextureFormat::Bgra8UnormSrgb, TextureFormat::Rgba8UnormSrgb]
            .into_iter()
            .find(|&format| capabilities.color_spaces(format).contains(srgb))
            .or_else(|| capabilities.formats.first().copied())
            .unwrap();

        let color_space = if capabilities.color_spaces(preferred_format).contains(srgb) {
            SurfaceColorSpace::Srgb
        } else {
            SurfaceColorSpace::Auto
        };

        let present_mode = [
            PresentMode::Mailbox,
            PresentMode::FifoRelaxed,
            PresentMode::Fifo,
        ]
        .into_iter()
        .find(|mode| capabilities.present_modes.contains(mode))
        .unwrap_or(PresentMode::Fifo);

        let alpha_mode = capabilities
            .alpha_modes
            .iter()
            .copied()
            .find(|mode| *mode == CompositeAlphaMode::Opaque)
            .unwrap_or(CompositeAlphaMode::Auto);

        info_1(&JsValue::from_str(&format!(
            "Surface format: {preferred_format:?}"
        )));

        info_1(&JsValue::from_str(&format!("Color space: {color_space:?}")));

        info_1(&JsValue::from_str(&format!(
            "Present mode: {present_mode:?}"
        )));

        info_1(&JsValue::from_str(&format!("Alpha mode: {alpha_mode:?}")));

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            width,
            height,
            present_mode,
            color_space,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: Vec::new(),
        };

        surface.configure(manager.get_device(), &config);

        let camera = Camera::new(manager.get_device(), width as f32 / height as f32);
        let msaa_samples = Self::get_max_msaa_samples(manager.get_adapter(), preferred_format);
        let msaa_texture = Self::create_msaa_texture(
            manager.get_device(),
            preferred_format,
            width,
            height,
            msaa_samples,
        );
        let world = World::new(
            manager.get_device(),
            &camera,
            preferred_format,
            wgsl,
            msaa_samples,
        );

        Ok(Self {
            surface,
            config,
            msaa_texture,
            camera,
            world,
            msaa_samples,
        })
    }

    #[inline]
    pub(super) fn get_manager() -> Option<Rc<GpuManager>> {
        GPU_MANAGER.with(|cell| cell.borrow().clone())
    }

    pub async fn set_wgsl(&mut self, wgsl: &str) -> Result<(), String> {
        let Some(manager) = Self::get_manager() else {
            return Err("GPU manager is unavailable".to_owned());
        };

        self.world
            .set_wgsl(
                manager.get_device(),
                self.config.format,
                wgsl,
                self.msaa_samples,
            )
            .await
    }

    #[inline]
    fn get_max_msaa_samples(adapter: &Adapter, format: TextureFormat) -> u8 {
        let features = adapter.get_texture_format_features(format);
        let flags = features.flags;

        if flags.contains(TextureFormatFeatureFlags::MULTISAMPLE_X16) {
            16
        } else if flags.contains(TextureFormatFeatureFlags::MULTISAMPLE_X8) {
            8
        } else if flags.contains(TextureFormatFeatureFlags::MULTISAMPLE_X4) {
            4
        } else if flags.contains(TextureFormatFeatureFlags::MULTISAMPLE_X2) {
            2
        } else {
            1
        }
    }

    async fn create_gpu_manager(
        canvas: &HtmlCanvasElement,
        backend: Backends,
    ) -> Result<Rc<GpuManager>, GpuInitError> {
        let instance = Instance::new(InstanceDescriptor {
            backends: backend,
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            display: None,
        });

        let surface = instance
            .create_surface(SurfaceTarget::Canvas(canvas.clone()))
            .map_err(GpuInitError::Surface)?;

        GpuManager::new(instance, &surface).await.map(Rc::new)
    }
}
