use std::collections::HashMap;
use std::slice::Iter;

use crate::input::keys::Key;

// all actions defined here
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    UpRss,
    DownRss,
    UpNews,
    DownNews,
    OpenLinkNews
}

impl Action {
    // All available actions
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 6] = [Action::Quit,
        Action::UpRss,
        Action::DownRss,
        Action::UpNews,
        Action::DownNews,
        Action::OpenLinkNews];
        ACTIONS.iter()
    }

    // which keys belongs to specfic action
    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Char('q')],
            Action::UpRss => &[Key::Down],
            Action::DownRss => &[Key::Up],
            Action::UpNews => &[Key::Right],
            Action::DownNews => &[Key::Left],
            Action::OpenLinkNews => &[Key::Enter],
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        Self(actions)
    }
}
