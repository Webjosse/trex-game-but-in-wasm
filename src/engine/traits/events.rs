#[derive(Clone)]
#[allow(dead_code)]
pub enum EventSource{
    EXTERNAL,
    ENGINE,
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