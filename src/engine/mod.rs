pub mod structs;
pub mod traits;
pub mod gamerunner;

use crate::engine::gamerunner::{new_updater, GameRunner};
use crate::engine::structs::config::EngineConfig;
use crate::engine::traits::entity::StaticEntity;
use std::cell::RefCell;

pub fn init(config:EngineConfig, controllers: Vec<Box<dyn StaticEntity<RefCell<u8>>>>) -> GameRunner {
    new_updater(controllers,  config)
}
