use domain::team_type::TeamType;
use std::error::Error;
use domain::debug::Debug;

pub trait DebugSenderTrait {
    fn create_socket(&mut self, team_type: TeamType) -> Result<(), Box<Error>>;
    fn send_debug(&self, debug: Debug) -> Result<(), Box<Error>>;
}