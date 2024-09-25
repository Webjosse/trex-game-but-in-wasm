use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::{CANVAS_H, CANVAS_W, FLOOR_LEVEL};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct BgEntity{
    /// The bg sprite
    sprite: Sprite,
    /// A clone of the bg sprite, used when reaching the end of the texture
    sprite_2: Sprite
}

const BG_W: f64 = 1204.0;
const X_END: f64 = - BG_W;

const X_START: f64 = CANVAS_W + X_END;

impl BgEntity {
    pub fn new<'a>(image_sheet: &HtmlImageElement) -> BgEntity {
        let mut sprite = Sprite::new(
            Texture::new(image_sheet.clone(), Rect{x:0.0,y:52.0,w:1204.0,h:16.0})
        );
        sprite.set_size(BG_W, 16.0);
        sprite.set_y(FLOOR_LEVEL - 11.0);

        let sprite_2 = sprite.clone();

        BgEntity { sprite, sprite_2 }
    }
}

impl Drawable for BgEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.clear_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
        if self.sprite.get_rect().x < X_START { self.sprite_2.draw(&ctx)?; }
        self.sprite.draw(&ctx)
    }
}
impl Processable<GameData> for BgEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(()) }
        let x = self.sprite.get_rect().x - delta_ms as f64 * data.speed;
        self.sprite.set_x(x);
        self.sprite_2.set_x(x + BG_W);
        // If sprite cannot be displayed, we swap it with sprite_2
        if x < X_END { std::mem::swap(&mut self.sprite, &mut self.sprite_2);}
        Ok(())
    }
}

impl EventListener for BgEntity {}

impl StaticEntity<GameData> for BgEntity {}
impl EngineEntity<GameData> for BgEntity {}