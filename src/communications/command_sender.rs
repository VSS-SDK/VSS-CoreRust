extern crate libc;

use domain::team_type::TeamType;
use domain::command::Command;

#[link(name = "vss-core")]
extern "C"{
    pub fn create_socket(team_type: TeamType);
    pub fn send_command(command: Command);
}

pub struct CommandSender;

impl CommandSender {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn create_socket(&self, team_type: TeamType) {
        unsafe {
            create_socket(team_type);
        }
    }

    pub fn send_command(&self, command: Command) {
        unsafe {
            send_command(command);
        }
    }
}

#[cfg(test)]
mod tests {
    use self::super::CommandSender;
    use self::super::Command;
    use self::super::super::super::domain::wheels_command::WheelsCommand;

    #[test]
    fn send_command() {
        let command_sender = CommandSender::new();
        let mut command = Command::new();

        command.commands.push(WheelsCommand {
            id: 0,
            left_vel: 10.0,
            right_vel: 10.0
        });

        command.commands.push(WheelsCommand {
            id: 1,
            left_vel: 10.0,
            right_vel: 10.0
        });

        command.commands.push(WheelsCommand {
            id: 2,
            left_vel: 10.0,
            right_vel: 10.0
        });

        loop {
            command_sender.send_command(command.clone());
        }
    }
}