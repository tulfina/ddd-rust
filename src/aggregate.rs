use crate::command::Command;
use crate::event::Event;

pub trait Aggregate<State> {
    type Commands;
    type Events;
    fn sink(&mut self, command: &Command<State>) -> Result<(), String>;
    fn apply(&mut self, event: &Event<State>) -> Result<(), String>;
    fn state(&self) -> State;
}