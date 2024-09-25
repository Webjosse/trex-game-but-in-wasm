mod r#abstract;
mod cactus;
mod ptero;

use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::obstacles::cactus::CactusEntity;
use crate::utils::random_call::MayCaller;
use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;
use web_sys::js_sys::Math;
use crate::gameplay::obstacles::ptero::PteroEntity;

pub struct ObstacleSpawner{
    may_caller: MayCaller,
    to_create: Option<Box<dyn EngineEntity<GameData>>>,
    img_sheet: HtmlImageElement
}

impl ObstacleSpawner {
    pub fn new(img_sheet: &HtmlImageElement) -> ObstacleSpawner {
        ObstacleSpawner{
            may_caller: MayCaller::new(500, 1300, 2000),
            to_create: None,
            img_sheet: img_sheet.clone(),
        }
    }

    fn new_obstacle(&mut self) -> Box<dyn EngineEntity<GameData>>{
        let type_int = (Math::random() * 3.0).floor() as u8;
        match type_int {
            0 => Box::new(CactusEntity::new_tiny(&self.img_sheet)),
            1 => Box::new(CactusEntity::new_big(&self.img_sheet)),
            _ => Box::new(PteroEntity::new(&self.img_sheet))
        }
    }

    fn spawn(&mut self){
        self.to_create = Some(self.new_obstacle())
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
        v
    }
}
impl Drawable for ObstacleSpawner {}

impl EventListener for ObstacleSpawner {}
