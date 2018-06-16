#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

