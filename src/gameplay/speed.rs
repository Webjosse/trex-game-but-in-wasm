use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::gamedata::GameData;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;

pub struct SpeedEntity {}

impl SpeedEntity {
    pub fn new() -> SpeedEntity {
        SpeedEntity {}
    }
}

impl Processable<GameData> for SpeedEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.game_over { data.speed = 0.5}
        if data.pause { return Ok(()) }

        let new_speed = Math::min(2.0,  data.speed + (delta_ms as f64)/5000000.0);
        data.speed = new_speed;
        Ok(())
    }
}

impl EventListener for SpeedEntity {}
impl Drawable for SpeedEntity {}

impl StaticEntity<GameData> for SpeedEntity {}
impl EngineEntity<GameData> for SpeedEntity {}