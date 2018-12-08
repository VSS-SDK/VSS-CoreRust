use protobuf::RepeatedField;
use protos::state::{Pose, Robot_State, Ball_State, Global_State};
use rand::{thread_rng, Rng};
use domain::constants::{MIN_RANDOM_BALL_QTD, MAX_RANDOM_BALL_QTD};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use domain::constants::{MIN_ANGLE_VALUE, MAX_ANGLE_VALUE};
use domain::constants::{MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE};
use traits::new_random_trait::NewRandom;
use traits::new_random_repeated_trait::NewRandomRepeatedField;


impl NewRandom for Pose {
    fn new_random() -> Self {
        let mut pose = Pose::new();

        pose.set_x(thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X));
        pose.set_y(thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y));
        pose.set_yaw(thread_rng().gen_range(MIN_ANGLE_VALUE, MAX_ANGLE_VALUE));

        pose
    }
}

impl NewRandom for Ball_State {
    fn new_random() -> Self {
        let mut ball_state = Ball_State::new();

        ball_state.set_pose(Pose::new_random());
        ball_state.set_v_pose(Pose::new_random());

        ball_state
    }
}

impl NewRandom for Robot_State {
    fn new_random() -> Self {
        let mut robot_state = Robot_State::new();

        robot_state.set_pose(Pose::new_random());
        robot_state.set_v_pose(Pose::new_random());

        robot_state
    }
}

impl NewRandom for Global_State {
    fn new_random() -> Self {
        let mut global_state = Global_State::new();

        global_state.set_balls(Ball_State::new_random_repeated());
        global_state.set_robots_blue(Robot_State::new_random_repeated());
        global_state.set_robots_yellow(Robot_State::new_random_repeated());

        global_state
    }
}

impl NewRandomRepeatedField<Ball_State> for Ball_State {
    fn new_random_repeated() -> RepeatedField<Ball_State> {
        let mut repetead_balls = RepeatedField::new();

        for _index in MIN_RANDOM_BALL_QTD..thread_rng().gen_range(MIN_RANDOM_BALL_QTD, MAX_RANDOM_BALL_QTD) {
            repetead_balls.push(Ball_State::new_random());
        }

        repetead_balls
    }
}

impl NewRandomRepeatedField<Robot_State> for Robot_State {
    fn new_random_repeated() -> RepeatedField<Robot_State> {
        let mut repetead_robots_blue = RepeatedField::new();

        for _index in MIN_RANDOM_TEAM_SIZE..thread_rng().gen_range(MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE) {
            repetead_robots_blue.push(Robot_State::new_random());
        }

        repetead_robots_blue
    }
}