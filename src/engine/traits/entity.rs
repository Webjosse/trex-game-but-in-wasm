use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::events::EventListener;
use crate::engine::traits::processable::Processable;

pub trait EngineEntity: Drawable + Processable + EventListener{
    fn is_active(&self) -> bool;
    fn to_create(&self) -> Vec<Box<&dyn EngineEntity>>;

}
