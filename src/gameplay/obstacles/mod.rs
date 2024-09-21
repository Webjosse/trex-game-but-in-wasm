mod r#abstract;
mod cactus;

use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::gamedata::GameData;
use crate::gameplay::obstacles::cactus::CactusEntity;
use crate::utils::random_call::MayCaller;
use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use web_sys::console::debug_1;
use web_sys::HtmlImageElement;

pub struct ObstacleSpawner{
    may_caller: MayCaller,
    to_create: Option<Box<dyn EngineEntity<GameData>>>,
    img_sheet: HtmlImageElement
}

impl ObstacleSpawner {
    pub fn new(img_sheet: &HtmlImageElement) -> ObstacleSpawner {
        ObstacleSpawner{
            may_caller: MayCaller::new(700, 1300, 2000),
            to_create: None,
            img_sheet: img_sheet.clone(),
        }
    }

    fn spawn(&mut self){
        debug_1(&"Spawn !!".to_string().into());
        self.to_create = Some(Box::new(CactusEntity::new(&self.img_sheet)))
    }
}

impl Processable<GameData> for ObstacleSpawner {
    fn process(&mut self, delta_ms: u16, data: &mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(())}
        if self.may_caller.must_call(delta_ms){
            self.spawn();
        }
        Ok(())
    }
}

impl StaticEntity<GameData> for ObstacleSpawner {}

impl EngineEntity<GameData> for ObstacleSpawner {
    fn entities_to_create(&mut self) -> VecDeque<Box<dyn EngineEntity<GameData>>> {
        let mut v = VecDeque::new();
        if self.to_create.is_none() { return v; }
        let cloud: Option<Box<dyn EngineEntity<GameData>>> = std::mem::replace(&mut self.to_create, None);
        v.push_back(cloud.unwrap());
        debug_1(&format!("V: {:?}", v.len()).into());
        v
    }
}
impl Drawable for ObstacleSpawner {}

impl EventListener for ObstacleSpawner {}
