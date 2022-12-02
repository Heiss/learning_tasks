use std::{cell::RefCell, rc::Rc};

use super::command::Command;

#[derive(Clone, Copy)]
pub enum CeilingFanSpeed {
    High,
    Medium,
    Low,
    Off,
}

pub struct CeilingFan {
    pub speed: CeilingFanSpeed,
    location: String,
}
#[allow(dead_code)]
impl CeilingFan {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
            speed: CeilingFanSpeed::Off,
        }
    }

    pub fn high(&mut self) {
        println!("{} ceiling fan set to high speed", self.location);
        self.speed = CeilingFanSpeed::High;
    }
    pub fn medium(&mut self) {
        println!("{} ceiling fan set to medium speed", self.location);
        self.speed = CeilingFanSpeed::Medium;
    }
    pub fn low(&mut self) {
        println!("{} ceiling fan set to low speed", self.location);
        self.speed = CeilingFanSpeed::Low;
    }
    pub fn off(&mut self) {
        println!("{} ceiling fan set off", self.location);
        self.speed = CeilingFanSpeed::Off;
    }
}
#[allow(dead_code)]
pub struct CeilingFanOffCommand {}
#[allow(dead_code)]
impl CeilingFanOffCommand {
    pub fn new_cmd(ceiling_fan: Rc<RefCell<CeilingFan>>) -> CeilingFanCommand {
        CeilingFanCommand::new("CeilingFanOffCommand", CeilingFanSpeed::Off, ceiling_fan)
    }
}
#[allow(dead_code)]
pub struct CeilingFanLowCommand {}
#[allow(dead_code)]
impl CeilingFanLowCommand {
    pub fn new_cmd(ceiling_fan: Rc<RefCell<CeilingFan>>) -> CeilingFanCommand {
        CeilingFanCommand::new("CeilingFanLowCommand", CeilingFanSpeed::Low, ceiling_fan)
    }
}
#[allow(dead_code)]
pub struct CeilingFanMediumCommand {}
#[allow(dead_code)]
impl CeilingFanMediumCommand {
    pub fn new_cmd(ceiling_fan: Rc<RefCell<CeilingFan>>) -> CeilingFanCommand {
        CeilingFanCommand::new(
            "CeilingFanMediumCommand",
            CeilingFanSpeed::Medium,
            ceiling_fan,
        )
    }
}

#[allow(dead_code)]
pub struct CeilingFanHighCommand {}

#[allow(dead_code)]
impl CeilingFanHighCommand {
    pub fn new_cmd(ceiling_fan: Rc<RefCell<CeilingFan>>) -> CeilingFanCommand {
        CeilingFanCommand::new("CeilingFanHighCommand", CeilingFanSpeed::High, ceiling_fan)
    }
}

pub struct CeilingFanCommand {
    name: String,
    method: CeilingFanSpeed,
    ceiling_fan: Rc<RefCell<CeilingFan>>,
    previous_speed: Option<CeilingFanSpeed>,
}

#[allow(dead_code)]
impl CeilingFanCommand {
    pub fn new(name: &str, method: CeilingFanSpeed, ceiling_fan: Rc<RefCell<CeilingFan>>) -> Self {
        Self {
            name: name.to_string(),
            method,
            ceiling_fan,
            previous_speed: None,
        }
    }
}

impl Command for CeilingFanCommand {
    fn execute(&mut self) {
        {
            self.previous_speed = Some(self.ceiling_fan.borrow().speed);
        }

        match self.method {
            CeilingFanSpeed::High => self.ceiling_fan.borrow_mut().high(),
            CeilingFanSpeed::Medium => self.ceiling_fan.borrow_mut().medium(),
            CeilingFanSpeed::Low => self.ceiling_fan.borrow_mut().low(),
            CeilingFanSpeed::Off => self.ceiling_fan.borrow_mut().off(),
        }
    }

    fn undo(&mut self) {
        if let Some(speed) = self.previous_speed {
            match speed {
                CeilingFanSpeed::High => self.ceiling_fan.borrow_mut().high(),
                CeilingFanSpeed::Medium => self.ceiling_fan.borrow_mut().medium(),
                CeilingFanSpeed::Low => self.ceiling_fan.borrow_mut().low(),
                CeilingFanSpeed::Off => self.ceiling_fan.borrow_mut().off(),
            }
        }
    }
    fn to_string(&self) -> &str {
        self.name.as_str()
    }
}
