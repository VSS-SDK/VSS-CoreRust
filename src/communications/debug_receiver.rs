extern crate libc;

use domain::team_type::TeamType;
use domain::debug::Debug;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket(team_type: TeamType);
    pub fn receive_debug() -> Debug;
}

pub struct DebugReceiver;

impl DebugReceiver {
    pub fn create_socket(team_type: TeamType) {
        unsafe {
            create_socket(team_type);
        }
    }

    pub fn receive_debug() -> Debug{
        unsafe {
            receive_debug()
        }
    }
}