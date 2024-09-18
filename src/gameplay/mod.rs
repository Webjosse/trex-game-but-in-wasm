use crate::engine;
use crate::engine::gamerunner::GameRunner;
use crate::engine::structs::config::EngineConfig;
use crate::engine::structs::controller::GameController;
use crate::gameplay::dino::DinoEntity;
use crate::gameplay::img::get_sheet_img;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::console::error_1;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::gameplay::background::BgEntity;

mod dino;
mod img;
mod background;

pub const CANVAS_H: f64 = 192.0;

pub const CANVAS_W: f64 = 514.0;


fn init_controllers() -> Vec<GameController>{
    let sheet_img = get_sheet_img();
    let mut controller= GameController::new();
    controller.add_entity(Box::new(BgEntity::new(&sheet_img)));
    controller.add_entity(Box::new(DinoEntity::new(&sheet_img)));
    vec!(controller)
}

fn get_context(canvas: Option<HtmlCanvasElement>) -> Result<CanvasRenderingContext2d, ()> {
    if canvas.is_none() {
        error_1(&"Error from WASM attach: Canvas is null !".into());
        return Err(());
    }
    Ok(canvas.unwrap().get_context("2d").unwrap().unwrap().dyn_into().unwrap())
}


#[wasm_bindgen]
pub fn init(canvas: Option<HtmlCanvasElement>) -> Option<GameRunner> {
    let controllers = init_controllers();
    let ctx = get_context(canvas);
    if ctx.is_err() { return None; }
    Some(engine::init(EngineConfig{ ctx: ctx.unwrap()}, controllers))
}