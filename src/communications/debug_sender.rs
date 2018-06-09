extern crate libc;

use domain::team_type::TeamType;
use domain::debug::Debug;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket(team_type: TeamType);
    pub fn send_debug(debug: Debug);
}

pub struct DebugSender;

impl DebugSender {
    pub fn create_socket(team_type: TeamType) {
        unsafe {
            create_socket(team_type);
        }
    }

    pub fn send_debug(debug: Debug){
        unsafe {
            send_debug(debug);
        }
    }
}