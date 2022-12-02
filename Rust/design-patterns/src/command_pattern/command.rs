pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
    fn to_string(&self) -> &str;
}

#[derive(Clone, Copy)]
pub struct NoCommand {}

impl Command for NoCommand {
    fn execute(&mut self) {}

    fn undo(&mut self) {}

    fn to_string(&self) -> &str {
        "NoCommand"
    }
}

impl NoCommand {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for NoCommand {
    fn default() -> Self {
        NoCommand::new()
    }
}
