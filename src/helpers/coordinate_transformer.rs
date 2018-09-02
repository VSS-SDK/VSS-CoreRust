use domain::state::State;
use domain::constants;
use domain::ball::Ball;
use domain::robot::Robot;

pub struct CoordinateTransformer;

impl CoordinateTransformer {
    pub fn new() -> Self {
        CoordinateTransformer
    }
    pub fn spin_180_degrees(&self, state: &mut State) {
        self.spin_ball_180_degrees(&mut state.ball);

        for index in 0..state.team_yellow.len() {
            self.spin_robots_180_degrees(&mut state.team_yellow[index]);
        }

        for index in 0..state.team_blue.len() {
            self.spin_robots_180_degrees(&mut state.team_blue[index]);
        }
    }

    pub fn spin_ball_180_degrees(&self, ball: &mut Ball) {
        ball.x = constants::MIN_COORDINATE_X - ball.x;
        ball.y = constants::MIN_COORDINATE_X - ball.y;

        ball.speed_x = -ball.speed_x;
        ball.speed_y = -ball.speed_y;
    }

    pub fn spin_robots_180_degrees(&self, robot: &mut Robot) {
        robot.x = constants::MIN_COORDINATE_X - robot.x;
        robot.y = constants::MIN_COORDINATE_X - robot.y;

        if robot.angle + (constants::MAX_ANGLE_VALUE/2.0) > constants::MAX_ANGLE_VALUE {
            robot.angle = robot.angle - (constants::MAX_ANGLE_VALUE/2.0);
        } else {
            robot.angle = robot.angle + (constants::MAX_ANGLE_VALUE/2.0);
        }

        robot.speed_x = -robot.speed_x;
        robot.speed_y = -robot.speed_y;
        robot.speed_angle = -robot.speed_angle;
    }
}