use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::{Event, EventListener};
use crate::engine::traits::processable::Processable;
use crate::events::binding::EventId;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::FLOOR_LEVEL;
use std::array::from_fn;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;
use web_sys::{CanvasRenderingContext2d, HtmlAudioElement, HtmlImageElement};

fn init_dino_textures(img_sheet: &HtmlImageElement) -> [Texture;5]{
    from_fn(|i| {
        Texture::new(img_sheet.clone(), Rect{x:677.0 + (i as f64 *44.0),y:4.0,w:44.0,h:45.0})
    })
}

fn init_dino_sneak_textures(img_sheet: &HtmlImageElement) -> [Texture;2]{
    from_fn(|i| {
        Texture::new(img_sheet.clone(), Rect{x:897.0 + (i as f64 *59.0),y:19.0,w:59.0,h:30.0})
    })
}

const Y_BASE:f64 = FLOOR_LEVEL - 49.0;

pub struct DinoEntity {
    sprite: Sprite,
    run_texture: Texture,
    sneak_sprite: Sprite,
    sneak_run_texture: Texture,
    sneaking: bool,
    jump: bool,
    on_roof: bool,
    jump_y: f64,
    remaining_ms: u16,
    damage:bool,
    dino_damage: Sprite,
    audio: HtmlAudioElement
}

const JUMP_FORCE: f64 = 12.0;
const TIME_JUMP: f64 = 20.0;

impl DinoEntity {
    pub fn new(image_sheet: &HtmlImageElement, audio: &HtmlAudioElement) -> DinoEntity {
        let [_dino_still, _dino_still2, dino_run, dino_run2, dino_damage] = init_dino_textures(image_sheet);
        let [dino_sneak1, dino_sneak2] = init_dino_sneak_textures(image_sheet);
        let mut sprite = Sprite::new(dino_run);
        sprite.set_y(Y_BASE);
        sprite.set_x(7.0);
        let mut sneak_sprite = Sprite::new(dino_sneak1);
        sneak_sprite.set_y(Y_BASE + 19.0);

        DinoEntity {
            sprite,
            sneak_sprite,
            run_texture: dino_run2,
            sneak_run_texture: dino_sneak2,
            sneaking: false,
            remaining_ms: 1,
            jump: false,
            on_roof: true,
            jump_y: 0.0,
            damage: false,
            dino_damage: Sprite::new(dino_damage),
            audio: audio.clone()
        }
    }

    /// Returns true if time has been reset
    fn process_remaining(&mut self, delta_ms: u16) -> bool{
        if self.remaining_ms > delta_ms{
            self.remaining_ms -= delta_ms;
            return false;
        }
        self.remaining_ms = 300; // 0.3s loop
        true
    }

    fn guard_get_y(&mut self) -> f64 {
        let y = self.sprite.get_rect().y;
        if y > Y_BASE {
            self.sprite.set_y(Y_BASE);
            return Y_BASE;
        }
        return y;
    }
    fn process_jump(&mut self, delta_ms: u16){
        let y = self.guard_get_y();
        self.on_roof = y == Y_BASE;
        if self.on_roof && self.sneaking {return;}
        let delta = delta_ms as f64 / TIME_JUMP;

        if (!self.on_roof) && self.jump_y > -JUMP_FORCE{
            //gravity *2 if sneaking
            self.jump_y -= delta * if self.sneaking { 3.0 } else { 1.0 };
        }
        if self.on_roof && self.jump {
            let _ = self.audio.play();
            self.jump_y = JUMP_FORCE;
        }
        let y_from_floor = Math::min(y - self.jump_y * delta, Y_BASE);
        self.sprite.set_y(y_from_floor);
    }

    fn process_collision(&mut self, data: &mut GameData){
        let d_rect = self.get_drawn_sprite().get_rect();
        // edit rect for collision
        let c_rect = Rect {
            x: d_rect.x+10.0, y: d_rect.y,
            w: d_rect.w-20.0, h: d_rect.h - 4.0
        };
        data.dino_collision = c_rect;
    }

    fn get_drawn_sprite(&self) -> &Sprite{
        if self.damage{
            return &self.dino_damage;
        }
        if self.sneaking && self.on_roof {
            &self.sneak_sprite
        } else {
            &self.sprite
        }
    }

    fn swap_run(&mut self){
        if self.sneaking & self.on_roof {
            self.sneak_run_texture = self.sneak_sprite.replace_texture(self.sneak_run_texture.clone());
        } else {
            self.run_texture = self.sprite.replace_texture(self.run_texture.clone());
        }
    }
}

impl Drawable for DinoEntity {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.get_drawn_sprite().draw(ctx)
    }
}

impl Processable<GameData> for DinoEntity {

    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        self.damage = data.game_over;
        if self.damage { self.dino_damage.set_pos(self.sprite.get_rect().x, self.sprite.get_rect().y); }
        if data.pause { return Ok(())}
        self.process_jump(delta_ms);
        if self.process_remaining(delta_ms){ self.swap_run(); }
        self.process_collision(data);
        Ok(())
    }
}


impl EventListener for DinoEntity {
    fn handle(&mut self, _evt: &Event) -> bool {
        match EventId::from_int(_evt.id) {
            EventId::JumpDownEvent => {self.jump=true; true}
            EventId::JumpUpEvent => {self.jump=false; true}
            EventId::SneakDownEvent => { self.sneaking = true; true }
            EventId::SneakUpEvent => { self.sneaking = false; true }
            _ => false
        }
    }
}

impl StaticEntity<GameData> for DinoEntity {}
impl EngineEntity<GameData> for DinoEntity {}