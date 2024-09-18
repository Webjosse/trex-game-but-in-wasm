use web_sys::js_sys::Math;

pub fn poisson_may_call(lambda: f64,delta_ms: u16, mut callback: impl FnMut()) {
    let delta_s = delta_ms as f64 / 1000.0;
    let probability = 1.0 - (-(lambda * delta_s)).exp();

    if Math::random() < probability {
        callback();
    }
}