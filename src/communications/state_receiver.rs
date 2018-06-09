extern crate libc;

use domain::state::State;
use domain::field_transaformation_type::FieldTransformationType;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket();
    pub fn receive_state(field_transaformation_type: FieldTransformationType) -> State;
}

pub struct StateReceiver;

impl StateReceiver {
    pub fn create_socket() {
        unsafe {
            create_socket();
        }
    }

    pub fn receive_state(field_transaformation_type: FieldTransformationType) -> State {
        unsafe {
            receive_state(field_transaformation_type)
        }
    }
}