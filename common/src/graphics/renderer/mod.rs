#[cfg(target_arch = "wasm32")]
mod init_web;

use wgpu::{Surface, SurfaceConfiguration};

pub struct Renderer {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
}

impl Renderer {
    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        let manager = Self::get_manager().unwrap();

        if width == 0 || height == 0 {
            return;
        }

        if width == self.config.width && height == self.config.height {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(manager.get_device(), &self.config);
    }
}
