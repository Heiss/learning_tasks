pub trait Pizza {
    fn prepare(&self) {
        println!(
            "Preparing {}\nTossing dough {}...\nAdding sauce {}...\nAdding toppings: {}",
            self.get_name(),
            self.get_dough(),
            self.get_sauce(),
            self.get_toppings().join(" ")
        );
    }
    fn bake(&self) {
        println!("Bake for 25 minutes at 350");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagnonal slices");
    }
    fn r#box(&self) {
        println!("Place pizza in official PizzaStore box");
    }
    fn get_name(&self) -> &str;
    fn get_dough(&self) -> &str;
    fn get_sauce(&self) -> &str;
    fn get_toppings(&self) -> &Vec<String>;
}

pub struct NYStyleCheesePizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}

impl NYStyleCheesePizza {
    pub fn new() -> Self {
        Self {
            name: "NY Style Sauce and Cheese Pizza".to_string(),
            dough: "Thin Crust Dough".to_string(),
            sauce: "Marinara Sauce".to_string(),
            toppings: vec!["Grated Reggiano Cheese".to_string()],
        }
    }
}

impl<'a> Pizza for NYStyleCheesePizza {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_dough(&self) -> &str {
        self.dough.as_str()
    }

    fn get_sauce(&self) -> &str {
        self.sauce.as_str()
    }

    fn get_toppings(&self) -> &Vec<String> {
        &self.toppings
    }
}
pub struct NYStylePepperoniPizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}
pub struct NYStyleClamPizza {}
pub struct NYStyleVeggiePizza {}

impl NYStylePepperoniPizza {
    pub fn new() -> Self {
        Self {
            name: "Chicago Style Deep Dish Cheese Pizza".to_string(),
            dough: "Extra Thich Crust Dough".to_string(),
            sauce: "Plum Tomato Sauce".to_string(),
            toppings: vec!["Shredded Mozzarella Cheese".to_string()],
        }
    }
}

impl Pizza for NYStylePepperoniPizza {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_dough(&self) -> &str {
        self.dough.as_str()
    }

    fn get_sauce(&self) -> &str {
        self.sauce.as_str()
    }
    fn get_toppings(&self) -> &Vec<String> {
        &self.toppings
    }
}

impl NYStyleClamPizza {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pizza for NYStyleClamPizza {
    fn get_name(&self) -> &str {
        todo!()
    }

    fn get_dough(&self) -> &str {
        todo!()
    }

    fn get_sauce(&self) -> &str {
        todo!()
    }

    fn get_toppings(&self) -> &Vec<String> {
        todo!()
    }
}
impl NYStyleVeggiePizza {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pizza for NYStyleVeggiePizza {
    fn get_name(&self) -> &str {
        todo!()
    }

    fn get_dough(&self) -> &str {
        todo!()
    }

    fn get_sauce(&self) -> &str {
        todo!()
    }

    fn get_toppings(&self) -> &Vec<String> {
        todo!()
    }
}

pub struct ChicagoStyleCheesePizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}

impl ChicagoStyleCheesePizza {
    pub fn new() -> Self {
        Self {
            name: "Chicago Style Deep Dish Cheese Pizza".to_string(),
            dough: "Extra Thich Crust Dough".to_string(),
            sauce: "Plum Tomato Sauce".to_string(),
            toppings: vec!["Shredded Mozzarella Cheese".to_string()],
        }
    }
}

impl<'a> Pizza for ChicagoStyleCheesePizza {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_dough(&self) -> &str {
        self.dough.as_str()
    }

    fn get_sauce(&self) -> &str {
        self.sauce.as_str()
    }

    fn get_toppings(&self) -> &Vec<String> {
        &self.toppings
    }

    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}
