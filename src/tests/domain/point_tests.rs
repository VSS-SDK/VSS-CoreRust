#[allow(unused_imports)] use domain::point::Point;
#[allow(unused_imports)] use protos::debug;
#[allow(unused_imports)] use protos::state;
#[allow(unused_imports)] use protos::control;
#[allow(unused_imports)] use traits::new_random_trait::NewRandom;

#[test]
pub fn when_create_new_point_should_be_zero_object() {
    let point = Point::new();

    assert!(point.is_zero());
    assert_eq!(point.x, 0.0);
    assert_eq!(point.y, 0.0);
}

#[test]
pub fn when_create_new_point_with_should_create_correctly() {
    let point = Point::new_with(1.0, 2.0);

    assert!(!point.is_zero());
    assert_eq!(point.x, 1.0);
    assert_eq!(point.y, 2.0);
}

#[test]
pub fn when_create_new_random_point_should_not_be_zero_object() {
    let point = Point::new_random();

    assert!(!point.is_zero());
}

#[test]
pub fn when_map_point_to_debug_pose_should_map_correctly() {
    let point = Point::new();
    let debug_pose = debug::Pose::from(point.clone());

    assert_eq!(debug_pose.get_x(), point.x);
    assert_eq!(debug_pose.get_y(), point.y);
    assert_eq!(debug_pose.get_yaw(), 0.0);
}

#[test]
pub fn when_map_point_to_state_pose_should_map_correctly() {
    let point = Point::new();
    let state_pose = state::Pose::from(point.clone());

    assert_eq!(state_pose.get_x(), point.x);
    assert_eq!(state_pose.get_y(), point.y);
    assert_eq!(state_pose.get_yaw(), 0.0);
}

#[test]
pub fn when_map_point_to_control_pose_should_map_correctly() {
    let point = Point::new();
    let control_pose = control::Pose::from(point.clone());

    assert_eq!(control_pose.get_x(), point.x);
    assert_eq!(control_pose.get_y(), point.y);
    assert_eq!(control_pose.get_yaw(), 0.0);
}