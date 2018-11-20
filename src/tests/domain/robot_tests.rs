#[allow(unused_imports)] use domain::robot::Robot;
#[allow(unused_imports)] use protos::state::Robot_State;

#[test]
pub fn when_create_new_robot_should_be_zero_object() {
    let robot = Robot::new();

    assert_eq!(robot.x, 0.0);
    assert_eq!(robot.y, 0.0);
    assert_eq!(robot.angle, 0.0);
    assert_eq!(robot.speed_x, 0.0);
    assert_eq!(robot.speed_y, 0.0);
    assert_eq!(robot.speed_angle, 0.0);
}

#[test]
pub fn when_map_robot_state_to_robot_should_map_correctly() {
    let robot_state = Robot_State::new();
    let robot = Robot::from(robot_state.clone());

    assert_eq!(robot.x, robot_state.get_pose().get_x());
    assert_eq!(robot.y, robot_state.get_pose().get_y());
    assert_eq!(robot.angle, robot_state.get_pose().get_yaw());
    assert_eq!(robot.speed_x, robot_state.get_v_pose().get_x());
    assert_eq!(robot.speed_y, robot_state.get_v_pose().get_y());
    assert_eq!(robot.speed_angle, robot_state.get_v_pose().get_yaw());
}