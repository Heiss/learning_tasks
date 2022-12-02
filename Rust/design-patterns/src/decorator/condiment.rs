use crate::decorator::Beverage;

pub trait CondimentDecorator: Beverage {
    fn get_beverage(&self) -> &Box<dyn Beverage>;
}

pub struct Mocha {
    beverage: Box<dyn Beverage>,
}

impl Mocha {
    #[allow(dead_code)]
    pub fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        Box::new(Self { beverage })
    }
}

impl Beverage for Mocha {
    fn get_description(&self) -> String {
        format!("{}, Mocha", self.beverage.get_description().clone())
    }

    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.2
    }
}

impl CondimentDecorator for Mocha {
    fn get_beverage(&self) -> &Box<dyn Beverage> {
        &self.beverage
    }
}

pub struct SteamedMilk {
    beverage: Box<dyn Beverage>,
}

impl SteamedMilk {
    #[allow(dead_code)]
    pub fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        Box::new(Self { beverage })
    }
}

impl Beverage for SteamedMilk {
    fn get_description(&self) -> String {
        format!("{}, Steamed Milk", self.beverage.get_description().clone())
    }

    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.2
    }
}

impl CondimentDecorator for SteamedMilk {
    fn get_beverage(&self) -> &Box<dyn Beverage> {
        &self.beverage
    }
}

pub struct Soy {
    beverage: Box<dyn Beverage>,
}

impl Soy {
    #[allow(dead_code)]
    pub fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        Box::new(Self { beverage })
    }
}

impl Beverage for Soy {
    fn get_description(&self) -> String {
        format!("{}, Soy", self.beverage.get_description().clone())
    }

    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.15
    }
}

impl CondimentDecorator for Soy {
    fn get_beverage(&self) -> &Box<dyn Beverage> {
        &self.beverage
    }
}

pub struct Whip {
    beverage: Box<dyn Beverage>,
}

impl Whip {
    #[allow(dead_code)]
    pub fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        Box::new(Self { beverage })
    }
}

impl Beverage for Whip {
    fn get_description(&self) -> String {
        format!("{}, Whip", self.beverage.get_description().clone())
    }

    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.1
    }
}

impl CondimentDecorator for Whip {
    fn get_beverage(&self) -> &Box<dyn Beverage> {
        &self.beverage
    }
}
