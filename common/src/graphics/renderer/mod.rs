#[cfg(target_arch = "wasm32")]
mod init_web;

mod resize;

use wgpu::{Surface, SurfaceConfiguration};

pub struct Renderer {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
}
