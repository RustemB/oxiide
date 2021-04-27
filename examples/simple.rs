use oxiide::{Execute, KeyBind, Keys, Layout, WindowsManager, Workspace};

fn main() -> oxiide::Result {
    Simple::run()?;
}

struct Simple {
    ws: Vec<AWorkspace>,
    cws: usize,
}

struct AWorkspace {
    name: String,
    idx: usize,
    layout: Layout,
}

enum Message {
    Spawn(String),
    SwitchWorkspace(usize),
}

impl WindowsManager for Simple {
    type Message = Message;
    type Workspace = AWorkspace;

    fn new() -> Self {
        Self {
            ws: AWorkspace::from_strings(vec!["one".to_string(), "two".to_string()]),
            cws: 0,
        }
    }

    fn events(&mut self, msg: Message) {
        match msg {
            Spawn(cmd) => Execute::spawn(cmd),
            SwitchWorkspace(idx) => self.cws = idx,
        }
    }

    fn keybindings(&self) -> Vec<KeyBind<Message>> {
        vec![
            KeyBind::new(&[Keys::Mod4], Keys::Return, Spawn("alacritty")),
            KeyBind::map(
                &[Keys::Shift],
                0..(self.ws.len()),
                Message::SwitchWorkspace,
            ),
        ]
    }
}

impl Workspace for AWorkspace {
    fn new(name: String, idx: usize, layout: Layout) -> Self {
        Self { name, idx, layout }
    }

    fn from_strings(self, ws: Vec<String>) -> Vec<Self> {
        ws.iter().enumerate().map(|(idx, name)| Self {
            name,
            idx,
            layout: Layout::fibonacci,
        })
    }
}
