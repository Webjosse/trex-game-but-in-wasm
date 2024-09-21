use crate::engine::structs::controller::GameController;
use crate::engine::traits::entity::StaticEntity;
use crate::gameplay::background::BgEntity;
use crate::gameplay::clouds::CloudsSpawner;
use crate::gameplay::dino::DinoEntity;
use crate::gameplay::gamedata::GameData;
use crate::gameplay::img::init_img;
use crate::gameplay::obstacles::ObstacleSpawner;
use crate::gameplay::speed::SpeedEntity;
use std::cell::RefCell;
use web_sys::HtmlImageElement;

pub struct GameInitializer{
    img: HtmlImageElement,
    controller: GameController<GameData>
}

impl GameInitializer{
    pub fn new() -> GameInitializer{
        let mut this = GameInitializer{
            img: init_img(None, "/assets/sprites.png"),
            controller: GameController::new(Box::new(GameData::new()))
        };
        this.init();
        this
    }

    fn init(&mut self){
        self.controller.add_entity(Box::new(
            BgEntity::new(&self.img)
        ));
        self.controller.add_entity(Box::new(
            CloudsSpawner::new(&self.img)
        ));
        self.controller.add_entity(Box::new(
            DinoEntity::new(&self.img)
        ));
        self.controller.add_entity(Box::new(
            ObstacleSpawner::new(&self.img)
        ));
        self.controller.add_entity(Box::new(
            SpeedEntity::new()
        ));

    }

    pub fn to_controllers(self) -> Vec<Box<dyn StaticEntity<RefCell<u8>>>> {
        vec!(Box::new(self.controller))
    }
}