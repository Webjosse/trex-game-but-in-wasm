use crate::engine::structs::rect::Rect;
use crate::engine::structs::texture::Texture;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::engine::traits::drawable::Drawable;

#[derive(Clone)]
pub struct Sprite {
    texture: Texture,
    rect: Rect
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        let rect = texture.get_rect().clone();
        Sprite { texture, rect: Rect{x:0.0,y:0.0,w:rect.w,h:rect.h} }
    }

    pub fn set_x(&mut self, value: f64) {
        self.rect.x = value;
    }

    pub fn set_y(&mut self, value: f64) {
        self.rect.y = value;
    }

    pub fn set_w(&mut self, value: f64) {
        self.rect.w = value;
    }

    pub fn set_h(&mut self, value: f64) {
        self.rect.h = value;
    }

    #[allow(dead_code)]
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }


    #[allow(dead_code)]
    pub fn set_size(&mut self, w: f64, h: f64) {
        self.set_w(w);
        self.set_h(h);
    }


    #[allow(dead_code)]
    pub fn replace_texture(&mut self, texture: Texture) -> Texture {
        std::mem::replace(&mut self.texture, texture)
    }

    pub fn get_rect(&self) -> &Rect {
        &self.rect
    }
}

impl Drawable for Sprite {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>{
        self.texture.draw(ctx, self.rect.clone())
    }
}