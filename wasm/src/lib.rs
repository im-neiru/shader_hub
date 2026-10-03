mod utils;

use std::mem::ManuallyDrop;

use common::graphics::Renderer as RendererInner;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub struct Renderer {
    inner: ManuallyDrop<RendererInner>,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
        set_panic_hook();

        RendererInner::from_canvas(canvas, common::graphics::INITIAL_WGSL)
            .await
            .map(|inner| Self {
                inner: ManuallyDrop::new(inner),
            })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = resize)]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.inner.resize(width, height);
    }

    #[wasm_bindgen(js_name = orbit)]
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.inner.orbit(delta_yaw, delta_pitch);
    }

    #[wasm_bindgen(js_name = render)]
    pub fn render(&mut self, canvas: &HtmlCanvasElement, time_seconds: f32) {
        self.inner.render(canvas, time_seconds);
    }

    #[wasm_bindgen(js_name = setWgsl)]
    pub async fn set_wgsl(&mut self, wgsl: String) -> Result<(), JsValue> {
        self.inner
            .set_wgsl(&wgsl)
            .await
            .map_err(|error| JsValue::from_str(&error))
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.inner) };
    }
}

#[wasm_bindgen(js_name = getDefaultWgsl)]
pub fn get_default_wgsl() -> String {
    common::graphics::INITIAL_WGSL.to_string()
}
