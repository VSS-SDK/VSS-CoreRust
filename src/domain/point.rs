use protos::{debug, state, control};

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