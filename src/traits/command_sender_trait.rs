use domain::team_type::TeamType;
use std::error::Error;
use domain::command::Command;

pub trait CommandSenderTrait {
    fn create_socket(&mut self, team_type: TeamType) -> Result<(), Box<Error>>;
    fn send_command(&self, command: Command) -> Result<(), Box<Error>>;
}