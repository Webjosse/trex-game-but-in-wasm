use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

/// Can be drawn
pub trait Drawable{
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>;
}