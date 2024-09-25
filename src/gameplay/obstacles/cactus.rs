use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::gamedata::GameData;
use crate::gameplay::obstacles::r#abstract::AbstractObstacle;
use crate::gameplay::{CANVAS_W, FLOOR_LEVEL};
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use web_sys::console::debug_1;

fn init_a_small_cactus(image_sheet: &HtmlImageElement) -> Texture{
    let i = (Math::random() * 3.0).floor() as u8;
    let x: f64 = (i * (i+1) * 17) as f64/2.0 + 228.0;
    debug_1(&format!("RECT => x: {}, w: {}", x, 17.0 * (i+1) as f64).into());
    Texture::new(image_sheet.clone(), Rect{x ,y:0.0,h:37.0,w:17.0* (i+1) as f64})
}

fn init_a_big_cactus(image_sheet: &HtmlImageElement) -> Texture{
    let i = (Math::random() * 3.0).floor() as u8;
    let x: f64 = ((i * (i+1) * 25)as f64)/2.0 + 332.0;
    Texture::new(image_sheet.clone(), Rect{x ,y:3.0,h:47.0,w:25.0* (i+1) as f64})
}

pub struct CactusEntity{
    obstacle: AbstractObstacle
}

impl CactusEntity{
    fn new(texture: Texture) -> CactusEntity{
        let mut sprite = Sprite::new(texture);
        sprite.set_x(CANVAS_W);
        sprite.set_y(FLOOR_LEVEL - sprite.get_rect().h);
        let mut rect = sprite.get_rect().clone();
        rect.x += 5.0;
        rect.w -= 10.0;
        rect.y += 5.0;
        rect.h -= 5.0;
        CactusEntity{
            obstacle: AbstractObstacle::new(sprite, rect)
        }
    }

    pub fn new_tiny(image_sheet: &HtmlImageElement) -> CactusEntity{
        let texture = init_a_small_cactus(image_sheet);
        CactusEntity::new(texture)
    }


    pub fn new_big(image_sheet: &HtmlImageElement) -> CactusEntity{
        let texture = init_a_big_cactus(image_sheet);
        debug_1(&"BIG CACTUS".into());
        CactusEntity::new(texture)
    }
}
impl Drawable for CactusEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>{
        self.obstacle.draw(ctx)
    }
}

impl Processable<GameData> for CactusEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(())}
        self.obstacle.approach(delta_ms, data.speed);
        self.obstacle.process_collision(data);
        Ok(())
    }
}

impl StaticEntity<GameData> for CactusEntity {}

impl EngineEntity<GameData> for CactusEntity{
    fn is_active(&self) -> bool {
        self.obstacle.is_alive()
    }
}

impl EventListener for CactusEntity {}