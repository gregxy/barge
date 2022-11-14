use parking_lot::Mutex;

use crate::config::Config;

#[derive(strum::Display, Debug, PartialEq)]
pub(crate) enum Role {
    Follower,
    Candidate,
    Leader,
}

pub(crate) struct State {
    pub role: Role,
    pub tick: u64,
    pub term: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            role: Role::Follower,
            tick: 0,
            term: 0,
        }
    }
}

pub(crate) struct BargeCore {
    pub state: Mutex<State>,
    pub config: Config,
}

unsafe impl Send for BargeCore {}
unsafe impl Sync for BargeCore {}

impl BargeCore {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            state: Mutex::new(State::default()),
            config
        }
    }
}
