use zmq::{Context, Socket, PAIR};
use domain::team_type::TeamType;
use domain::debug::Debug;
use protos::debug::Global_Debug;
use protobuf::Message;
use std::error::Error;
use traits::debug_sender_trait::DebugSenderTrait;

pub struct DebugSender{
    _context: Context,
    socket: Socket,
    address: String
}

impl DebugSenderTrait for DebugSender {
    fn create_socket(&mut self, team_type: TeamType) -> Result<(), Box<Error>> {
        self.setup_address(team_type);

        Ok(self.socket.connect(&self.address)?)
    }

    fn send_debug(&self, debug: Debug) -> Result<(), Box<Error>>{
        let global_debug = Global_Debug::from(debug);

        let bytes = global_debug
            .write_to_bytes()?;

        Ok(self.socket.send(bytes, 0)?)
    }
}

impl DebugSender {
    pub fn new() -> Result<Self, Box<Error>> {
        let context = Context::new();
        let socket = context.socket(PAIR)?;

        Ok(
            Self {
                _context: context,
                socket,
                address: String::from("tcp://localhost:5558")
            }
        )
    }

    pub fn new_box() -> Result<Box<DebugSenderTrait>, Box<Error>> {
        let context = Context::new();
        let socket = context.socket(PAIR)?;

        let _self = Self {
            _context: context,
            socket,
            address: String::from("tcp://localhost:5558")
        };

        Ok(Box::new(_self))
    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5558"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5559")
        }
    }

}