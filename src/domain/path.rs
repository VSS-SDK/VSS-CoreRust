extern crate libc;

use domain::point::Point;

#[repr(C)]
pub struct Path {
    pub points: Vec<Point>
}

