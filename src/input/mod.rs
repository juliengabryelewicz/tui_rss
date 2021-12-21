use self::keys::Key;

pub mod events;
pub mod keys;

pub enum InputEvent {
    Input(Key)
}
