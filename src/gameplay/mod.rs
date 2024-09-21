use crate::engine;
use crate::engine::gamerunner::GameRunner;
use crate::engine::structs::config::EngineConfig;
use crate::gameplay::initializer::GameInitializer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::console::error_1;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod dino;
mod img;
mod background;
mod speed;
mod clouds;
mod initializer;
mod obstacles;
mod gamedata;

pub const CANVAS_H: f64 = 160.0;

pub const CANVAS_W: f64 = 514.0;

/// floor level: y position of the floor
const FLOOR_LEVEL: f64 = CANVAS_H - 11.0;

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
    let controllers = GameInitializer::new().to_controllers();
    let ctx = get_context(canvas);
    if ctx.is_err() { return None; }
    Some(engine::init(EngineConfig{ ctx: ctx.unwrap()}, controllers))
}