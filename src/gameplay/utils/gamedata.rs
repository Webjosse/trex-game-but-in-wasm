use crate::engine::structs::rect::Rect;
use wasm_bindgen::JsValue;
use web_sys::console::error_1;
use web_sys::window;

pub struct GameData{
    pub dino_collision: Rect,
    pub pause: bool,
    pub game_over: bool,
    pub speed: f64,
    pub score: u64,
    pub hiscore: u64
}

const HISCORE_KEY: &str = "DINO_HISCORE";

impl GameData {
    fn load_hiscore() -> Result<u64, ()>{
        let window = window().ok_or_else(|| error_1(&"no `window` exists".into()))?;
        let storage = window.local_storage().unwrap().ok_or_else(|| error_1(&"no `localStorage` exists".into()))?;
        let opt_s = storage.get_item(HISCORE_KEY).unwrap_or(None);
        if opt_s.is_some() {
            Ok(opt_s.unwrap().parse::<u64>().unwrap_or(0))
        } else{ Ok(0) }
    }

    pub fn save_hiscore(&self) -> Result<(), JsValue> {
        let window = window().ok_or_else(|| error_1(&"no `window` exists".into())).unwrap();
        let storage = window.local_storage().unwrap().ok_or_else(|| error_1(&"no `localStorage` exists".into())).unwrap();
        storage.set_item(HISCORE_KEY, &format!("{}",self.hiscore))
    }

    pub(crate) fn new() -> GameData {
        GameData{
            dino_collision: Rect{x:0.0,y:0.0,w:0.0,h:0.0},
            pause: true,
            game_over: false,
            speed: 0.0,
            score: 0,
            hiscore: GameData::load_hiscore().unwrap_or(0)
        }
    }
}