use protos::debug;
use protos::state;
use protos::control;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use domain::constants::{MIN_ANGLE_VALUE, MAX_ANGLE_VALUE};
use traits::new_random_trait::NewRandom;
use traits::is_zero_trait::IsZero;

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

    pub fn new_with(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle
        }
    }
}

impl NewRandom for Pose {
    fn new_random() -> Self {
        Self {
            x: thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X),
            y: thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y),
            angle: thread_rng().gen_range(MIN_ANGLE_VALUE, MAX_ANGLE_VALUE),
        }
    }
}

impl IsZero for Pose {
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.angle == 0.0
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