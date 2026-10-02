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

        RendererInner::from_canvas(canvas)
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

    #[wasm_bindgen(js_name = render)]
    pub fn render(&mut self, canvas: &HtmlCanvasElement) {
        self.inner.render(canvas);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.inner) };
    }
}
