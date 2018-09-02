use domain::team_type::TeamType;
use domain::command::Command;

pub struct CommandReceiver;

impl CommandReceiver {

    pub fn create_socket(&self, _team_type: TeamType) {

    }

    pub fn receive_command(&self) -> Command {
        Command::new()
    }

}