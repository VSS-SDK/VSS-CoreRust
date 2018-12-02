use protos::state::Ball_State;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use domain::constants::{MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY};
use traits::new_random_trait::NewRandom;
use traits::is_zero_trait::IsZero;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn new_with(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y
        }
    }
}

impl NewRandom for Ball {
    fn new_random() -> Self {
        Self {
            x: thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X),
            y: thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y),
            speed_x: thread_rng().gen_range(MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY),
            speed_y: thread_rng().gen_range(MIN_RANDOM_VELOCITY, MAX_RANDOM_VELOCITY),
        }
    }
}

impl IsZero for Ball {
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.speed_x == 0.0 && self.speed_y == 0.0
    }
}

impl From<Ball_State> for Ball {
    fn from(ball_state: Ball_State) -> Self {
        Ball {
            x: ball_state
                .get_pose()
                .get_x(),

            y: ball_state
                .get_pose()
                .get_y(),

            speed_x: ball_state
                .get_v_pose()
                .get_x(),

            speed_y: ball_state
                .get_v_pose()
                .get_y(),
        }
    }
}