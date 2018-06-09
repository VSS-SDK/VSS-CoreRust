extern crate libc;

use self::libc::c_int;
use domain::wheels_command::WheelsCommand;

#[repr(C)]
#[derive(Clone)]
pub struct Command {
    pub id: c_int,
    pub commands: Vec<WheelsCommand>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            id: 0,
            commands: Vec::new()
        }
    }
}
