use protos::state::Robot_State;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use domain::constants::{MIN_ANGLE_VALUE, MAX_ANGLE_VALUE};
use domain::constants::{MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY};

#[derive(Clone, Debug, PartialEq)]
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

    pub fn new_with(x: f32, y: f32, angle: f32, speed_x: f32, speed_y: f32, speed_angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            speed_x,
            speed_y,
            speed_angle
        }
    }

    pub fn new_random() -> Self {
        Self {
            x: thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X),
            y: thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y),
            angle: thread_rng().gen_range(MIN_ANGLE_VALUE, MAX_ANGLE_VALUE),
            speed_x: thread_rng().gen_range(MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY),
            speed_y: thread_rng().gen_range(MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY),
            speed_angle: thread_rng().gen_range(MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.angle == 0.0 && self.speed_x == 0.0 && self.speed_y == 0.0 && self.speed_angle == 0.0
    }
}

impl From<Robot_State> for Robot {
    fn from(robot_state: Robot_State) -> Self {
        Robot {
            x: robot_state
                .get_pose()
                .get_x(),

            y: robot_state
                .get_pose()
                .get_y(),

            angle: robot_state
                .get_pose()
                .get_yaw(),

            speed_x: robot_state
                .get_v_pose()
                .get_x(),

            speed_y: robot_state
                .get_v_pose()
                .get_y(),

            speed_angle: robot_state
                .get_v_pose()
                .get_yaw()
        }
    }
}
