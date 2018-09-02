use domain::point::Point;
use domain::pose::Pose;
use domain::path::Path;
use protos::debug::Global_Debug;
use protos::debug;
use protobuf::RepeatedField;

#[derive(Clone, Debug)]
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

impl From<Debug> for Global_Debug {
    fn from(debug: Debug) -> Self {
        let mut _self = Global_Debug::new();

        let step_points = debug
            .step_points
            .iter()
            .map(|x| debug::Pose::from(x.clone()))
            .collect();

        let final_poses = debug
            .final_poses
            .iter()
            .map(|x| debug::Pose::from(x.clone()))
            .collect();

        let paths = debug
            .paths
            .iter()
            .map(|x| debug::Path::from(x.clone()))
            .collect();

        _self.set_step_poses(RepeatedField::from_vec(step_points));
        _self.set_final_poses(RepeatedField::from_vec(final_poses));
        _self.set_paths(RepeatedField::from_vec(paths));

        _self
    }
}