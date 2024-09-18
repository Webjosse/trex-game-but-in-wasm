pub mod binding;

use crate::engine::traits::events::{Event as EngineEvent, EventListener, EventSource};
use crate::events::binding::EventId;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

pub fn transform_event(event: &Event, listener: &mut dyn EventListener) {
    transform_keyboard_event(event, listener);
}


fn get_keyboard_ids(event: &KeyboardEvent) -> &'static [EventId]{
    match event.type_().to_lowercase().as_str() {
        "keydown" => EventId::from_keycode_down(event.key_code()),
        "keyup" => EventId::from_keycode_up(event.key_code()),
        "keypress" => EventId::from_keycode_press(event.key_code()),
        _ => &[]
    }
}

fn transform_keyboard_event(event: &Event, listener: &mut dyn EventListener) {
    let may_event = event.dyn_ref::<KeyboardEvent>();
    if may_event.is_none() { return; }
    let events: Vec<EngineEvent> = get_keyboard_ids(may_event.unwrap()).iter().map(
        |id: &EventId| EngineEvent { id: id.as_int(), source: EventSource::EXTERNAL }
    ).collect();
    for evt in events {
        listener.handle(&evt);
    }
}