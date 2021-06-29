use crate::keybind;
use crate::result;

pub trait WindowManager {
    type Message;
    type Workspace;
    type Window;

    fn new() -> Self;
    fn events(&mut self, message: Self::Message);
    fn keybindings(&self) -> Vec<keybind::KeyBind<Self::Message>>;
    fn run(&self) -> result::Result;
}
