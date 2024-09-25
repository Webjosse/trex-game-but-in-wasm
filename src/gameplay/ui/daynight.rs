use js_sys::Function;
use wasm_bindgen::JsValue;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::utils::gamedata::GameData;
use web_sys::js_sys::Math;

pub struct DayNightEntity {
    next_switch: u64,
    fun: Option<Function>,
}

impl DayNightEntity {
    pub fn new(fun: Option<Function>) -> Self{
        DayNightEntity {
            next_switch: 700,
            fun
        }
    }


}

impl Drawable for DayNightEntity {}

impl Processable<GameData> for DayNightEntity {
    fn process(&mut self, _delta_ms: u16, _data: &mut GameData) -> Result<(), JsValue>{
        if _data.game_over { self.next_switch = 700; }
        if _data.score > self.next_switch{
            self.next_switch += (Math::random()*5.0).floor() as u64 * 100;
            if let Some(f) = &self.fun {
                f.call0(&JsValue::undefined()).unwrap();
            }
        }
        Ok(())
    }

}

impl EventListener for DayNightEntity {}

impl StaticEntity<GameData> for DayNightEntity {}

impl EngineEntity<GameData> for DayNightEntity {}