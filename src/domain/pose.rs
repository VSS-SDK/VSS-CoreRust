#[derive(Clone, Debug)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Pose {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0
        }
    }
}