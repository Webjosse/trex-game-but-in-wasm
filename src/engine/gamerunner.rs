use crate::engine::structs::config::EngineConfig;
#[allow(unused_imports)]
use crate::engine::traits::drawable::Drawable;
use crate::engine::traits::entity::StaticEntity;
use crate::engine::traits::events::{Event as EngineEvent, EventListener};
#[allow(unused_imports)]
use crate::engine::traits::processable::Processable;
use crate::events::transform_event;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::debug_1;
use web_sys::{window, CanvasRenderingContext2d, Event, HtmlCanvasElement};

#[wasm_bindgen]
pub struct GameRunner {
    final_ctx: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    controllers: Vec<Box<dyn StaticEntity<RefCell<u8>>>>
}

fn create_canvas(ref_canvas:&HtmlCanvasElement) -> HtmlCanvasElement {
    let window = window().expect("No global `window`");
    let document = window.document().expect("No global `window.document`");
    let canvas = document
        .create_element("canvas")
        .expect("Could not document.createElement a <canvas>")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Could not document.createElement a HtmlCanvasElement");
    canvas.set_width(ref_canvas.width());
    canvas.set_height(ref_canvas.height());
    canvas
}

pub fn new_updater(controllers: Vec<Box<dyn StaticEntity<RefCell<u8>>>>, config: EngineConfig) -> GameRunner {
    let canvas = create_canvas(&config.canvas);
    let final_ctx = config.canvas.get_context(&"2d")
        .unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    let ctx = canvas.get_context(&"2d")
        .unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    GameRunner { canvas , ctx, controllers, final_ctx }
}

#[wasm_bindgen]
impl GameRunner {
    /// Updates the canvas
    /// * `delta` the milliseconds elapsed since last update (maximum 80ms please)
    pub fn update(&mut self, delta: u16) -> Result<(), JsValue>{
        let mut entity_count = RefCell::new(0u8);
        for controller in &mut self.controllers.iter_mut() {
            let err = controller.process(delta, &mut entity_count).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        debug_1(&format!("Updated {} entities", entity_count.get_mut()).into());
        for controller in self.controllers.iter() {
            let err = controller.draw(&self.ctx).err();
            if err.is_some() { return Err(err.unwrap()); }
        }
        self.final_ctx.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        self.final_ctx.draw_image_with_html_canvas_element(&self.canvas, 0.0, 0.0)
    }

    /// Sends an event from the DOM
    pub fn send(&mut self, evt: Option<Event>){
        if evt.is_some(){
            transform_event(&evt.unwrap(), self);
        }
    }
}

impl EventListener for GameRunner {
    fn handle(&mut self, evt: &EngineEvent) -> bool {
        for controller in self.controllers.iter_mut() {
            if controller.handle(evt) { return true; }
        }
        false
    }
}