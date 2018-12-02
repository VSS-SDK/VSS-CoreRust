#[allow(unused_imports)] use domain::state::State;
#[allow(unused_imports)] use protos::state::Pose;
#[allow(unused_imports)] use protos::state_random_extension;
#[allow(unused_imports)] use protos::state::Ball_State;
#[allow(unused_imports)] use protos::state::Global_State;
#[allow(unused_imports)] use domain::ball::Ball;
#[allow(unused_imports)] use protobuf::RepeatedField;
#[allow(unused_imports)] use rand::{thread_rng, Rng};
#[allow(unused_imports)] use domain::robot::Robot;
#[allow(unused_imports)] use protos::state::Robot_State;
#[allow(unused_imports)] use domain::constants::{MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE};
#[allow(unused_imports)] use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
#[allow(unused_imports)] use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
#[allow(unused_imports)] use domain::constants::{MIN_ANGLE_VALUE, MAX_ANGLE_VALUE};
#[allow(unused_imports)] use domain::constants::{MIN_RANDOM_BALL_QTD, MAX_RANDOM_BALL_QTD};
#[allow(unused_imports)] use traits::new_random_trait::NewRandom;
#[allow(unused_imports)] use traits::new_random_repeated_trait::NewRandomRepeatedField;

#[test]
pub fn when_create_new_state_should_be_zero_object() {
    let state = State::new();

    assert!(state.is_zero());
    assert_eq!(state.team_blue.len(), 0);
    assert_eq!(state.team_yellow.len(), 0);
    assert_eq!(state.ball, Ball::new());
}

#[test]
pub fn when_create_new_state_with_should_create_correctly() {
    let team_blue = (0..thread_rng().gen_range(MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE))
        .map(|_| {
            Robot::new_random()
        })
        .collect::<Vec<Robot>>();

    let team_yellow =  (0..thread_rng().gen_range(MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE))
        .map(|_| {
            Robot::new_random()
        })
        .collect::<Vec<Robot>>();

    let ball = Ball::new_random();

    let state = State::new_with(ball.clone(), team_blue.clone(), team_yellow.clone());

    assert!(!state.is_zero());
    assert_eq!(state.team_blue.len(), team_blue.len());
    assert_eq!(state.team_yellow.len(), team_yellow.len());
    assert_eq!(state.ball, ball);

    assert!(!state.team_blue
        .iter()
        .zip(team_blue.iter())
        .map(|(x, y)| {
            x == y
        })
        .collect::<Vec<bool>>()
        .contains(&false));

    assert!(!state.team_yellow
        .iter()
        .zip(team_yellow.iter())
        .map(|(x, y)| {
            x == y
        })
        .collect::<Vec<bool>>()
        .contains(&false));
}

#[test]
pub fn when_create_new_random_state_should_not_be_zero_object() {
    let point = State::new_random();

    assert!(!point.is_zero());
}

#[test]
pub fn when_map_global_state_to_state_should_map_correctly() {
    let mut _global_state = Global_State::new_random();
    let state = State::from(_global_state.clone());

    if _global_state.get_balls().len() > 0 {
        let ball_state = _global_state.get_balls().first().unwrap();
        assert_eq!(ball_state.get_pose().get_x(), state.ball.x);
        assert_eq!(ball_state.get_pose().get_y(), state.ball.y);
        assert_eq!(ball_state.get_v_pose().get_x(), state.ball.speed_x);
        assert_eq!(ball_state.get_v_pose().get_y(), state.ball.speed_y);
    } else {
        assert!(state.ball.is_zero());
    }

    assert_eq!(_global_state.get_robots_yellow().len(), state.team_yellow.len());
    assert_eq!(_global_state.get_robots_blue().len(), state.team_blue.len());
}