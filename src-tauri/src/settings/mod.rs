mod types;

use crate::{dialog, storage};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::fs as async_fs;
use types::{HomePage, Theme};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub struct Settings {
    theme: Theme,
    home: HomePage,
}

impl Settings {
    pub fn new_or_load() -> Self {
        if let Some(loaded) = Self::load() {
            return loaded;
        }

        Self::new()
    }

    pub async fn save(&self) -> bool {
        let json = serde_json::to_string_pretty(&self).unwrap();

        async_fs::write(storage::config_path(), json).await.is_ok()
    }

    fn new() -> Self {
        let settings = Self::default();
        let json = serde_json::to_string_pretty(&settings).unwrap();

        if let Err(why) = fs::write(storage::config_path(), json) {
            dialog::error(format!(
                "Failed to create default settings configuration ({why})"
            ));
        }

        settings
    }

    fn load() -> Option<Self> {
        let json = fs::read_to_string(storage::config_path()).ok()?;

        match serde_json::from_str(&json) {
            Ok(settings) => settings,
            Err(why) => {
                dialog::error(format!(
                    "Settings configuration file could not be parsed ({why})"
                ));
            }
        }
    }
}
