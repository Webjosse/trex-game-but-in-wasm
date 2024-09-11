use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::console::debug_1;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn attach(canvas: &HtmlCanvasElement) {
    let ctx : CanvasRenderingContext2d =
        canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();

    ctx.rect(30.0f64,30.0f64,30.0f64,30.0f64);
    ctx.fill();
    debug_1(&"Hello World".into());

}
