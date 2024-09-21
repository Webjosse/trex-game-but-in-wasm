use crate::engine::structs::config::EngineConfig;
#[allow(unused_imports)]
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::StaticEntity;
use crate::engine::traits::events::{Event as EngineEvent, EventListener};
#[allow(unused_imports)]
use crate::engine::traits::processable::Processable;
use crate::events::transform_event;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console::debug_1;
use web_sys::{CanvasRenderingContext2d, Event};

#[wasm_bindgen]
pub struct GameRunner {
    ctx: CanvasRenderingContext2d,
    controllers: Vec<Box<dyn StaticEntity<RefCell<u8>>>>
}

pub fn new_updater(controllers: Vec<Box<dyn StaticEntity<RefCell<u8>>>>, config: EngineConfig) -> GameRunner {
    GameRunner { ctx: config.ctx.clone(), controllers }
}

#[wasm_bindgen]
impl GameRunner {
    /// Updates the canvas
    /// * `delta` the milliseconds elapsed since last update (maximum 80ms please)
    pub fn update(&mut self, delta: u16) -> Result<(), JsValue>{
        let mut entity_count = RefCell::new(0u8);
        for controller in &mut self.controllers.iter_mut() {
            let err = controller.process(delta, &mut entity_count).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        debug_1(&format!("Updated {} entities", entity_count.get_mut()).into());
        for controller in self.controllers.iter() {
            let err = controller.draw(&self.ctx).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        Ok(())
    }

    /// Sends an event from the DOM
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