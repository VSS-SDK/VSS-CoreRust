extern crate libc;

use self::libc::c_int;
use domain::robot::Robot;
use domain::ball::Ball;

#[repr(C)]
pub struct Control {
    pub paused: c_int,
    pub ball: Ball,
    pub team_blue: Vec<Robot>,
    pub team_yellow: Vec<Robot>
}

