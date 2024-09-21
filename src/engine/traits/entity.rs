use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use std::collections::VecDeque;


pub trait StaticEntity<T: ?Sized>: Drawable + Processable<T> + EventListener{}

/// Contains a game logic
pub trait EngineEntity<T: ?Sized>: StaticEntity<T> {
    fn is_active(&self) -> bool { true }
    fn entities_to_create(&mut self) -> VecDeque<Box<dyn EngineEntity<T>>>{
        VecDeque::new()
    }
}