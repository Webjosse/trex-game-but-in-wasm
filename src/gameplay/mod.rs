use std::rc::Rc;
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
use crate::gameplay::clouds::CloudsSpawner;
use crate::gameplay::speed::{init_speed_factor, SpeedEntity};

mod dino;
mod img;
mod background;
mod speed;
mod clouds;

pub const CANVAS_H: f64 = 160.0;

pub const CANVAS_W: f64 = 514.0;

/// floor level: y position of the floor
const FLOOR_LEVEL: f64 = CANVAS_H - 11.0;


fn init_controllers() -> Vec<GameController>{
    let factor = init_speed_factor();
    let sheet_img = get_sheet_img();
    let mut controller= GameController::new();
    controller.add_entity(Box::new(
        BgEntity::new(&sheet_img, Rc::clone(&factor))
    ));
    controller.add_entity(Box::new(
        CloudsSpawner::new(Rc::clone(&factor), &sheet_img)
    ));
    controller.add_entity(Box::new(
        DinoEntity::new(&sheet_img)
    ));
    controller.add_entity(Box::new(
        SpeedEntity::new(factor)
    ));
    vec!(controller)
}


fn get_context(canvas: Option<HtmlCanvasElement>) -> Result<CanvasRenderingContext2d, ()> {
    if canvas.is_none() {
        error_1(&"Error from WASM attach: Canvas is null !".into());
        return Err(());
    }
    Ok(canvas.unwrap().get_context("2d").unwrap().unwrap().dyn_into().unwrap())
}

/// Creates a [`GameRunner`] from a canvas
#[wasm_bindgen]
pub fn init(canvas: Option<HtmlCanvasElement>) -> Option<GameRunner> {
    let controllers = init_controllers();
    let ctx = get_context(canvas);
    if ctx.is_err() { return None; }
    Some(engine::init(EngineConfig{ ctx: ctx.unwrap()}, controllers))
}
