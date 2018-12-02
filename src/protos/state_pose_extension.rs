use protos::state::Pose;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COORDINATE_X, MAX_COORDINATE_X};
use domain::constants::{MIN_COORDINATE_Y, MAX_COORDINATE_Y};
use domain::constants::{MIN_ANGLE_VALUE, MAX_ANGLE_VALUE};
use traits::new_random_trait::NewRandom;

impl NewRandom for Pose {
    fn new_random() -> Self {
        let mut pose = Pose::new();

        pose.set_x(thread_rng().gen_range(MIN_COORDINATE_X, MAX_COORDINATE_X));
        pose.set_y(thread_rng().gen_range(MIN_COORDINATE_Y, MAX_COORDINATE_Y));
        pose.set_yaw(thread_rng().gen_range(MIN_ANGLE_VALUE, MAX_ANGLE_VALUE));

        pose
    }
}

