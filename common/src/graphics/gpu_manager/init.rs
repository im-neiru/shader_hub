use wgpu::{
    DeviceDescriptor, ExperimentalFeatures, Features, Instance, Limits, MemoryHints,
    PowerPreference, RequestAdapterOptions, Surface, Trace,
};

use crate::errors::GpuInitError;

impl super::GpuManager {
    pub(crate) async fn new<'s>(
        instance: Instance,
        surface: &Surface<'s>,
    ) -> Result<Self, GpuInitError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
                apply_limit_buckets: false,
            })
            .await
            .map_err(GpuInitError::Adapter)?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("GpuManager Device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
                experimental_features: ExperimentalFeatures::disabled(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await
            .map_err(GpuInitError::Device)?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
