use domain::control::Control;

pub struct CommandReceiver;

impl CommandReceiver {
    pub fn create_socket() {

    }

    pub fn receive_control() -> Control {
        Control::new()
    }
}