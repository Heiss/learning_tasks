use crate::behaviour::Duck;
use crate::behaviour::FlyBehaviour;
use crate::behaviour::QuackBehaviour;

use super::FlyWithWings;
use super::Quack;

pub struct MallardDuck {
    pub fly_behaviour: Box<dyn FlyBehaviour>,
    pub quack_behaviour: Box<dyn QuackBehaviour>,
}

impl MallardDuck {
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("I'm a real Mallard duck");
    }

    #[allow(dead_code)]
    pub fn swim(&self) {
        println!("I'm swimming")
    }
}

impl Default for MallardDuck {
    fn default() -> Self {
        MallardDuck {
            fly_behaviour: Box::new(FlyWithWings::default()),
            quack_behaviour: Box::new(Quack::default()),
        }
    }
}

impl Duck for MallardDuck {
    fn perform_fly(&self) {
        self.fly_behaviour.fly();
    }
    fn perform_quack(&self) {
        self.quack_behaviour.quack();
    }

    fn set_fly_behaviour(&mut self, fb: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fb;
    }

    fn set_quack_behaviour(&mut self, qb: Box<dyn QuackBehaviour>) {
        self.quack_behaviour = qb;
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_mallard() {
        let mallard = MallardDuck::default();

        mallard.display();
        mallard.swim();
        mallard.perform_quack();
        mallard.perform_fly();
    }
}
