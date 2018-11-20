#[allow(unused_imports)] use domain::state::State;
#[allow(unused_imports)] use protos::state::Pose;
#[allow(unused_imports)] use protos::state::Ball_State;
#[allow(unused_imports)] use protos::state::Global_State;
#[allow(unused_imports)] use domain::ball::Ball;
#[allow(unused_imports)] use protobuf::RepeatedField;
#[allow(unused_imports)] use rand::{thread_rng, Rng};
#[allow(unused_imports)] use domain::constants::{MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE};
#[allow(unused_imports)] use domain::robot::Robot;

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


/*#[test]
pub fn when_map_global_state_to_state_should_map_correctly() {


    assert_eq!(state.team_blue.len(), 0);
    assert_eq!(state.team_yellow.len(), 0);
    assert_eq!(state.ball, Ball::new());
}

fn mock_state() -> State {
    let mut state = Global_State::new();

    let mut pose = Pose::new();
    pose.set_x(1.0);
    pose.set_y(2.0);

    let mut ball = Ball_State::new();
    ball.set_pose(pose);
    ball.set_v_pose( pose);

    let mut balls = RepeatedField::new();
    balls.push(ball);

    state.set_balls(balls);

    state
}*/