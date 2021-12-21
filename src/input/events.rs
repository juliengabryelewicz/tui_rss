use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;

use super::keys::Key;
use super::InputEvent;

// A small event handler that wrap crossterm input. Each event
pub struct Events {
    rx: Receiver<InputEvent>,
    _sx: Sender<InputEvent>
}

impl Events {
    // Constructs an new instance of `Events` with default configuration.
    pub fn new(tick_rate: Duration) -> Events {
        let (sx, rx) = channel();
        let event_sx = sx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, do nothing.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        let key = Key::from(key);
                        event_sx.send(InputEvent::Input(key)).unwrap();
                    }
                }
            }
        });
        Events { rx, _sx: sx }
    }

    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
