extern crate libc;

use self::libc::c_float;

#[repr(C)]
pub struct Pose {
    pub x: c_float,
    pub y: c_float,
    pub angle: c_float,
}

