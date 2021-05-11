use oxiide::{Color, Execute, KeyBind, Keys, Layout, Window, WindowManager, Workspace};

fn main() -> oxiide::Result {
    Simple::run()?;
}

/// Model of WM itself
struct Simple {
    ws: Vec<AWorkspace>,
    cws: usize,
}

/// Model of Workspace
struct AWorkspace {
    name: String,
    idx: usize,
    layout: Layout,
    windows: Vec<AWindow>,
}

/// Model of Window
struct AWindow {
    border_width: usize,
    focused_border_color: Color,
    normal_border_color: Color,
}

enum Message {
    Spawn(String),
    SwitchWorkspace(usize),
    Exit,
}

impl WindowManager for Simple {
    type Message = Message;
    type Workspace = AWorkspace;

    /// Initial state of WM
    fn new() -> Self {
        Self {
            ws: AWorkspace::from_strings(vec!["one".to_string(), "two".to_string()]),
            cws: 0,
        }
    }

    /// Communicate with WM, Workspace, Windows, by handling messages
    fn events(&mut self, msg: Message) {
        match msg {
            Spawn(cmd) => Execute::spawn(cmd),
            SwitchWorkspace(idx) => self.cws = idx,
            Exit => self.exit(),
        }
    }

    /// Returns vector of key bindings for communicate with WM
    fn keybindings(&self) -> Vec<KeyBind<Message>> {
        vec![
            KeyBind::new(&[Keys::Mod4], Keys::Return, Spawn("alacritty")),
            KeyBind::new(&[Keys::Mod4, Keys::Alt], Keys::Escape, Exit)
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
