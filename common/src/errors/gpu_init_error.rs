use miette::Diagnostic;
use thiserror::Error;

use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError};

#[derive(Debug, Error, Diagnostic)]
pub enum GpuInitError {
    #[error("failed to request GPU adapter")]
    #[diagnostic(
        code(gpu_manager::adapter),
        help(
            "make sure a Vulkan driver is installed (native) or that WebGPU/WebGL2 is available (browser)"
        )
    )]
    Adapter(#[source] RequestAdapterError),

    #[error("failed to request GPU device")]
    #[diagnostic(
        code(gpu_manager::device),
        help("the adapter may not support the required features or limits")
    )]
    Device(#[source] RequestDeviceError),

    #[error("failed to create surface")]
    #[diagnostic(code(gpu_manager::surface))]
    Surface(#[source] CreateSurfaceError),
}
