use crate::engine::structs::rect::Rect;

pub struct GameData{
    pub dino_collision: Rect,
    pub pause: bool,
    pub game_over: bool,
    pub speed: f64
}

impl GameData {
    pub(crate) fn new() -> GameData {
        GameData{
            dino_collision: Rect{x:0.0,y:0.0,w:0.0,h:0.0},
            pause: false,
            game_over: false,
            speed: 0.5
        }
    }
}