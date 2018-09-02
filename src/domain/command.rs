use domain::wheels_command::WheelsCommand;

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
