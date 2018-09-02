use protos::command::Robot_Command;

#[derive(Clone, Debug)]
pub struct WheelsCommand {
    pub left_vel: f32,
    pub right_vel: f32,
}

impl WheelsCommand {
    pub fn new() -> Self {
        Self {
            left_vel: 0.0,
            right_vel: 0.0
        }
    }
}

impl From<WheelsCommand> for Robot_Command {
    fn from(wheels_command: WheelsCommand) -> Self {
        let mut _self = Robot_Command::new();

        _self.set_left_vel(wheels_command.left_vel);
        _self.set_right_vel(wheels_command.right_vel);

        _self
    }
}

impl From<Robot_Command> for WheelsCommand {
    fn from(robot_command: Robot_Command) -> Self {
        WheelsCommand {
            left_vel: robot_command.get_left_vel(),
            right_vel: robot_command.get_right_vel(),
        }
    }
}