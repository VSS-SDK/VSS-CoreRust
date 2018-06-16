use domain::point::Point;
use domain::pose::Pose;
use domain::path::Path;

#[derive(Clone)]
pub struct Debug {
    pub step_points: Vec<Point>,
    pub final_poses: Vec<Pose>,
    pub paths: Vec<Path>
}

impl Debug {
    pub fn new() -> Self {
        Self {
            step_points: Vec::new(),
            final_poses: Vec::new(),
            paths: Vec::new()
        }
    }
}

