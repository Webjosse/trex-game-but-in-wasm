use wasm_bindgen::JsValue;

/// Can be processed at each update (it's just a guideline, won't be automatically called)
pub trait Processable<T: ?Sized>{
    fn process(&mut self, _delta_ms: u16, _data:&mut T) -> Result<(), JsValue>{ Ok(())}
}