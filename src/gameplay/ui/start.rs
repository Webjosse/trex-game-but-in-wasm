use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::binding::EventId;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::{CANVAS_H, CANVAS_W};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const CAN_W_INT: u16 = 512;


pub struct StartEntity{
    active: bool,
    initial: bool,
    w: u16
}

impl StartEntity{
    pub fn new() -> StartEntity{
        StartEntity{
            active: true, initial: true, w: 64
        }
    }
}

impl StaticEntity<GameData> for StartEntity {}
impl Drawable for StartEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.active {
            ctx.clear_rect( self.w as f64, 0.0, CANVAS_W - self.w as f64, CANVAS_H);
        }
        Ok(())
    }
}
impl Processable<GameData> for StartEntity {
    fn process(&mut self, _delta_ms: u16, _data: &mut GameData) -> Result<(), JsValue> {
        if !self.initial {
            self.w += _delta_ms * 3;
            if self.w > CAN_W_INT {
                self.active = false;
            }
        }
        Ok(())
    }
}
impl EventListener for StartEntity {
    fn handle(&mut self, evt: &Event) -> bool {
        if evt.id == EventId::RestartPressEvent.as_int() {
            self.initial = false;
        }
        false
    }
}
impl EngineEntity<GameData> for StartEntity {
    fn is_active(&self) -> bool { self.active }
}