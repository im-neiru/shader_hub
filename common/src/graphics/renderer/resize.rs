use wasm_bindgen::JsValue;
use web_sys::console::{debug_1, info_1};

impl super::Renderer {
    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        let Some(manager) = Self::get_manager() else {
            debug_1(&JsValue::from("Resize failed"));

            return;
        };

        if width == 0 || height == 0 {
            return;
        }

        if width == self.config.width && height == self.config.height {
            return;
        }

        info_1(&JsValue::from_str(&format!("Resizing to {width}x{height}")));

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(manager.get_device(), &self.config);
        self.msaa_texture = Self::create_msaa_texture(
            manager.get_device(),
            self.config.format,
            width,
            height,
            self.msaa_samples,
        );

        self.camera.set_aspect(width as f32 / height as f32);
    }
}
