use std::rc::Rc;

use super::command::Command;

pub struct Light {
    location: String,
}

#[allow(dead_code)]
impl Light {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
        }
    }

    pub fn on(&self) {
        println!("{} light is On", self.location);
    }

    pub fn off(&self) {
        println!("{} light is Off", self.location);
    }
}

pub struct LightOnCommand {
    light: Rc<Light>,
}
#[allow(dead_code)]
impl LightOnCommand {
    pub fn new(light: Rc<Light>) -> Self {
        Self { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.on();
    }

    fn to_string(&self) -> &str {
        "LightOnCommand"
    }

    fn undo(&mut self) {
        print!("[undo] ");
        self.light.off();
    }
}

pub struct LightOffCommand {
    light: Rc<Light>,
}
#[allow(dead_code)]
impl LightOffCommand {
    pub fn new(light: Rc<Light>) -> Self {
        Self { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&mut self) {
        self.light.off()
    }

    fn to_string(&self) -> &str {
        "LightOffCommand"
    }

    fn undo(&mut self) {
        print!("[undo] ");
        self.light.on();
    }
}
