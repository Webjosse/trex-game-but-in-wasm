use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::traits::drawable::Drawable;
use wasm_bindgen::JsValue;
use web_sys::console::debug_1;
use web_sys::CanvasRenderingContext2d;

pub struct AbstractObstacle{
    sprite: Sprite,
    rect: Rect
}

impl AbstractObstacle {
    pub fn new(sprite: Sprite, rect: Rect) -> AbstractObstacle {
        AbstractObstacle{ sprite, rect }
    }

    pub fn approach(&mut self, delta_ms: u16, speed:f64){
        let min_x = (delta_ms as f64) * 0.7 * speed;
        self.sprite.set_x(self.sprite.get_rect().x - min_x);
        self.rect.x -= min_x;
        debug_1(&format!("Approaching by {} : {}", min_x, self.sprite.get_rect().x).into());
    }

    pub fn is_alive(&self) -> bool{
        let rect = self.sprite.get_rect();
        (rect.x + rect.w) > 0.0
    }
}

impl Drawable for AbstractObstacle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.sprite.draw(ctx)
    }
}