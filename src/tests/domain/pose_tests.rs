#[allow(unused_imports)] use domain::pose::Pose;
#[allow(unused_imports)] use protos::debug;
#[allow(unused_imports)] use protos::state;
#[allow(unused_imports)] use protos::control;
#[allow(unused_imports)] use traits::new_random_trait::NewRandom;
#[allow(unused_imports)] use traits::is_zero_trait::IsZero;

#[test]
pub fn when_create_new_point_should_be_zero_object() {
    let pose = Pose::new();

    assert!(pose.is_zero());
    assert_eq!(pose.x, 0.0);
    assert_eq!(pose.y, 0.0);
    assert_eq!(pose.angle, 0.0);
}

#[test]
pub fn when_create_new_point_with_should_create_correctly() {
    let pose = Pose::new_with(1.0, 2.0, 3.0);

    assert!(!pose.is_zero());
    assert_eq!(pose.x, 1.0);
    assert_eq!(pose.y, 2.0);
    assert_eq!(pose.angle, 3.0);
}

#[test]
pub fn when_create_new_random_pose_should_not_be_zero_object() {
    let pose = Pose::new_random();

    assert!(!pose.is_zero());
}

#[test]
pub fn when_map_pose_to_debug_pose_should_map_correctly() {
    let pose = Pose::new();
    let debug_pose = debug::Pose::from(pose.clone());

    assert_eq!(debug_pose.get_x(), pose.x);
    assert_eq!(debug_pose.get_y(), pose.y);
    assert_eq!(debug_pose.get_yaw(), pose.angle);
}

#[test]
pub fn when_map_pose_to_state_pose_should_map_correctly() {
    let pose = Pose::new();
    let state_pose = state::Pose::from(pose.clone());

    assert_eq!(state_pose.get_x(), pose.x);
    assert_eq!(state_pose.get_y(), pose.y);
    assert_eq!(state_pose.get_yaw(), pose.angle);
}

#[test]
pub fn when_map_pose_to_control_pose_should_map_correctly() {
    let pose = Pose::new();
    let control_pose = control::Pose::from(pose.clone());

    assert_eq!(control_pose.get_x(), pose.x);
    assert_eq!(control_pose.get_y(), pose.y);
    assert_eq!(control_pose.get_yaw(), pose.angle);
}