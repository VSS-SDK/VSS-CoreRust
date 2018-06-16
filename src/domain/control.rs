use domain::robot::Robot;
use domain::ball::Ball;

#[derive(Clone)]
pub struct Control {
    pub paused: bool,
    pub ball: Ball,
    pub team_blue: Vec<Robot>,
    pub team_yellow: Vec<Robot>
}

impl Control {
    pub fn new() -> Self {
        Self {
            paused: true,
            ball: Ball::new(),
            team_blue: Vec::new(),
            team_yellow: Vec::new()
        }
    }
}

