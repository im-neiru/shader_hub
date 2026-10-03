mod init;

use wgpu::{Adapter, Device, Instance, Queue};

pub(crate) struct GpuManager {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl GpuManager {
    #[inline]
    pub(crate) const fn get_instance(&self) -> &Instance {
        &self.instance
    }

    #[inline]
    pub(crate) const fn get_adapter(&self) -> &Adapter {
        &self.adapter
    }

    #[inline]
    pub(crate) const fn get_device(&self) -> &Device {
        &self.device
    }

    #[inline]
    pub(crate) const fn get_device_and_queue(&self) -> (&Device, &Queue) {
        (&self.device, &self.queue)
    }
}
