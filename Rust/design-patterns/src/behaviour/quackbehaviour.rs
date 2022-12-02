pub trait QuackBehaviour {
    fn quack(&self);
}

pub struct Quack {}

pub struct Squeak {}

pub struct MuteQuack {}

impl Default for Quack {
    fn default() -> Self {
        Self {}
    }
}

impl Default for Squeak {
    fn default() -> Self {
        Self {}
    }
}

impl Default for MuteQuack {
    fn default() -> Self {
        Self {}
    }
}

impl QuackBehaviour for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

impl QuackBehaviour for Squeak {
    fn quack(&self) {
        println!("Squeak");
    }
}

impl QuackBehaviour for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}
