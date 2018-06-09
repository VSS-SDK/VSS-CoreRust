extern crate libc;

use domain::team_type::TeamType;
use domain::command::Command;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket(team_type: TeamType);
    pub fn receive_command() -> Command;
}

pub struct CommandReceiver;

impl CommandReceiver {
    pub fn create_socket(team_type: TeamType) {
        unsafe {
            create_socket(team_type);
        }
    }

    pub fn receive_command() -> Command {
        unsafe {
            receive_command()
        }
    }
}