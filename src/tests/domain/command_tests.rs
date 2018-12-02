#[allow(unused_imports)] use domain::command::Command;
#[allow(unused_imports)] use protos::command::Global_Commands;

#[test]
pub fn when_create_new_wheels_command_should_be_zero_object() {
    let command = Command::new();

    assert!(command.is_zero());
    assert_eq!(command.commands.len(), 0);
}