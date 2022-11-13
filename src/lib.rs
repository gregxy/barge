mod config;
mod inner;
pub mod messaging;
pub use config::{Config, ConfigError};

use inner::BargeCore;
use std::sync::Arc;

pub struct Barge {
    core: Arc<BargeCore>,
}

impl Barge {
    pub fn new(config: Config) -> errs::Result<Self> {
        config.validate()?;

        Ok(Self {
            core: Arc::new(BargeCore::new(config)),
        })
    }
}
