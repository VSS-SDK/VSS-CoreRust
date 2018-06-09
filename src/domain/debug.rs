extern crate libc;

use domain::point::Point;
use domain::pose::Pose;
use domain::path::Path;

#[repr(C)]
pub struct Command {
    pub step_points: Vec<Point>,
    pub final_poses: Vec<Pose>,
    pub team_yellow: Vec<Path>
}

