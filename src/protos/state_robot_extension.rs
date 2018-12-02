use protos::state::Robot_State;
use protos::state::Pose;
use traits::new_random_trait::NewRandom;

impl NewRandom for Robot_State {
    fn new_random() -> Self {
        let mut robot_state = Robot_State::new();

        robot_state.set_pose(Pose::new_random());
        robot_state.set_v_pose(Pose::new_random());

        robot_state
    }
}

