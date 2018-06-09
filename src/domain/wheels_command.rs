extern crate libc;

use self::libc::c_int;
use self::libc::c_float;

#[repr(C)]
pub struct WheelsCommand {
    pub id: c_int,
    pub left_vel: c_float,
    pub right_vel: c_float,
}

