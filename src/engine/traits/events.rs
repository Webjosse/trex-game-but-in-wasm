#[derive(Clone)]
#[allow(dead_code)]
pub enum EventSource{
    /// Comes from outside the game (user inputs)
    EXTERNAL,
    /// Comes from the engine (unused for now)
    ENGINE,
    /// Comes from the game logic
    INTERNAL
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Event{
    pub id: u16,
    pub source: EventSource
}

#[allow(dead_code)]
pub trait EventListener {
    fn handle(&mut self, evt: &Event) -> bool;
}