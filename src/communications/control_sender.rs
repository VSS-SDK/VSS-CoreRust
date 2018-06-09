extern crate libc;

use domain::control::Control;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket();
    pub fn send_control(control: Control);
}

pub struct CommandSender;

impl CommandSender {
    pub fn create_socket() {
        unsafe {
            create_socket();
        }
    }

    pub fn send_control(control: Control) {
        unsafe {
            send_control(control);
        }
    }
}