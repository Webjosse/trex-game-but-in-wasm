use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::binding::EventId;
use crate::gameplay::gamedata::GameData;
use crate::gameplay::CANVAS_W;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

/// This entity displays the Game Over message
pub struct GameOverEntity {
    sprite_game_over: Sprite,
    sprite_restart: Sprite,
    restart: bool,
    show: bool
}

const GO_W: f64 = 191.0;
const RS_W: f64 = 34.0;

impl GameOverEntity {
    pub fn new(img_sheet: &HtmlImageElement) -> GameOverEntity {
        let go_texture = Texture::new(img_sheet.clone(), Rect {x: 484.0, y:15.0, w: GO_W, h: 11.0 });
        let mut sprite_game_over = Sprite::new(go_texture.clone());
        sprite_game_over.set_x((CANVAS_W - GO_W) / 2.0);
        sprite_game_over.set_y(35.0);
        let restart_texture = Texture::new(img_sheet.clone(), Rect {x: 3.0, y:3.0, w: RS_W, h: 30.0 });
        let mut sprite_restart = Sprite::new(restart_texture.clone());
        sprite_restart.set_x((CANVAS_W - RS_W) / 2.0);
        sprite_restart.set_y(75.0);
        GameOverEntity{
            sprite_restart, sprite_game_over, restart: false, show: false
        }
    }
}

impl Drawable for GameOverEntity {
    fn draw(&self, _ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.show {
            self.sprite_game_over.draw(_ctx)?;
            self.sprite_restart.draw(_ctx)
        }
        else { Ok(()) }
    }
}

impl Processable<GameData> for GameOverEntity {
    fn process(&mut self, _delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.game_over && self.restart{
            data.game_over = false;
            data.pause = false;
        }
        self.restart = false;
        self.show = data.game_over;
        Ok(())
    }
}

impl EventListener for GameOverEntity {
    fn handle(&mut self, evt: &Event) -> bool {
        match EventId::from_int(evt.id){
            EventId::RestartPressEvent => {
                self.restart = true;
                true
            }
            _ => false
        }
    }
}

impl StaticEntity<GameData> for GameOverEntity {} impl EngineEntity<GameData> for GameOverEntity {}