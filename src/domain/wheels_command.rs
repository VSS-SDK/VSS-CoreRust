extern crate libc;

use self::libc::c_int;
use self::libc::c_float;

#[repr(C)]
#[derive(Clone)]
pub struct WheelsCommand {
    pub id: c_int,
    pub left_vel: c_float,
    pub right_vel: c_float,
}

impl WheelsCommand {
    pub fn new() -> Self {
        Self {
            id: 0,
            left_vel: 0.0,
            right_vel: 0.0
        }
    }
}