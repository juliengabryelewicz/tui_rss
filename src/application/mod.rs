use self::actions::Actions;
use self::state::State;
use crate::application::actions::Action;
use crate::input::keys::Key;
pub mod actions;
pub mod state;
pub mod ui;

// Continue rendering or quit the application
#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Quit,
    Continue,
}

// Our main application
pub struct App {
    actions: Actions,
    state: State,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let actions = vec![Action::Quit, Action::UpRss, Action::DownRss, Action::UpNews, Action::DownNews, Action::OpenLinkNews].into();
        let mut state = State::initialized();
        state.read_news_from_rss();
        Self { actions,  state }
    }

    /// We define what to do with our key actions
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Quit,
                Action::UpRss => {
                    self.state.up_rss();
                    self.state.read_news_from_rss();
                    self.state.initialize_news();
                    AppReturn::Continue
                },
                Action::DownRss => {
                    self.state.down_rss();
                    self.state.read_news_from_rss();
                    self.state.initialize_news();
                    AppReturn::Continue
                },
                Action::UpNews => {
                    self.state.up_news();
                    AppReturn::Continue
                },
                Action::DownNews => {
                    self.state.down_news();
                    AppReturn::Continue
                },
                Action::OpenLinkNews => {
                    self.state.open_browser();
                    AppReturn::Continue
                }
            }
        } else {
            AppReturn::Continue
        }
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    
}
