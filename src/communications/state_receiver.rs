use domain::state::State;
use domain::field_transaformation_type::FieldTransformationType;
use zmq::{Context, Socket, SUB};
use protos::state::Global_State;
use protobuf::parse_from_bytes;

pub struct StateReceiver{
    context: Context,
    socket: Socket,
    address: String
}

impl StateReceiver {
    pub fn new() -> Self {
        let context_helper = Context::new();
        Self {
            context: context_helper.clone(),
            socket: context_helper.socket(SUB).unwrap(),
            address: String::from("tcp://localhost:5555")
        }
    }

    pub fn create_socket(&self) {
        let filter = "".to_string();

        assert!(
            self.socket
                .connect(&self.address)
                .is_ok()
        );

        assert!(
            self.socket
                .set_subscribe(filter.as_bytes())
                .is_ok()
        );
    }

    pub fn receive_state(&self, field_transaformation_type: FieldTransformationType) -> State {
        let bytes_state = self.socket
            .recv_bytes(0)
            .unwrap_or_default();

        let global_state = parse_from_bytes::<Global_State>(&bytes_state).unwrap_or_default();

        println!("{:?}", global_state);

        State::new()
    }

}