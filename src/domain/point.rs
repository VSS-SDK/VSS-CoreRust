extern crate libc;

use self::libc::c_float;

#[repr(C)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}

