#[cfg(target_arch = "wasm32")]
mod init_web;

mod camera;
mod render;
mod resize;
mod world;

use wgpu::{Surface, SurfaceConfiguration, Texture};

pub struct Renderer {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    msaa_texture: Option<Texture>,
    camera: camera::Camera,
    world: world::World,
    msaa_samples: u8,
}

impl Renderer {
    pub(super) fn create_msaa_texture(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        sample_count: u8,
    ) -> Option<Texture> {
        (sample_count > 1).then(|| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MSAA color target"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: sample_count as u32,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
        })
    }
}
