use domain::state::State;
use domain::field_transaformation_type::FieldTransformationType;
use zmq::{Context, Socket, PAIR};
use protos::state::Global_State;
use protobuf::parse_from_bytes;
use domain::team_type::TeamType;
use domain::debug::Debug;
use protos::debug::Global_Debug;
use protobuf::Message;

pub struct DebugSender{
    context: Context,
    socket: Socket,
    address: String
}

impl DebugSender {
    pub fn new() -> Self {
        let context_helper = Context::new();
        Self {
            context: context_helper.clone(),
            socket: context_helper.socket(PAIR).unwrap(),
            address: String::from("tcp://localhost:5558")
        }
    }

    pub fn create_socket(&mut self, team_type: TeamType) {
        self.setup_address(team_type);

        assert!(
            self.socket
                .connect(&self.address)
                .is_ok()
        );
    }

    pub fn send_debug(&self, debug: Debug) {
        let global_debug = Global_Debug::from(debug);

        let bytes = global_debug
            .write_to_bytes()
            .unwrap_or_default();

        let result = self
            .socket
            .send(bytes, 0);

        if result.is_err() {
            println!("{:?}", result.err())
        }
    }

    fn setup_address(&mut self, team_type: TeamType) {
        match team_type {
            TeamType::Yellow => self.address = String::from("tcp://localhost:5558"),
            TeamType::Blue => self.address = String::from("tcp://localhost:5559")
        }
    }

}