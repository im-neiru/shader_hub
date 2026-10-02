#[cfg(target_arch = "wasm32")]
mod init_web;

use wgpu::Surface;

pub struct Renderer {
    surface: Surface<'static>,
}
