use std::rc::Rc;

use super::command::Command;

pub struct GarageDoor {}

#[allow(dead_code)]
impl GarageDoor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn up(&self) {
        println!("Door goes up!");
    }
    pub fn down(&self) {
        println!("Door goes down!");
    }
    pub fn stop(&self) {
        println!("Door stopped!");
    }
    pub fn light_on(&self) {
        println!("Light in Garage is on!");
    }
    pub fn light_off(&self) {
        println!("Light in Garage is off!");
    }
}

pub struct GarageDoorOpenCommand {
    garage_door: Rc<GarageDoor>,
}

impl Command for GarageDoorOpenCommand {
    fn execute(&mut self) {
        self.garage_door.up();
    }

    fn to_string(&self) -> &str {
        "GarageDoorOpenCommand"
    }

    fn undo(&mut self) {
        print!("[undo] ");
        self.garage_door.down();
    }
}

#[allow(dead_code)]
impl GarageDoorOpenCommand {
    pub fn new(garage_door: Rc<GarageDoor>) -> Self {
        Self { garage_door }
    }
}

pub struct GarageDoorCloseCommand {
    garage_door: Rc<GarageDoor>,
}

impl Command for GarageDoorCloseCommand {
    fn execute(&mut self) {
        self.garage_door.down();
    }

    fn to_string(&self) -> &str {
        "GarageDoorCloseCommand"
    }

    fn undo(&mut self) {
        print!("[undo] ");
        self.garage_door.up();
    }
}

#[allow(dead_code)]
impl GarageDoorCloseCommand {
    pub fn new(garage_door: Rc<GarageDoor>) -> Self {
        Self { garage_door }
    }
}
