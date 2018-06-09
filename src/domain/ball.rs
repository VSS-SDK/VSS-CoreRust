extern crate libc;

use self::libc::c_float;

#[repr(C)]
pub struct Ball {
    pub x: c_float,
    pub y: c_float,
    pub speed_x: c_float,
    pub speed_y: c_float
}

