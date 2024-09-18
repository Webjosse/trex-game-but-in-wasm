use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::engine::structs::rect::Rect;

#[derive(Clone)]
pub struct Texture {
    img: HtmlImageElement,
    src_rect: Rect
}

impl Texture {
    pub fn new(img: HtmlImageElement, src_rect: Rect) -> Self {
        Texture { img, src_rect }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, dst_rect: Rect) -> Result<(), JsValue> {
        if !self.img.complete() { return Ok(()) }

        ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            self.img.as_ref(),
            self.src_rect.x,
            self.src_rect.y,
            self.src_rect.w,
            self.src_rect.h,
            dst_rect.x,
            dst_rect.y,
            dst_rect.w,
            dst_rect.h
        )
    }
    pub fn get_rect(&self) -> &Rect {
        &self.src_rect
    }
}