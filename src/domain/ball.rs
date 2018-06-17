#[derive(Clone, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32
}

impl Ball {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed_x: 0.0,
            speed_y: 0.0
        }
    }
}

