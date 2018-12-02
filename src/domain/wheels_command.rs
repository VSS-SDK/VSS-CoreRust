use protos::command::Robot_Command;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND};
use traits::new_random_trait::NewRandom;

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

    pub fn new_with(left_vel: f32, right_vel: f32) -> Self {
        Self {
            left_vel,
            right_vel
        }
    }

    pub fn is_zero(&self) -> bool {
        self.left_vel == 0.0 && self.right_vel == 0.0
    }
}

impl NewRandom for WheelsCommand {
    fn new_random() -> Self {
        Self {
            left_vel: thread_rng().gen_range(MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND),
            right_vel: thread_rng().gen_range(MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND),
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