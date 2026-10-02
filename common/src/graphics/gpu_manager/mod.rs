mod init;

use wgpu::{Adapter, Device, Instance, Queue};

pub(crate) struct GpuManager {
    pub(crate) instance: Instance,
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}
