#[derive(Clone, Debug)]
pub struct Robot {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_angle: f32
}

impl Robot {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_angle: 0.0
        }
    }
}

