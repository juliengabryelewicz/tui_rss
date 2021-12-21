use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;

use application::{App, AppReturn};
use eyre::Result;
use input::events::Events;
use input::InputEvent;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::application::ui;

pub mod application;
pub mod input;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        terminal.draw(|rect| ui::draw(rect, &app))?;

        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
        };
        // Check if we ask for quitting application
        if result == AppReturn::Quit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
