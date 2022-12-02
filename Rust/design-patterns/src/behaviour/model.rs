use crate::behaviour::Duck;
use crate::behaviour::{FlyBehaviour, FlyNoWay};
use crate::behaviour::{Quack, QuackBehaviour};

pub struct ModelDuck {
    pub fly_behaviour: Box<dyn FlyBehaviour>,
    pub quack_behaviour: Box<dyn QuackBehaviour>,
}

impl Default for ModelDuck {
    fn default() -> Self {
        ModelDuck {
            fly_behaviour: Box::new(FlyNoWay::default()),
            quack_behaviour: Box::new(Quack {}),
        }
    }
}

impl ModelDuck {
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("I'm a model duck");
    }

    #[allow(dead_code)]
    pub fn swim(&self) {}
}

impl Duck for ModelDuck {
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
    use crate::behaviour::flybehaviour::FlyRocketPowered;

    use super::*;

    #[test]
    fn test_model() {
        let mut model = ModelDuck::default();

        model.display();
        model.perform_fly();
        model.perform_quack();
        model.set_fly_behaviour(Box::new(FlyRocketPowered {}));
        model.perform_fly();
    }
}
