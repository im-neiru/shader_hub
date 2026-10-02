use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, console::info_1};
use wgpu::{
    BackendOptions, Backends, CompositeAlphaMode, InstanceDescriptor, InstanceFlags,
    MemoryBudgetThresholds, PresentMode, SurfaceColorSpace, SurfaceConfiguration, SurfaceTarget,
    TextureFormat, TextureUsages, util::new_instance_with_webgpu_detection,
};

use crate::{errors::GpuInitError, graphics::GpuManager};

thread_local! {
    static GPU_MANAGER: RefCell<Option<Rc<GpuManager>>> =
        const { RefCell::new(None) };
}

impl super::Renderer {
    pub async fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self, GpuInitError> {
        let width = canvas.width().max(1);
        let height = canvas.height().max(1);

        let manager = match GPU_MANAGER.with(|cell| cell.borrow().clone()) {
            Some(manager) => manager,

            None => {
                let instance = new_instance_with_webgpu_detection(InstanceDescriptor {
                    backends: Backends::BROWSER_WEBGPU | Backends::GL,
                    flags: InstanceFlags::default(),
                    memory_budget_thresholds: MemoryBudgetThresholds::default(),
                    backend_options: BackendOptions::default(),
                    display: None,
                })
                .await;

                let surface = instance
                    .create_surface(SurfaceTarget::Canvas(canvas.clone()))
                    .map_err(GpuInitError::Surface)?;

                let manager = Rc::new(GpuManager::new(instance, &surface).await?);

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

        Ok(Self { surface, config })
    }

    #[inline]
    pub(super) fn get_manager() -> Option<Rc<GpuManager>> {
        GPU_MANAGER.with(|cell| cell.borrow().clone())
    }
}
