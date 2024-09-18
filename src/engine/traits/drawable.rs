use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;


pub trait Drawable{
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>;
}