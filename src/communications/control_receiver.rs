extern crate libc;

use domain::control::Control;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket();
    pub fn receive_control() -> Control;
}

pub struct CommandReceiver;

impl CommandReceiver {
    pub fn create_socket() {
        unsafe {
            create_socket();
        }
    }

    pub fn receive_control() -> Control {
        unsafe {
            receive_control()
        }
    }
}