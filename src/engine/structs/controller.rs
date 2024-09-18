use std::collections::VecDeque;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::EngineEntity;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;

pub struct GameController{
    entities: VecDeque<Box<dyn EngineEntity>>,
}

impl GameController{
    pub fn new() -> GameController {
        GameController { entities: VecDeque::new() }
    }

    pub fn add_entity(&mut self, entity: Box<dyn EngineEntity>){
        self.entities.push_back(entity);
    }
}

impl Drawable for GameController{
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        Ok(for drawable in &self.entities {
            drawable.draw(ctx)?;
        })
    }
}

impl Processable for GameController{
    fn process(&mut self, delta_ms: u16) -> Result<(), JsValue> {
        let mut new_entities: VecDeque<Box<dyn EngineEntity>> = VecDeque::new();
        while let Some(mut entity) = self.entities.pop_front(){
            entity.process(delta_ms)?;
            let mut entities_to_create = entity.entities_to_create();
            if entity.is_active() {
                new_entities.push_back(entity);
            }
            while let Some(entity_to_create) = entities_to_create.pop_front(){
                new_entities.push_back(entity_to_create);
            }
        }
        self.entities = new_entities;
        Ok(())
    }
}

impl EventListener for GameController{
    fn handle(&mut self, evt: &Event) -> bool {
        for entity in self.entities.iter_mut() {
            if entity.handle(evt) { return true; }
        }
        false
    }
}