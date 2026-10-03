use glam::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferUsages,
    ColorTargetState, Device, ErrorFilter, FragmentState, FrontFace, IndexFormat, MultisampleState,
    PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState,
    PrimitiveTopology, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat, VertexAttribute,
    VertexBufferLayout, VertexState, VertexStepMode,
};

use super::camera::Camera;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: Vec3,
    color: Vec3,
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];

    fn new(position: Vec3, color: Vec3) -> Self {
        Self { position, color }
    }

    fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub(crate) struct World {
    vertex: Buffer,
    index: Buffer,
    index_count: u32,
    time_buffer: Buffer,
    time_bind_group: BindGroup,
    pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline,
}

impl World {
    pub fn new(
        device: &Device,
        camera: &Camera,
        surface_format: TextureFormat,
        wgsl: &str,
        msaa_samples: u8,
    ) -> Self {
        let vertices = [
            // Front
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::X),
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::Y),
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::Z),
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::ONE),
            // Back
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::X),
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::Z),
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::ONE),
            // Left
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 0.5, 0.0)),
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.5, 1.0, 0.0)),
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(0.0, 0.5, 1.0)),
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 0.0, 0.5)),
            // Right
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.5, 0.0, 1.0)),
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.5)),
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, 0.5, 0.5)),
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.5, 1.0)),
            // Top
            Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::X),
            Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::Y),
            Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::Z),
            Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::ONE),
            // Bottom
            Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.5, 0.0, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 0.5, 0.0)),
            Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 0.5)),
            Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.5, 0.5, 0.0)),
        ];

        let indices: &[u16] = &[
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Left
            8, 9, 10, 10, 11, 8, // Right
            12, 13, 14, 14, 15, 12, // Top
            16, 17, 18, 18, 19, 16, // Bottom
            20, 21, 22, 22, 23, 20,
        ];

        let vertex = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("world vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("world index buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        let time_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("world time uniform buffer"),
            contents: bytemuck::bytes_of(&[0.0f32; 4]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let time_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("world time bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let time_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("world time bind group"),
            layout: &time_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("world pipeline layout"),
            bind_group_layouts: &[
                Some(camera.bind_group_layout()),
                Some(&time_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let pipeline =
            Self::create_pipeline(device, &pipeline_layout, surface_format, wgsl, msaa_samples);

        Self {
            vertex,
            index,
            index_count: indices.len() as u32,
            time_buffer,
            time_bind_group,
            pipeline_layout,
            pipeline,
        }
    }

    pub fn update(&self, queue: &Queue, time_seconds: f32) {
        queue.write_buffer(
            &self.time_buffer,
            0,
            bytemuck::bytes_of(&[time_seconds, 0.0, 0.0, 0.0]),
        );
    }

    pub async fn set_wgsl(
        &mut self,
        device: &Device,
        surface_format: TextureFormat,
        wgsl: &str,
        msaa_samples: u8,
    ) -> Result<(), String> {
        let error_scope = device.push_error_scope(ErrorFilter::Validation);

        let pipeline = Self::create_pipeline(
            device,
            &self.pipeline_layout,
            surface_format,
            wgsl,
            msaa_samples,
        );

        if let Some(error) = error_scope.pop().await {
            return Err(error.to_string());
        }

        self.pipeline = pipeline;

        Ok(())
    }

    fn create_pipeline(
        device: &Device,
        pipeline_layout: &PipelineLayout,
        surface_format: TextureFormat,
        wgsl: &str,
        msaa_samples: u8,
    ) -> RenderPipeline {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("world shader"),
            source: ShaderSource::Wgsl(wgsl.into()),
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("world pipeline"),
            layout: Some(pipeline_layout),

            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Some(Vertex::layout())],
            },

            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },

            depth_stencil: None,

            multisample: MultisampleState {
                count: msaa_samples as u32,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            multiview_mask: None,
            cache: None,
        })
    }

    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>, camera: &'a Camera) {
        pass.set_pipeline(&self.pipeline);

        pass.set_bind_group(0, camera.bind_group(), &[]);
        pass.set_bind_group(1, &self.time_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex.slice(..));
        pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint16);

        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
