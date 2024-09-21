/// Defines a rectangle
#[derive(Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

impl Rect {

    fn aligned(x1: f64, x2: f64, xx1: f64, xx2: f64) -> bool {
        xx1 >= x2 && xx2 >= x1
    }
    pub fn collides(&self, rect:&Rect) -> bool {
        Self::aligned(self.x, rect.x, self.w + self.x, rect.w + rect.x)
        && Self::aligned(self.y, rect.y, self.h + self.y, rect.h + rect.y)
    }
}