extern crate libc;

use domain::robot::Robot;
use domain::ball::Ball;

#[repr(C)]
pub struct State {
    pub ball: Ball,
    pub team_blue: Vec<Robot>,
    pub team_yellow: Vec<Robot>
}

