use domain::robot::Robot;
use domain::ball::Ball;
use protos::state::Global_State;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE};
use traits::new_random_trait::NewRandom;
use traits::is_zero_trait::IsZero;

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

    pub fn new_with(ball: Ball, team_blue: Vec<Robot>, team_yellow: Vec<Robot>) -> Self {
        Self {
            ball,
            team_blue,
            team_yellow
        }
    }
}

impl NewRandom for State {
    fn new_random() -> Self {
        Self {
            ball: Ball::new_random(),
            team_blue: (0..thread_rng().gen_range(MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE))
                .map(|_| {
                    Robot::new_random()
                })
                .collect(),
            team_yellow: (0..thread_rng().gen_range(MIN_RANDOM_TEAM_SIZE, MAX_RANDOM_TEAM_SIZE))
                .map(|_| {
                    Robot::new_random()
                })
                .collect()
        }
    }
}

impl IsZero for State {
    fn is_zero(&self) -> bool {
        self.ball.is_zero() && self.team_yellow.len() == 0 && self.team_blue.len() == 0
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