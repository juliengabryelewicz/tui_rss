use crossterm::event;

// Represents a key from keyboard.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Enter,
    Left,
    Right,
    Up,
    Down,
    Char(char),
    Unknown,
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),

            _ => Key::Unknown,
        }
    }
}
