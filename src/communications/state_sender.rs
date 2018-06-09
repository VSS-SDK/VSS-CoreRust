extern crate libc;

use domain::state::State;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket();
    pub fn send_state(state: State);
}

pub struct StateReceiver;

impl StateReceiver {
    pub fn create_socket() {
        unsafe {
            create_socket();
        }
    }

    pub fn send_state(state: State) {
        unsafe {
            send_state(state);
        }
    }
}