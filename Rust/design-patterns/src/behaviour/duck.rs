use crate::behaviour::{FlyBehaviour, QuackBehaviour};

pub trait Duck {
    fn perform_fly(&self);
    fn perform_quack(&self);
    fn set_fly_behaviour(&mut self, fb: Box<dyn FlyBehaviour>);
    fn set_quack_behaviour(&mut self, qb: Box<dyn QuackBehaviour>);
}
