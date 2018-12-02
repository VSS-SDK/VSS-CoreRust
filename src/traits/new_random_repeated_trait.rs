use traits::new_random_trait::NewRandom;
use protobuf::RepeatedField;

pub trait NewRandomRepeatedField<T> where T : NewRandom {
    fn new_random() -> RepeatedField<T>;
}