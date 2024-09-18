use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::EngineEntity;
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;
use web_sys::CanvasRenderingContext2d;

pub fn init_speed_factor() -> Rc<RefCell<f64>> {
    let speed_factor: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.5));
    speed_factor
}

pub struct SpeedEntity {
    speed_factor: Rc<RefCell<f64>>
}

impl SpeedEntity {
    pub fn new(speed_factor: Rc<RefCell<f64>>) -> SpeedEntity {
        SpeedEntity { speed_factor }
    }
}

impl Drawable for SpeedEntity {
    fn draw(&self, _ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {Ok(())}
}

impl Processable for SpeedEntity {
    fn process(&mut self, delta_ms: u16) -> Result<(), JsValue> {
        let mut factor: RefMut<f64> = self.speed_factor.borrow_mut();

        let new_speed = Math::min(2.0,  factor.clone() + (delta_ms as f64)/5000000.0);
        *factor = new_speed;
        Ok(())
    }
}

impl EventListener for SpeedEntity {
    fn handle(&mut self, _evt: &Event) -> bool {false}
}

impl EngineEntity for SpeedEntity {}