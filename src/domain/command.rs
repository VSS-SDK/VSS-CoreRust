use domain::wheels_command::WheelsCommand;
use protos::command::Global_Commands;
use protos::command::Robot_Command;
use protobuf::RepeatedField;

#[derive(Clone, Debug)]
pub struct Command {
    pub commands: Vec<WheelsCommand>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            commands: Vec::new()
        }
    }
}

impl From<Command> for Global_Commands {
    fn from(command: Command) -> Self {
        let mut _self = Global_Commands::new();

        let robots_command : Vec<Robot_Command  > = command.commands
            .iter()
            .map(|x| Robot_Command::from(x.clone()))
            .collect();

        _self.set_robot_commands(RepeatedField::from_vec(robots_command));

        _self
    }
}
