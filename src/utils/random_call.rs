use web_sys::js_sys::Math;

/// Using a poisson distribution
pub fn poisson_may_call(lambda: f64,delta_ms: u16, mut callback: impl FnMut()) {
    let delta_s = delta_ms as f64 / 1000.0;
    let probability = 1.0 - (-(lambda * delta_s)).exp();

    if Math::random() < probability {
        callback();
    }
}

///Equiprobability in the range
pub struct MayCaller{
    min_ms: u16,
    ms_range: u16,
    curr_ms: u16
}
impl MayCaller{
    pub fn new(min_ms: u16, max_ms: u16, first_ms: u16) -> MayCaller {
        MayCaller{min_ms, ms_range: max_ms - min_ms, curr_ms: first_ms}
    }
    fn reset_curr(&mut self){
        self.curr_ms = (Math::random() * self.ms_range as f64).floor() as u16 + self.min_ms;
    }
    pub fn must_call(&mut self, delta_ms: u16) -> bool {
        if delta_ms > self.curr_ms{
            self.reset_curr();
            return true;
        }
        self.curr_ms -= delta_ms;
        false
    }
}