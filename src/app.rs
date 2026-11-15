use std::io;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

pub const CHANNELS: [&str; 4] = ["General", "Serial monitor", "Device logs", "Scratchpad"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    Input,
}

pub struct Message {
    pub author: &'static str,
    pub text: String,
}

pub struct App {
    pub selected: usize,
    pub focus: Focus,
    pub input: String,
    pub conversations: Vec<Vec<Message>>,
}

impl Default for App {
    fn default() -> Self {
        let conversations = [
            vec![
                ("system", "Welcome to your little corner of the terminal."),
                (
                    "Maya",
                    "The rain sounds like a thousand tiny keyboards today.",
                ),
                (
                    "Finn",
                    "Meanwhile, my coffee has successfully compiled. Zero warnings.",
                ),
                ("Maya", "Perfect. Let's build something interesting."),
                (
                    "system",
                    "Select a channel on the left, or press Tab to write a message.",
                ),
            ],
            vec![
                (
                    "system",
                    "Serial monitor preview. No hardware connection is active.",
                ),
                ("device", "[demo] /dev/ttyUSB0 ready at 115200 baud"),
                ("device", "[demo] temperature=23.4 humidity=48 status=ok"),
                (
                    "you",
                    "A good place for incoming serial data once we wire it up.",
                ),
            ],
            vec![
                ("system", "Device log preview."),
                ("device", "[demo] INFO  Boot sequence complete"),
                ("device", "[demo] DEBUG Listening for the next adventure"),
                ("device", "[demo] INFO  All systems are feeling optimistic"),
            ],
            vec![
                (
                    "system",
                    "A scratchpad for stray thoughts and questionable ideas.",
                ),
                ("you", "TODO: teach the toaster to speak Rust."),
                (
                    "you",
                    "Note: the best debugging tool is sometimes a walk outside.",
                ),
            ],
        ]
        .into_iter()
        .map(|messages| {
            messages
                .into_iter()
                .map(|(author, text)| Message {
                    author,
                    text: text.into(),
                })
                .collect()
        })
        .collect();

        Self {
            selected: 0,
            focus: Focus::Sidebar,
            input: String::new(),
            conversations,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| crate::ui::draw(frame, self))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && self.handle_key(key) {
                    return Ok(());
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }
        match key.code {
            KeyCode::Tab | KeyCode::BackTab => {
                self.focus = if self.focus == Focus::Sidebar {
                    Focus::Input
                } else {
                    Focus::Sidebar
                };
            }
            KeyCode::Esc => self.focus = Focus::Sidebar,
            _ => match self.focus {
                Focus::Sidebar => match key.code {
                    KeyCode::Char('q') => return true,
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.selected = (self.selected + 1) % CHANNELS.len();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.selected = (self.selected + CHANNELS.len() - 1) % CHANNELS.len();
                    }
                    KeyCode::Enter | KeyCode::Char('i') => self.focus = Focus::Input,
                    _ => {}
                },
                Focus::Input => match key.code {
                    KeyCode::Enter if !self.input.trim().is_empty() => {
                        self.conversations[self.selected].push(Message {
                            author: "you",
                            text: std::mem::take(&mut self.input),
                        });
                    }
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    KeyCode::Char(c)
                        if !key
                            .modifiers
                            .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
                    {
                        self.input.push(c);
                    }
                    _ => {}
                },
            },
        }
        false
    }
}
