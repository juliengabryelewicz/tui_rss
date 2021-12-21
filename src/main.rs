use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use tui_rss::application::App;
use tui_rss::start_ui;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
