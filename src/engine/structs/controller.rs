use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use std::cell::RefCell;
use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

/// A controller is an entity manager, it processes and updates entities at each update. \
/// An [`EngineEntity`] can be deleted or added using [`EngineEntity::is_active`] and [`EngineEntity::entities_to_create`]
pub struct GameController<T: ?Sized>{
    entities: VecDeque<Box<dyn EngineEntity<T>>>,
    data: Box<T>
}

impl <T: ?Sized> GameController<T>{
    pub fn new(initial_data: Box<T>) -> GameController<T> {
        GameController { entities: VecDeque::new(), data: initial_data }
    }

    pub fn add_entity(&mut self, entity: Box<dyn EngineEntity<T>>){
        self.entities.push_back(entity);
    }
}

impl <T: ?Sized> Drawable for GameController<T>{
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        Ok(for drawable in &self.entities {
            drawable.draw(ctx)?;
        })
    }
}

impl <T: ?Sized> Processable<RefCell<u8>> for GameController<T>{
    fn process(&mut self, delta_ms: u16, entity_count:&mut RefCell<u8>) -> Result<(), JsValue> {
        let mut new_entities: VecDeque<Box<dyn EngineEntity<T>>> = VecDeque::new();
        let mut count: u8 = entity_count.get_mut().clone();
        while let Some(mut entity) = self.entities.pop_front(){
            count += 1;
            entity.process(delta_ms, &mut self.data)?;
            let mut entities_to_create = entity.entities_to_create();
            if entity.is_active() {
                new_entities.push_back(entity);
            }
            while let Some(entity_to_create) = entities_to_create.pop_front(){
                new_entities.push_back(entity_to_create);
            }
        }
        self.entities = new_entities;
        entity_count.replace(count);
        Ok(())
    }
}

impl <T: ?Sized> EventListener for GameController<T>{
    fn handle(&mut self, evt: &Event) -> bool {
        for entity in self.entities.iter_mut() {
            if entity.handle(evt) { return true; }
        }
        false
    }
}

impl <T: ?Sized> StaticEntity<RefCell<u8>> for GameController<T>{}