use wasm_bindgen::JsValue;

pub trait Processable{
    fn process(&mut self, delta_ms: u16) -> Result<(), JsValue>;
}