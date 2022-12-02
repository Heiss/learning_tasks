pub trait FlyBehaviour {
    fn fly(&self);
}

pub struct FlyWithWings {}

impl Default for FlyWithWings {
    fn default() -> Self {
        Self {}
    }
}

pub struct FlyNoWay {}

impl Default for FlyNoWay {
    fn default() -> Self {
        Self {}
    }
}

pub struct FlyRocketPowered {}

impl Default for FlyRocketPowered {
    fn default() -> Self {
        Self {}
    }
}

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying!!");
    }
}

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly");
    }
}

impl FlyBehaviour for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket!");
    }
}
