use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::binding::EventId;
use crate::gameplay::utils::gamedata::GameData;
use wasm_bindgen::JsValue;

///This struct disables restart events if not game over
pub struct RestartHandler{
    is_pause: bool,
    exit_pause: bool
}

impl RestartHandler{
    pub fn new() -> RestartHandler{ RestartHandler{ is_pause: false, exit_pause: false } }
}

impl Processable<GameData> for RestartHandler {
    fn process(&mut self, _delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if self.exit_pause {
            data.pause = false;
            data.game_over = false;
            self.exit_pause = false;
        }
        self.is_pause = data.pause;
        Ok(())
    }
}

impl EventListener for RestartHandler {
    fn handle(&mut self, evt: &Event) -> bool {
        if evt.id != EventId::RestartPressEvent.as_int(){
            return false
        }
        if self.is_pause {
            self.exit_pause = true;
            return false
        }
        true
    }
}

impl Drawable for RestartHandler {}
impl StaticEntity<GameData> for RestartHandler {}
impl EngineEntity<GameData> for RestartHandler {}