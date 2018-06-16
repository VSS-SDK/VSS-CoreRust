use domain::state::State;
use domain::field_transaformation_type::FieldTransformationType;

pub struct StateReceiver;

impl StateReceiver {
    pub fn create_socket() {

    }

    pub fn receive_state(_field_transaformation_type: FieldTransformationType) -> State {
        State::new()
    }
}