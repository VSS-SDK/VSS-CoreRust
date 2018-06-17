use domain::robot::Robot;
use domain::ball::Ball;

#[derive(Clone, Debug)]
pub struct State {
    pub ball: Ball,
    pub team_blue: Vec<Robot>,
    pub team_yellow: Vec<Robot>
}

impl State {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            team_blue: Vec::new(),
            team_yellow: Vec::new()
        }
    }
}

