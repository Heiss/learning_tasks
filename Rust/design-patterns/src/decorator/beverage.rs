pub trait Beverage {
    fn get_description(&self) -> String;
    fn cost(&self) -> f32;
}

pub struct HouseBlend {
    description: String,
}

impl HouseBlend {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {
            description: "House Blend Coffee".to_string(),
        })
    }
}

impl Beverage for HouseBlend {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f32 {
        0.89
    }
}

pub struct DarkRoast {
    description: String,
}

impl DarkRoast {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {
            description: "Dark Roast".to_string(),
        })
    }
}

impl Beverage for DarkRoast {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f32 {
        0.99
    }
}

pub struct Espresso {
    description: String,
}

impl Espresso {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {
            description: "Espresso".to_string(),
        })
    }
}

impl Beverage for Espresso {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f32 {
        1.99
    }
}

pub struct Decaf {
    description: String,
}

impl Beverage for Decaf {
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f32 {
        1.05
    }
}
