use domain::ball::Ball;

#[test]
pub fn when_create_new_ball_should_be_zero_object() {
    let ball = Ball::new();

    assert_eq!(ball.x, 0.0);
    assert_eq!(ball.y, 0.0);
    assert_eq!(ball.speed_x, 0.0);
    assert_eq!(ball.speed_y, 0.0);
}
