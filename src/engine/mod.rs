use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::console::{debug_1, debug_2, error_1};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[wasm_bindgen]
pub fn attach(canvas: Option<HtmlCanvasElement>) {
    if canvas.is_none() {
        error_1(&"Error from WASM attach: Canvas is null !".into());
        return;
    }
    let ctx : CanvasRenderingContext2d =
        canvas.unwrap().get_context("2d").unwrap().unwrap().dyn_into().unwrap();

    let sprites = HtmlImageElement::new().unwrap();
    sprites.set_src("/assets/sprites.png");
    let spr = sprites.clone();

    let closure = Closure::<dyn FnMut()>::new(move || {
        //drawImage(img, 0, 0, halfWidth, img.height, 0, 0, halfWidth, img.height);
        ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &spr, 0.0, 52.0, 755.0, 16.0, 0.0, 156.0, 2408.0, 32.0 ).unwrap();
    });

    sprites.set_onload(Some(&closure.as_ref().unchecked_ref()));
    closure.forget();

}
