use protos::state::Ball_State;
use protos::state::Pose;

impl Ball_State {
    pub fn new_random() -> Self {
        let mut ball_state = Ball_State::new();

        ball_state.set_pose(Pose::new_random());
        ball_state.set_v_pose(Pose::new_random());

        ball_state
    }
}

