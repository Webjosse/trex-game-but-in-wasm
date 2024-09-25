use crate::engine::structs::controller::GameController;
use crate::engine::traits::entity::StaticEntity;
use crate::gameplay::clouds::CloudsSpawner;
use crate::gameplay::dino::DinoEntity;
use crate::gameplay::logic::restarthandler::RestartHandler;
use crate::gameplay::logic::speed::SpeedEntity;
use crate::gameplay::obstacles::ObstacleSpawner;
use crate::gameplay::ui::background::BgEntity;
use crate::gameplay::ui::daynight::DayNightEntity;
use crate::gameplay::ui::gameover::GameOverEntity;
use crate::gameplay::ui::score::ScoreEntity;
use crate::gameplay::utils::gamedata::GameData;
use crate::gameplay::utils::img::init_img;
use js_sys::Function;
use std::cell::RefCell;
use web_sys::HtmlImageElement;
use crate::gameplay::ui::start::StartEntity;

pub struct GameInitializer{
    img: HtmlImageElement,
    controller: GameController<GameData>,
    invert_callback: Option<Function>,
}

impl GameInitializer{
    pub fn new(invert_callback: Option<Function>) -> GameInitializer{
        let mut this = GameInitializer{
            img: init_img(None, "/assets/sprites.png"),
            controller: GameController::new(Box::new(GameData::new())),
            invert_callback
        };
        this.init();
        this
    }

    fn init(&mut self){
        self.controller.add_entity(Box::new(RestartHandler::new()));
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
        self.controller.add_entity(Box::new(
            GameOverEntity::new(&self.img)
        ));
        self.controller.add_entity(Box::new(ScoreEntity::new(&"10px 'Press Start'")));
        self.controller.add_entity(Box::new(DayNightEntity::new(std::mem::replace(&mut self.invert_callback, None))));
        self.controller.add_entity(Box::new(StartEntity::new()));


    }

    pub fn to_controllers(self) -> Vec<Box<dyn StaticEntity<RefCell<u8>>>> {
        vec!(Box::new(self.controller))
    }
}