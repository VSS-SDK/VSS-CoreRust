use domain::point::Point;

#[derive(Clone)]
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

