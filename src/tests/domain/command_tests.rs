#[allow(unused_imports)] use domain::command::Command;
#[allow(unused_imports)] use protos::command::Global_Commands;
#[allow(unused_imports)] use traits::is_zero_trait::IsZero;
#[allow(unused_imports)] use domain::wheels_command::WheelsCommand;
#[allow(unused_imports)] use traits::new_random_vec::NewRandomVec;

#[test]
pub fn when_create_new_command_should_be_zero_object() {
    let command = Command::new();

    assert!(command.is_zero());
    assert_eq!(command.commands.len(), 0);
}

#[test]
pub fn when_create_new_command_with_should_create_correctly() {
    let wheels_command = WheelsCommand::new_random_vec();
    let command = Command::new_with(wheels_command);

    assert!(!command.is_zero());
}