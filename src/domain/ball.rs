use protos::state::Ball_State;

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

    pub fn new_with(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y
        }
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