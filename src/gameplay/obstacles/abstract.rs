use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::gameplay::utils::gamedata::GameData;
use wasm_bindgen::JsValue;
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
        let min_x = (delta_ms as f64) * speed;
        self.sprite.set_x(self.sprite.get_rect().x - min_x);
        self.rect.x -= min_x;
    }

    pub fn is_alive(&self) -> bool{
        let rect = self.sprite.get_rect();
        (rect.x + rect.w) > 0.0
    }

    pub fn kill(&mut self){
        self.sprite.set_x(-10.0-self.sprite.get_rect().w);
        self.rect = self.sprite.get_rect().clone();
    }

    pub fn process_collision(&self, data: &mut GameData){
        if data.dino_collision.collides(&self.rect) {
            data.pause = true;
            data.game_over = true;
        }
    }

    pub fn replace_texture(&mut self, texture: Texture) -> Texture {
        self.sprite.replace_texture(texture)
    }
}

impl Drawable for AbstractObstacle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.sprite.draw(ctx)//?;
        //ctx.set_fill_style(&"rgba(0,0,0,0.5)".into());
        //Ok(ctx.fill_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h))
    }
}