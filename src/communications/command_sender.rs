extern crate zmq;

use domain::team_type::TeamType;
use domain::command::Command;

use self::zmq::{Context, SUB, PAIR, Socket};

use domain::state;

pub struct CommandSender{
    context: Context,
    socket: Socket,
    address: String
}

impl CommandSender {

    pub fn create_socket(&mut self, team_type: TeamType) {
        self.setup_address(team_type);

        self.context = Context::new();
        self.socket = self.context.socket(PAIR).unwrap();

        assert!(self.socket.connect(&self.address).is_ok());
    }

    pub fn send_command(&self, command: Command) {
        // let global_command = CommandMapper.
        // let mut bytes_commands = command.write_to_bytes().unwrap();
    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5556"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5557")
        }
    }
}