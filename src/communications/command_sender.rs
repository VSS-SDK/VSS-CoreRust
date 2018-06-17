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

    pub fn new() -> Self {
        let context_helper = Context::new();
        Self {
            context: context_helper.clone(),
            socket: context_helper.socket(PAIR).unwrap(),
            address: String::from("")
        }
    }

    pub fn create_socket(&mut self, team_type: TeamType) {
        self.setup_address(team_type);

        assert!(self.socket.connect(&self.address).is_ok());
    }

    pub fn send_command(&self, command: Command) {
        println!("teste1");
        let global_command = CommandMapper.command_to_global_commands(command);

        match global_command.write_to_bytes(){
            Ok(V) => {
                println!("teste2");
                let string_bytes = str::from_utf8(&V).unwrap();
                println!("teste3");
                match self.socket.send(string_bytes, 0){
                    Ok(V2) => println!("enviado"),
                    Err(E2) => println!("{:?}", E2)
                }
            },
            Err(E) => println!("{:?}", E)
        }

    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5556"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5557")
        }
    }
}