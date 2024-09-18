use crate::engine::structs::config::EngineConfig;
use crate::engine::structs::controller::GameController;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::events::{Event as EngineEvent, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::transform_event;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Event};

#[wasm_bindgen]
pub struct GameRunner {
    ctx: CanvasRenderingContext2d,
    controllers: Vec<GameController>
}

pub fn new_updater(controllers: Vec<GameController>, config: EngineConfig) -> GameRunner {
    GameRunner { ctx: config.ctx.clone(), controllers }
}

#[wasm_bindgen]
impl GameRunner {
    pub fn update(&mut self, delta: u16) -> Result<(), JsValue>{
        for controller in &mut self.controllers.iter_mut() {
            let err = controller.process(delta).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        for controller in self.controllers.iter() {
            let err = controller.draw(&self.ctx).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        Ok(())
    }

    pub fn send(&mut self, evt: Option<Event>){
        if evt.is_some(){
            transform_event(&evt.unwrap(), self);
        }
    }
}

impl EventListener for GameRunner {
    fn handle(&mut self, evt: &EngineEvent) -> bool {
        for controller in self.controllers.iter_mut() {
            if controller.handle(evt) { return true; }
        }
        false
    }
}