#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[allow(dead_code)]
    pub fn new(_x: f64, _y: f64) -> Vec2 {
        Vec2 {
            x: _x,
            y: _y,
        }
    }
}
