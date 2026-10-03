mod gpu_manager;
mod initial_shader;
mod renderer;

pub(crate) use gpu_manager::GpuManager;
pub use initial_shader::INITIAL_WGSL;
pub use renderer::Renderer;
