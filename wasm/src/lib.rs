mod utils;

use std::mem::ManuallyDrop;

use common::graphics::Renderer as RendererInner;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct Renderer {
    inner: ManuallyDrop<RendererInner>,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
        RendererInner::from_canvas(canvas)
            .await
            .map(|inner| Self {
                inner: ManuallyDrop::new(inner),
            })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.inner) };
    }
}
