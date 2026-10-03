use core::f32::consts::FRAC_PI_2;

use glam::{
    Mat4, Vec3,
    camera::rh::{proj::vulkan::perspective, view::look_at_mat4},
};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

pub(crate) struct Camera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
    fov_y: f32,
    aspect: f32,
    near: f32,
    far: f32,
    dirty: bool,
    buffer: Buffer,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
}

impl Camera {
    const UP: Vec3 = Vec3::Y;
    const MIN_DISTANCE: f32 = 0.04;
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

    pub(crate) fn new(device: &Device, aspect: f32) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("camera buffer"),
            contents: bytemuck::bytes_of(&Mat4::IDENTITY),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera bind group layout"),
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

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            target: Vec3::ZERO,
            yaw: 0.6,
            pitch: 0.35,
            distance: 5.0,
            fov_y: 45f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
            dirty: true,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub(crate) fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    pub(crate) fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub(crate) fn eye(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();

        self.target + self.distance * Vec3::new(cos_pitch * sin_yaw, sin_pitch, cos_pitch * cos_yaw)
    }

    pub(crate) fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-Self::PITCH_LIMIT, Self::PITCH_LIMIT);
        self.dirty = true;
    }

    pub(crate) fn pan(&mut self, dx: f32, dy: f32) {
        let forward = (self.target - self.eye()).normalize();
        let right = forward.cross(Self::UP).normalize();
        let up = right.cross(forward);

        let scale = 2.0 * self.distance * (self.fov_y * 0.5).tan();

        self.target += (-right * dx + up * dy) * scale;
        self.dirty = true;
    }

    pub(crate) fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance * (1.0 + delta)).max(Self::MIN_DISTANCE);
        self.dirty = true;
    }

    pub(crate) fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.dirty = true;
    }

    pub(crate) fn set_distance(&mut self, distance: f32) {
        self.distance = distance.max(Self::MIN_DISTANCE);
        self.dirty = true;
    }

    pub(crate) fn set_fov_y(&mut self, fov_y: f32) {
        self.fov_y = fov_y;
        self.dirty = true;
    }

    pub(crate) fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.dirty = true;
    }

    pub(crate) fn set_clip(&mut self, near: f32, far: f32) {
        self.near = near;
        self.far = far;
        self.dirty = true;
    }

    pub(crate) fn look_at(&mut self, pos: Vec3, at: Vec3) {
        let offset = pos - at;

        self.target = at;
        self.distance = offset.length().max(Self::MIN_DISTANCE);
        self.yaw = offset.x.atan2(offset.z);
        self.pitch = (offset.y / self.distance)
            .clamp(-1.0, 1.0)
            .asin()
            .clamp(-Self::PITCH_LIMIT, Self::PITCH_LIMIT);

        self.dirty = true;
    }

    fn view_proj(&self) -> Mat4 {
        let view = look_at_mat4(self.eye(), self.target, Self::UP);
        let proj = perspective(self.fov_y, self.aspect, self.near, self.far);

        proj * view
    }

    pub(crate) fn update(&mut self, queue: &Queue) {
        if !self.dirty {
            return;
        }

        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.view_proj()));

        self.dirty = false;
    }
}

impl super::Renderer {
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.camera.orbit(delta_yaw, delta_pitch);
    }
}
