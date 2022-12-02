use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::command::{Command, NoCommand};

#[allow(dead_code)]
pub struct SimpleRemoteControl {
    slot: Option<Box<dyn Command>>,
}

#[allow(dead_code)]
impl SimpleRemoteControl {
    pub fn new() -> Self {
        Self { slot: None }
    }

    pub fn set_command(&mut self, command: Box<dyn Command>) {
        self.slot = Some(command);
    }

    pub fn button_was_pressed(&mut self) {
        match &mut self.slot {
            Some(v) => v.execute(),
            None => (),
        };
    }
}

pub struct RemoteControl {
    on_commands: [Rc<RefCell<dyn Command>>; 7],
    off_commands: [Rc<RefCell<dyn Command>>; 7],
    undo_command: Vec<Rc<RefCell<dyn Command>>>,
}

#[allow(dead_code)]
impl RemoteControl {
    pub fn new() -> Self {
        let ptr = Rc::new(RefCell::new(NoCommand::new()));
        Self {
            on_commands: [
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
            ],
            off_commands: [
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr.clone(),
                ptr,
            ],
            undo_command: Vec::new(),
        }
    }

    pub fn set_command(
        &mut self,
        slot: usize,
        on_command: Rc<RefCell<dyn Command>>,
        off_command: Rc<RefCell<dyn Command>>,
    ) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_was_pushed(&mut self, slot: usize) {
        self.on_commands[slot].borrow_mut().execute();
        self.undo_command.push(self.on_commands[slot].clone())
    }

    pub fn off_button_was_pushed(&mut self, slot: usize) {
        self.off_commands[slot].borrow_mut().execute();
    }

    pub fn undo_button_was_pushed(&mut self) {
        match self.undo_command.pop() {
            Some(v) => v.borrow_mut().undo(),
            None => println!("No undo available."),
        };
    }
}

impl Display for RemoteControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n------ Remote Control ------")?;
        let cmd_string = (0..self.on_commands.len())
            .map(|slot| {
                format!(
                    "[slot {}] {}  {}",
                    slot,
                    self.on_commands[slot].borrow().to_string(),
                    self.off_commands[slot].borrow().to_string()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "\n{}\n", cmd_string)
    }
}
