use wgpu::{
    BackendOptions, Backends, DeviceDescriptor, ExperimentalFeatures, Features, Instance,
    InstanceDescriptor, InstanceFlags, Limits, MemoryBudgetThresholds, MemoryHints,
    PowerPreference, RequestAdapterOptions, Surface, Trace, wgt::WgpuHasDisplayHandle,
};

use crate::errors::GpuInitError;

impl super::GpuManager {
    pub(crate) async fn new(
        display: Box<dyn WgpuHasDisplayHandle>,
        compatible_surface: &Surface<'static>,
    ) -> Result<Self, GpuInitError> {
        let descriptor = InstanceDescriptor {
            backends: {
                #[cfg(target_arch = "wasm32")]
                {
                    Backends::BROWSER_WEBGPU | Backends::GL
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    Backends::VULKAN
                }
            },

            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            display: Some(display),
        };

        let instance = {
            #[cfg(target_arch = "wasm32")]
            {
                wgpu::util::new_instance_with_webgpu_detection(descriptor).await
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                Instance::new(descriptor)
            }
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(compatible_surface),
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
