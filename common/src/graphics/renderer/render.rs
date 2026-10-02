use wgpu::{
    Color, CommandEncoderDescriptor, CurrentSurfaceTexture, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

impl super::Renderer {
    pub fn render(&self) {
        let Some(manager) = Self::get_manager() else {
            return;
        };

        let (_, queue) = manager.get_device_and_queue();

        let frame = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) | CurrentSurfaceTexture::Suboptimal(frame) => {
                frame
            }

            CurrentSurfaceTexture::Timeout
            | CurrentSurfaceTexture::Occluded
            | CurrentSurfaceTexture::Outdated
            | CurrentSurfaceTexture::Lost
            | CurrentSurfaceTexture::Validation => return,
        };

        let device = manager.get_device();

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
}
