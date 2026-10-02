use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use web_sys::HtmlCanvasElement;
use wgpu::{
    BackendOptions, Backends, CompositeAlphaMode, InstanceDescriptor, InstanceFlags,
    MemoryBudgetThresholds, PresentMode, SurfaceColorSpace, SurfaceConfiguration, SurfaceTarget,
    TextureFormat, TextureUsages, util::new_instance_with_webgpu_detection,
};

use crate::{errors::GpuInitError, graphics::GpuManager};

thread_local! {
    static GPU_MANAGER: RefCell<Weak<GpuManager>> =
        const { RefCell::new(Weak::new()) };
}

impl super::Renderer {
    pub async fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self, GpuInitError> {
        let existing = GPU_MANAGER.with(|cell| cell.borrow().upgrade());

        let width = canvas.width().max(1);
        let height = canvas.height().max(1);

        let (manager, surface) = match existing {
            Some(manager) => {
                let surface = manager
                    .get_instance()
                    .create_surface(SurfaceTarget::Canvas(canvas))
                    .map_err(GpuInitError::Surface)?;

                (manager, surface)
            }

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
                    .create_surface(SurfaceTarget::Canvas(canvas))
                    .map_err(GpuInitError::Surface)?;

                let manager = Rc::new(GpuManager::new(instance, &surface).await?);

                GPU_MANAGER.with(|cell| {
                    *cell.borrow_mut() = Rc::downgrade(&manager);
                });

                (manager, surface)
            }
        };

        let capabilities = surface.get_capabilities(manager.get_adapter());

        let preferred_format = [TextureFormat::Bgra8UnormSrgb, TextureFormat::Rgba8UnormSrgb]
            .into_iter()
            .find(|&format| {
                capabilities
                    .color_spaces(format)
                    .contains(SurfaceColorSpace::Srgb.to_color_spaces().unwrap())
            })
            .unwrap_or_else(|| {
                capabilities
                    .formats
                    .first()
                    .copied()
                    .unwrap_or(TextureFormat::Bgra8UnormSrgb)
            });

        let color_space = if capabilities
            .color_spaces(preferred_format)
            .contains(SurfaceColorSpace::Srgb.to_color_spaces().unwrap())
        {
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

        surface.configure(&manager.get_device(), &config);

        Ok(Self { surface, config })
    }

    #[inline]
    pub(super) fn get_manager() -> Option<Rc<GpuManager>> {
        GPU_MANAGER.with(|cell| cell.borrow().upgrade())
    }
}
