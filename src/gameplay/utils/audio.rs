use web_sys::HtmlAudioElement;

pub fn init_audio() -> [web_sys::HtmlAudioElement; 3]{
    [
        HtmlAudioElement::new_with_src("assets/sounds/jump.wav").unwrap(),
        HtmlAudioElement::new_with_src("assets/sounds/die.wav").unwrap(),
        HtmlAudioElement::new_with_src("assets/sounds/point.wav").unwrap(),
    ]
}