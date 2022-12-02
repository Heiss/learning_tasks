pub struct Roof {
    pub name: String,
    _material: String,
}

#[allow(dead_code)]
impl Roof {
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn to_string(&self) -> &str {
        self.name.as_str()
    }
}

impl Default for Roof {
    fn default() -> Self {
        let material = "Wood".to_string();
        Self {
            name: format!("Roof made out of {}", material),
            _material: material,
        }
    }
}
