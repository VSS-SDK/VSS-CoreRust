use protobuf::RepeatedField;

use protos::command::{Global_Commands, Robot_Command};
use domain::command::Command;
use domain::wheels_command::WheelsCommand;

pub struct CommandMapper;

impl CommandMapper {

    pub fn command_to_global_commands(&self, command: Command) -> Global_Commands {
        let mut robots_command = Vec::new();

        for wheels_command in &command.commands {
            robots_command.push(self.wheels_command_to_robot_command(wheels_command.clone()));
        }

        let mut global_commands = Global_Commands::new();
        global_commands.set_robot_commands(RepeatedField::from_vec(robots_command));

        global_commands
    }

    pub fn wheels_command_to_robot_command(&self, wheels_command: WheelsCommand) -> Robot_Command {
        let mut robot_command = Robot_Command::new();

        robot_command.set_left_vel(wheels_command.left_vel);
        robot_command.set_right_vel(wheels_command.right_vel);

        robot_command
    }

}