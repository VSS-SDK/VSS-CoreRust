#[allow(unused_imports)] use domain::ball::Ball;
#[allow(unused_imports)] use protos::state::Ball_State;

#[test]
pub fn when_create_new_ball_should_be_zero_object() {
    let ball = Ball::new();

    assert!(ball.is_zero());
    assert_eq!(ball.x, 0.0);
    assert_eq!(ball.y, 0.0);
    assert_eq!(ball.speed_x, 0.0);
    assert_eq!(ball.speed_y, 0.0);
}

#[test]
pub fn when_create_new_ball_with_should_create_correctly() {
    let ball = Ball::new_with(1.0, 2.0, 3.0, 4.0);

    assert!(!ball.is_zero());
    assert_eq!(ball.x, 1.0);
    assert_eq!(ball.y, 2.0);
    assert_eq!(ball.speed_x, 3.0);
    assert_eq!(ball.speed_y, 4.0);
}

#[test]
pub fn when_create_new_random_point_should_not_be_zero_object() {
    let ball = Ball::new_random();

    assert!(!ball.is_zero());
}

#[test]
pub fn when_map_ball_state_to_ball_should_map_correctly() {
    let ball_state = Ball_State::new();
    let ball = Ball::from(ball_state.clone());

    assert_eq!(ball.x, ball_state.get_pose().get_x());
    assert_eq!(ball.y, ball_state.get_pose().get_y());
    assert_eq!(ball.speed_x, ball_state.get_v_pose().get_x());
    assert_eq!(ball.speed_y, ball_state.get_v_pose().get_y());
}
