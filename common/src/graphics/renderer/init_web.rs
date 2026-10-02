use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use web_sys::HtmlCanvasElement;
use wgpu::{
    BackendOptions, Backends, InstanceDescriptor, InstanceFlags, MemoryBudgetThresholds,
    SurfaceTarget, util::new_instance_with_webgpu_detection,
};

use crate::{errors::GpuInitError, graphics::GpuManager};

thread_local! {
    static GPU_MANAGER: RefCell<Weak<GpuManager>> = const { RefCell::new(Weak::new()) };
}

impl super::Renderer {
    pub async fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self, GpuInitError> {
        let existing = GPU_MANAGER.with(|c| c.borrow().upgrade());

        let surface = match existing {
            Some(manager) => manager
                .get_instance()
                .create_surface(SurfaceTarget::Canvas(canvas))
                .map_err(GpuInitError::Surface)?,
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
                GPU_MANAGER.with(|c| *c.borrow_mut() = Rc::downgrade(&manager));

                surface
            }
        };

        Ok(Self { surface })
    }
}
