mod cloudentity;

use crate::engine::structs::rect::Rect;
use crate::engine::structs::sprite::Sprite;
use crate::engine::structs::texture::Texture;
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::{EngineEntity, StaticEntity};
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use crate::gameplay::clouds::cloudentity::CloudEntity;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::{CANVAS_W, FLOOR_LEVEL};
use crate::utils::random_call::poisson_may_call;
use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

#[allow(unused_imports)]
use std::ops::Deref;

pub struct CloudsSpawner{
    to_create: Option<CloudEntity>,
    cloud_sprite: Sprite
}

/// ~ 1 cloud / second
const POISSON_LAMBDA: f64 = 1.0;

const MIN_CLOUD_Y: f64 = FLOOR_LEVEL - 25.0;

fn init_sprite(image_sheet: &HtmlImageElement) -> Sprite{
    let mut sprite = Sprite::new(Texture::new(
        image_sheet.clone(),
        Rect {
            x: 86.0,
            y: 0.0,
            w: 46.0,
            h: 16.0,
        }
    ));
    sprite.set_x(CANVAS_W);
    sprite
}

impl CloudsSpawner {
    pub fn new(image_sheet: &HtmlImageElement) -> CloudsSpawner {
        CloudsSpawner{ to_create: None, cloud_sprite: init_sprite(image_sheet) }
    }

    fn create_cloud(&mut self){
        let mut sprite = self.cloud_sprite.clone();
        sprite.set_y(Math::random()*MIN_CLOUD_Y);
        self.to_create = Some(CloudEntity::new(sprite));
    }
}

impl Drawable for CloudsSpawner {
    fn draw(&self, _ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {Ok(())}
}
impl EventListener for CloudsSpawner {}

impl Processable<GameData> for CloudsSpawner {
    fn process(&mut self, delta_ms: u16, data:&mut GameData) -> Result<(), JsValue> {
        if data.pause { return Ok(())}
        poisson_may_call(POISSON_LAMBDA, delta_ms, || {
            self.create_cloud();
        });
        Ok(())
    }
}

impl StaticEntity<GameData> for CloudsSpawner {}

impl EngineEntity<GameData> for CloudsSpawner {

    fn entities_to_create(&mut self) -> VecDeque<Box<dyn EngineEntity<GameData>>> {
        let mut to_return: VecDeque<Box<dyn EngineEntity<GameData>>> = VecDeque::new();
        if self.to_create.is_none() { return to_return; }
        let cloud: Option<CloudEntity> = std::mem::replace(&mut self.to_create, None);
        to_return.push_back(Box::new(cloud.unwrap()));
        to_return
    }
}