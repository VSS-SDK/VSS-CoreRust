use domain::team_type::TeamType;
use domain::debug::Debug;

pub struct DebugReceiver;

impl DebugReceiver {
    pub fn create_socket(_team_type: TeamType) {

    }

    pub fn receive_debug() -> Debug {
        Debug::new()
    }
}