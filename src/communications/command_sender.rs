extern crate zmq;

use domain::team_type::TeamType;
use domain::command::Command;
use protobuf::Message;

use self::zmq::{Context, PAIR, Socket};
use protos::command::Global_Commands;
use std::error::Error;
use traits::command_sender_trait::CommandSenderTrait;

pub struct CommandSender{
    _context: Context,
    socket: Socket,
    address: String
}

impl CommandSenderTrait for CommandSender {
    fn create_socket(&mut self, team_type: TeamType) -> Result<(), Box<Error>> {
        self.setup_address(team_type);

        Ok(self.socket.connect(&self.address)?)
    }

    fn send_command(&self, command: Command) -> Result<(), Box<Error>> {
        let global_command = Global_Commands::from(command);

        let bytes = global_command
            .write_to_bytes()?;

        Ok(self.socket.send(bytes, 0)?)
    }
}

impl CommandSender {
    pub fn new() -> Result<Self, Box<Error>> {
        let context = Context::new();
        let socket = context.socket(PAIR)?;

        Ok(
            Self {
                _context: context,
                socket,
                address: String::from("")
            }
        )
    }

    pub fn new_box() -> Result<Box<CommandSenderTrait>, Box<Error>> {
        let context = Context::new();
        let socket = context.socket(PAIR)?;

        let _self = Self {
            _context: context,
            socket,
            address: String::from("")
        };

        Ok(Box::new(_self))
    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5556"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5557")
        }
    }
}