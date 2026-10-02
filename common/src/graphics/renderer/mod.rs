#[cfg(target_arch = "wasm32")]
mod init_web;

mod camera;
mod render;
mod resize;
mod world;

use wgpu::{Surface, SurfaceConfiguration};

pub struct Renderer {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    camera: camera::Camera,
    world: world::World,
}
