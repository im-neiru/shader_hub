use wgpu::{
    Color, CommandEncoderDescriptor, CurrentSurfaceTexture, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

pub trait SurfaceSize {
    fn size(&self) -> (u32, u32);
}

impl super::Renderer {
    pub fn render(&mut self, surface: &impl SurfaceSize) {
        let Some(manager) = Self::get_manager() else {
            return;
        };

        let Some(frame) = self.get_current_frame(surface) else {
            return;
        };

        let device = manager.get_device();
        let (_, queue) = manager.get_device_and_queue();

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.8,
                            g: 0.8,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        queue.submit(std::iter::once(encoder.finish()));
        queue.present(frame);
    }

    #[inline]
    fn get_current_frame(&mut self, surface: &impl SurfaceSize) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) | CurrentSurfaceTexture::Suboptimal(frame) => {
                Some(frame)
            }

            CurrentSurfaceTexture::Outdated => {
                let (width, height) = surface.size();

                self.resize(width, height);

                match self.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(frame)
                    | CurrentSurfaceTexture::Suboptimal(frame) => Some(frame),

                    CurrentSurfaceTexture::Timeout
                    | CurrentSurfaceTexture::Occluded
                    | CurrentSurfaceTexture::Outdated
                    | CurrentSurfaceTexture::Lost
                    | CurrentSurfaceTexture::Validation => None,
                }
            }

            CurrentSurfaceTexture::Timeout
            | CurrentSurfaceTexture::Occluded
            | CurrentSurfaceTexture::Lost
            | CurrentSurfaceTexture::Validation => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl SurfaceSize for web_sys::HtmlCanvasElement {
    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}
