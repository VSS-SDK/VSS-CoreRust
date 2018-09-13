use std::error::Error;
use domain::field_transaformation_type::FieldTransformationType;
use domain::state::State;

pub trait StateReceiverTrait {
    fn create_socket(&self) -> Result<(), Box<Error>>;
    fn receive_state(&self, transform_type: FieldTransformationType) -> Result<State, Box<Error>>;
}