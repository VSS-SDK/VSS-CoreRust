use protos::state::Ball_State;
use protos::state::Pose;
use protobuf::RepeatedField;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_RANDOM_BALL_QTD, MAX_RANDOM_BALL_QTD};
use traits::new_random_trait::NewRandom;
use traits::new_random_repeated_trait::NewRandomRepeatedField;

impl NewRandom for Ball_State {
    fn new_random() -> Self {
        let mut ball_state = Ball_State::new();

        ball_state.set_pose(Pose::new_random());
        ball_state.set_v_pose(Pose::new_random());

        ball_state
    }
}

/*impl<T> NewRandomRepeatedField<T> for Ball_State {
    fn new_random() -> RepeatedField<T> {
        let mut repetead_balls = RepeatedField::new();

        for _index in MIN_RANDOM_BALL_QTD..thread_rng().gen_range(MIN_RANDOM_BALL_QTD, MAX_RANDOM_BALL_QTD) {
            repetead_balls.push(Ball_State::new_random());
        }

        repetead_balls
    }
}*/
