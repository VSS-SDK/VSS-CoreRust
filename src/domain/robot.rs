use protos::state::Robot_State;

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
