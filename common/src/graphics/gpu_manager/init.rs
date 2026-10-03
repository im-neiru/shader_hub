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

        let adapter_info = adapter.get_info();
        let required_limits = if adapter_info.backend == wgpu::Backend::Gl {
            Limits::downlevel_webgl2_defaults()
        } else {
            Limits::default()
        };

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("GpuManager Device"),
                required_features: Features::empty(),
                required_limits,
                experimental_features: ExperimentalFeatures::disabled(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await
            .map_err(GpuInitError::Device)?;

        #[cfg(target_arch = "wasm32")]
        web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(&format!(
            "GPU adapter: {} ({:?})",
            adapter_info.name, adapter_info.backend
        )));

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
