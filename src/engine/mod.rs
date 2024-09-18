pub mod structs;
pub mod traits;
pub mod gamerunner;

use crate::engine::gamerunner::{new_updater, GameRunner};
use crate::engine::structs::config::EngineConfig;
use structs::controller::GameController;

pub fn init(config:EngineConfig, controllers: Vec<GameController>) -> GameRunner {
    new_updater(controllers,  config)
}