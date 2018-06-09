extern crate libc;

use self::libc::c_int;
use domain::wheels_command::WheelsCommand;

#[repr(C)]
pub struct Command {
    pub id: c_int,
    pub commands: Vec<WheelsCommand>,
}

