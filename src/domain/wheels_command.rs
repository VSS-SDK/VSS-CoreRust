#[derive(Clone)]
pub struct WheelsCommand {
    pub left_vel: f32,
    pub right_vel: f32,
}

impl WheelsCommand {
    pub fn new() -> Self {
        Self {
            left_vel: 0.0,
            right_vel: 0.0
        }
    }
}