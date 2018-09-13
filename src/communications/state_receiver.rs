use domain::state::State;
use domain::field_transaformation_type::FieldTransformationType;
use helpers::coordinate_transformer::CoordinateTransformer;
use zmq::{Context, Socket, SUB};
use protos::state::Global_State;
use protobuf::parse_from_bytes;
use std::error::Error;
use std::result::Result::Ok;
use traits::state_receiver_trait::StateReceiverTrait;

pub struct StateReceiver{
    _context: Context,
    socket: Socket,
    address: String
}

impl StateReceiverTrait for StateReceiver {
    fn create_socket(&self) -> Result<(), Box<Error>> {
        self.socket.connect(&self.address)?;

        Ok(self.socket.set_subscribe(String::from("").as_bytes())?)
    }

    fn receive_state(&self, transform_type: FieldTransformationType) -> Result<State, Box<Error>> {
        let bytes_state = self.socket
            .recv_bytes(0)?;

        let global_state = parse_from_bytes::<Global_State>(&bytes_state)?;

        let mut state = State::from(global_state);

        if transform_type == FieldTransformationType::Flip180Degrees {
            let transformer = CoordinateTransformer::new();
            transformer.spin_180_degrees(&mut state);
        }

        Ok(state)
    }
}

impl StateReceiver {
    pub fn new() -> Result<Self, Box<Error>> {
        let context = Context::new();
        let socket = context.socket(SUB)?;

        Ok(
            Self {
                _context: context,
                socket,
                address: String::from("tcp://localhost:5555")
            }
        )
    }



}