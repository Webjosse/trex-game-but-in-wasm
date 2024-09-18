use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::EngineEntity;
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::engine::structs::sprite::Sprite;

pub struct CloudEntity{
    sprite: Sprite,
    speed_factor: f64
}

impl CloudEntity{
   pub fn new(sprite: Sprite, speed_factor: f64) -> CloudEntity{
       CloudEntity{ sprite, speed_factor }
   }
}

impl Drawable for CloudEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.sprite.draw(ctx)
    }
}

impl Processable for CloudEntity {
    fn process(&mut self, delta_ms: u16) -> Result<(), JsValue> {
        self.sprite.set_x(self.sprite.get_rect().x - delta_ms as f64 * self.speed_factor);
        Ok(())
    }
}

impl EventListener for CloudEntity {
    fn handle(&mut self, _evt: &Event) -> bool {false}
}

impl EngineEntity for CloudEntity {
    fn is_active(&self) -> bool {
        let rect = self.sprite.get_rect();
        rect.x > -rect.w
    }
}
