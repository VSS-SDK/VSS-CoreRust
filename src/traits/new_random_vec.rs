use traits::new_random_trait::NewRandom;

pub trait   NewRandomVec<T> where T : NewRandom {
    fn new_random_vec() -> Vec<T>;
}