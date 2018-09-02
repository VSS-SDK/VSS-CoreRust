use domain::point::Point;
use protos::debug;
use protobuf::RepeatedField;

#[derive(Clone, Debug)]
pub struct Path {
    pub points: Vec<Point>
}

impl Path {
    pub fn new() -> Self {
        Self {
            points: Vec::new()
        }
    }
}

impl From<Path> for debug::Path {
    fn from(path: Path) -> Self {
        let mut _self = debug::Path::new();

        let points = path
            .points
            .iter()
            .map(|x| debug::Pose::from(x.clone()))
            .collect();

        _self.set_poses(RepeatedField::from_vec(points));

        _self
    }
}
