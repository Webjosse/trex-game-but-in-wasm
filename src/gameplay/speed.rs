use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::gameplay::gamedata::GameData;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;

pub struct SpeedEntity {
    reset_speed: bool
}

impl SpeedEntity {
    pub fn new() -> SpeedEntity {
        SpeedEntity { reset_speed: false}
    }
}

impl Processable<GameData> for SpeedEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if self.reset_speed { data.speed = 0.5; self.reset_speed = false; }
        if data.pause { return Ok(()) }

        let new_speed = Math::min(2.0,  data.speed + (delta_ms as f64)/5000000.0);
        data.speed = new_speed;
        Ok(())
    }
}

impl EventListener for SpeedEntity {
    fn handle(&mut self, _evt: &Event) -> bool {
        self.reset_speed = true;
        false
    }
}
impl Drawable for SpeedEntity {}

impl StaticEntity<GameData> for SpeedEntity {}
impl EngineEntity<GameData> for SpeedEntity {}