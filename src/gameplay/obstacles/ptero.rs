use std::array::from_fn;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::obstacles::r#abstract::AbstractObstacle;
use crate::gameplay::{CANVAS_W, FLOOR_LEVEL};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlAudioElement, HtmlImageElement};
use web_sys::js_sys::Math;
use crate::engine::structs::rect::Rect;
use crate::events::binding::EventId;

const MAX_Y: f64 = FLOOR_LEVEL - 40.0;

fn init_ptero(image_sheet: &HtmlImageElement) -> [Texture;2]{
    from_fn(|i| {
        Texture::new(image_sheet.clone(), Rect{x:(136 + 46 * i) as f64 ,y:4.0,h:36.0,w:42.0})
    })
}

pub struct PteroEntity{
    obstacle: AbstractObstacle,
    fly_texture: Texture,
    remaining_ms: u16
}

impl PteroEntity{
    pub fn new(image_sheet: &HtmlImageElement, audio: &HtmlAudioElement) -> PteroEntity{
        let [text1, text2] = init_ptero(image_sheet);
        let mut sprite = Sprite::new(text1);
        sprite.set_x(CANVAS_W);
        sprite.set_y((Math::random() * MAX_Y).floor());
        let mut rect = sprite.get_rect().clone();
        rect.x += 4.0;
        rect.w -= 6.0;
        rect.y += 7.0;
        rect.h -= 14.0;
        PteroEntity{
            obstacle: AbstractObstacle::new(sprite, rect, audio),
            fly_texture: text2,
            remaining_ms: 0
        }
    }

    fn process_fly(&mut self, delta_ms: u16){
        if delta_ms > self.remaining_ms{
            self.remaining_ms += 500;
            self.fly_texture = self.obstacle.replace_texture(self.fly_texture.clone());
        }
        self.remaining_ms -= delta_ms;
    }
}
impl Drawable for PteroEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>{
        self.obstacle.draw(ctx)
    }
}

impl Processable<GameData> for PteroEntity {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(())}
        self.process_fly(delta_ms);
        self.obstacle.approach(delta_ms, data.speed);
        self.obstacle.process_collision(data);
        Ok(())
    }
}

impl EventListener for PteroEntity {
    fn handle(&mut self, evt: &Event) -> bool {
        if evt.id == EventId::RestartPressEvent.as_int() {
            self.obstacle.kill();
        }
        false
    }
}

impl StaticEntity<GameData> for PteroEntity {}

impl EngineEntity<GameData> for PteroEntity {
    fn is_active(&self) -> bool {
        self.obstacle.is_alive()
    }
}