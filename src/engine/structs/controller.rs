use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::EngineEntity;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;

pub struct GameController{
    entities: Vec<Box<dyn EngineEntity>>,
}

impl GameController{
    pub fn new() -> Self {
        GameController { entities: Vec::new() }
    }

    pub fn add_entity(&mut self, entity: Box<dyn EngineEntity>){
        self.entities.push(entity);
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
        let mut new_entities: Vec<Box<&dyn EngineEntity>> = Vec::new();
        for entity in &mut self.entities {
            (*entity).process(delta_ms)?;
            if entity.is_active() {
                new_entities.push(Box::new(&**entity));
            }
            new_entities.extend(entity.to_create());
        };
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
