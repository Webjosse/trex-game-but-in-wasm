use crate::engine::structs::sprite::Sprite;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::gamedata::GameData;
use wasm_bindgen::JsValue;
use web_sys::console::debug_1;
use web_sys::CanvasRenderingContext2d;

pub struct CloudEntity{
    sprite: Sprite
}

impl CloudEntity{
   pub fn new(sprite: Sprite) -> CloudEntity{
       CloudEntity{ sprite }
   }
}

impl Drawable for CloudEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        debug_1(&format!("CLOUD DRAW : {} {}", self.sprite.get_rect().x.floor(), self.sprite.get_rect().y.floor()).into());
        self.sprite.draw(ctx)
    }
}

impl Processable<GameData> for CloudEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(());}
        self.sprite.set_x(self.sprite.get_rect().x - delta_ms as f64 * data.speed * 0.7);
        Ok(())
    }
}

impl EventListener for CloudEntity {}

impl StaticEntity<GameData> for CloudEntity {}

impl EngineEntity<GameData> for CloudEntity {
    fn is_active(&self) -> bool {
        let rect = self.sprite.get_rect();
        rect.x > -rect.w
    }
}
