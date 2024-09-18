use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;
use std::collections::VecDeque;

/// Contains a game logic
pub trait EngineEntity: Drawable + Processable + EventListener{
    fn is_active(&self) -> bool { true }
    fn entities_to_create(&mut self) -> VecDeque<Box<dyn EngineEntity>>{
        VecDeque::new()
    }
}