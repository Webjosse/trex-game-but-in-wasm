use crate::engine;
use crate::engine::gamerunner::GameRunner;
use crate::engine::structs::config::EngineConfig;
use crate::gameplay::initializer::GameInitializer;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
#[allow(unused_imports)]
use wasm_bindgen::JsCast;

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

/// Creates a [`GameRunner`] from a canvas
#[wasm_bindgen]
pub fn init(canvas: Option<HtmlCanvasElement>) -> Option<GameRunner> {
    let controllers = GameInitializer::new().to_controllers();
    if canvas.is_none() { return None; }
    Some(engine::init(EngineConfig{ canvas: canvas?}, controllers))
}