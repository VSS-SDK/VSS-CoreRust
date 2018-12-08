use protos::{debug, state, control};
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use traits::new_random_trait::NewRandom;
use traits::is_zero_trait::IsZero;

#[derive(Clone, Debug)]
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

    pub fn new_with(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl NewRandom for Point {
    fn new_random() -> Self {
        Self {
            x: thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X),
            y: thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y),
        }
    }
}

impl IsZero for Point {
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl From<Point> for debug::Pose {
    fn from(point: Point) -> Self {
        let mut _self = debug::Pose::new();

        _self.set_x(point.x);
        _self.set_y(point.y);
        _self.set_yaw(0.0);

        _self
    }
}

impl From<Point> for state::Pose {
    fn from(point: Point) -> Self {
        let mut _self = state::Pose::new();

        _self.set_x(point.x);
        _self.set_y(point.y);
        _self.set_yaw(0.0);

        _self
    }
}

impl From<Point> for control::Pose {
    fn from(point: Point) -> Self {
        let mut _self = control::Pose::new();

        _self.set_x(point.x);
        _self.set_y(point.y);
        _self.set_yaw(0.0);

        _self
    }
}