use protos::debug;
use protos::state;
use protos::control;

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

impl From<Pose> for debug::Pose {
    fn from(pose: Pose) -> Self {
        let mut _self = debug::Pose::new();

        _self.set_x(pose.x);
        _self.set_y(pose.y);
        _self.set_yaw(pose.angle);

        _self
    }
}

impl From<Pose> for state::Pose {
    fn from(pose: Pose) -> Self {
        let mut _self = state::Pose::new();

        _self.set_x(pose.x);
        _self.set_y(pose.y);
        _self.set_yaw(pose.angle);

        _self
    }
}

impl From<Pose> for control::Pose {
    fn from(pose: Pose) -> Self {
        let mut _self = control::Pose::new();

        _self.set_x(pose.x);
        _self.set_y(pose.y);
        _self.set_yaw(pose.angle);

        _self
    }
}