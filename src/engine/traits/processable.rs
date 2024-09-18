use wasm_bindgen::JsValue;

/// Can be processed at each update (it's just a guideline, won't be automatically called)
pub trait Processable{
    fn process(&mut self, delta_ms: u16) -> Result<(), JsValue>;
}