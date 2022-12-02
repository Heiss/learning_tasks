pub struct Window {
    pub name: String,
    _material: String,
}

impl Window {
    pub fn _set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn _to_string(&self) -> &str {
        self.name.as_str()
    }
}

impl Default for Window {
    fn default() -> Self {
        let material = "Wood".to_string();
        Self {
            name: format!("Windows made out of {}", material),
            _material: material,
        }
    }
}
