extern crate zmq;

use std::str;
use domain::team_type::TeamType;
use domain::command::Command;
use helpers::command_mapper::CommandMapper;
use protobuf::Message;

use self::zmq::{Context, PAIR, Socket};

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
        let global_command = CommandMapper.command_to_global_commands(command);

        let bytes = global_command.write_to_bytes().unwrap();
        let string_bytes = str::from_utf8(&bytes).unwrap();

        self.socket.send(string_bytes, 0).unwrap();
    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5556"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5557")
        }
    }
}