use traits::new_random_trait::NewRandom;
use protos::command::Robot_Command;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND};
use domain::constants::{MIN_COMMAND_SIZE, MAX_COMMAND_SIZE};
use traits::new_random_repeated_trait::NewRandomRepeatedField;
use protobuf::RepeatedField;

impl NewRandom for Robot_Command {
    fn new_random() -> Self {
        let mut robot_command = Robot_Command::new();

        robot_command.set_left_vel(thread_rng().gen_range(MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND));
        robot_command.set_right_vel(thread_rng().gen_range(MIN_RANDOM_WHEEL_COMMAND, MAX_RANDOM_WHEEL_COMMAND));

        robot_command
    }
}

impl NewRandomRepeatedField<Robot_Command> for Robot_Command {
    fn new_random_repeated() -> RepeatedField<Robot_Command> {
        let mut repetead_robots_blue = RepeatedField::new();

        for _index in MIN_COMMAND_SIZE..thread_rng().gen_range(MIN_COMMAND_SIZE, MAX_COMMAND_SIZE) {
            repetead_robots_blue.push(Robot_Command::new_random());
        }

        repetead_robots_blue
    }
}