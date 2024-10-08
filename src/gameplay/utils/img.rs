use web_sys::HtmlImageElement;
use web_sys::js_sys::Function;

pub fn init_img(onload:Option<&Function>, src:&str ) -> HtmlImageElement {
    let img = HtmlImageElement::new().unwrap();
    img.set_src(src);
    img.set_onload(onload);
    img
}
