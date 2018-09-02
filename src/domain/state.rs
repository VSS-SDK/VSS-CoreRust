use domain::robot::Robot;
use domain::ball::Ball;
use protos::state::Global_State;

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

impl From<Global_State> for State {
    fn from(global_state: Global_State) -> Self {
        State {
            ball: global_state
                .get_balls()
                .iter()
                .map(|x| Ball::from(x.clone()))
                .nth(0)
                .unwrap_or(Ball::new()),

            team_blue: global_state
                .get_robots_blue()
                .iter()
                .map(|x| Robot::from(x.clone()))
                .collect(),

            team_yellow: global_state
                .get_robots_yellow()
                .iter()
                .map(|x| Robot::from(x.clone()))
                .collect(),
        }
    }
}