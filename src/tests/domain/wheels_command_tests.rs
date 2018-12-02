#[allow(unused_imports)] use domain::wheels_command::WheelsCommand;
#[allow(unused_imports)] use protos::command::Robot_Command;
#[allow(unused_imports)] use traits::new_random_trait::NewRandom;
#[allow(unused_imports)] use traits::is_zero_trait::IsZero;

#[test]
pub fn when_create_new_wheels_command_should_be_zero_object() {
    let wheels_command = WheelsCommand::new();

    assert!(wheels_command.is_zero());
    assert_eq!(wheels_command.left_vel, 0.0);
    assert_eq!(wheels_command.right_vel, 0.0);
}

#[test]
pub fn when_create_new_wheels_command_with_should_create_correctly() {
    let wheels_command = WheelsCommand::new_with(1.0, 2.0);

    assert!(!wheels_command.is_zero());
    assert_eq!(wheels_command.left_vel, 1.0);
    assert_eq!(wheels_command.right_vel, 2.0);
}

#[test]
pub fn when_create_new_random_wheels_command_should_not_be_zero_object() {
    let wheels_command = WheelsCommand::new_random();

    assert!(!wheels_command.is_zero());
}

#[test]
pub fn when_map_robot_command_to_wheels_command_should_map_correctly() {
    let robot_command = Robot_Command::new();
    let wheels_command = WheelsCommand::from(robot_command.clone());

    assert_eq!(wheels_command.left_vel, robot_command.get_left_vel());
    assert_eq!(wheels_command.right_vel, robot_command.get_right_vel());
}
