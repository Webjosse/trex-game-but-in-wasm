use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::binding::EventId;
use crate::gameplay::gamedata::GameData;
use wasm_bindgen::JsValue;

///This struct disables restart events if not game over
pub struct RestartHandler{
    is_game_over: bool
}

impl RestartHandler{
    pub fn new() -> RestartHandler{ RestartHandler{ is_game_over: false } }
}

impl Processable<GameData> for RestartHandler {
    fn process(&mut self, _delta_ms: u16, _data: &mut GameData) -> Result<(), JsValue> {
        self.is_game_over = _data.game_over;
        Ok(())
    }
}

impl EventListener for RestartHandler {
    fn handle(&mut self, evt: &Event) -> bool {
        evt.id == EventId::RestartPressEvent.as_int() && !self.is_game_over
    }
}

impl Drawable for RestartHandler {}
impl StaticEntity<GameData> for RestartHandler {}
impl EngineEntity<GameData> for RestartHandler {}