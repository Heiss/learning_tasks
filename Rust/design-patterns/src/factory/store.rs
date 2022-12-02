use crate::factory::pizza::Pizza;

use super::pizza::{
    ChicagoStyleCheesePizza, NYStyleCheesePizza, NYStyleClamPizza, NYStylePepperoniPizza,
    NYStyleVeggiePizza,
};

pub trait PizzaStore {
    fn create_pizza(&self, r#type: &str) -> Box<dyn Pizza>;
    fn order_pizza(&self, r#type: &str) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(r#type);

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.r#box();

        pizza
    }
}

pub struct NYStylePizzaStore {}

impl NYStylePizzaStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl PizzaStore for NYStylePizzaStore {
    fn create_pizza(&self, r#type: &str) -> Box<dyn Pizza> {
        match r#type {
            "cheese" => Box::new(NYStyleCheesePizza::new()),
            "pepperoni" => Box::new(NYStylePepperoniPizza::new()),
            "clam" => Box::new(NYStyleClamPizza::new()),
            "veggie" => Box::new(NYStyleVeggiePizza::new()),
            _ => panic!("Should not be here"),
        }
    }
}

pub struct ChicagoStylePizzaStore {}

impl ChicagoStylePizzaStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl PizzaStore for ChicagoStylePizzaStore {
    fn create_pizza(&self, r#type: &str) -> Box<dyn Pizza> {
        match r#type {
            "cheese" => Box::new(ChicagoStyleCheesePizza::new()),
            "pepperoni" => Box::new(NYStylePepperoniPizza::new()),
            "clam" => Box::new(NYStyleClamPizza::new()),
            "veggie" => Box::new(NYStyleVeggiePizza::new()),
            _ => panic!("Should not be here"),
        }
    }
}
