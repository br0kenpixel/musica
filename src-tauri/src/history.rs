use crate::{db::LibraryDb, dialog, storage::history_path, types::Track};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::fs as async_fs;

#[derive(Serialize, Deserialize)]
pub struct HistoryDb(Vec<PathBuf>);

impl HistoryDb {
    pub fn new() -> Self {
        if Self::exists() {
            Self::load()
        } else {
            Self::create_new()
        }
    }

    pub fn get(&self, library: &LibraryDb) -> Vec<Track> {
        self.0
            .iter()
            .map(|path| {
                library
                    .tracks
                    .iter()
                    .find(|track| &track.path == path)
                    .unwrap()
            })
            .cloned()
            .collect()
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        let pbuf = path.as_ref().to_path_buf();

        self.0.retain(|path| path != &pbuf);
        self.0.insert(0, path.as_ref().to_path_buf());
    }

    pub fn cleanup_with(&mut self, library: &LibraryDb) {
        self.0
            .retain(|path| library.tracks.iter().any(|track| &track.path == path));
    }

    pub async fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();

        if let Err(why) = async_fs::write(history_path(), json).await {
            dialog::error(format!("Failed to save history file: {why}"));
        }
    }

    pub async fn clear(&mut self) {
        self.0.clear();
        self.save().await;
    }

    fn create_new() -> Self {
        let new = Self(Vec::new());
        let json = serde_json::to_string_pretty(&new).unwrap();

        if let Err(why) = fs::write(history_path(), json) {
            dialog::error(format!("Failed to create history file: {why}"));
        }

        new
    }

    fn load() -> Self {
        let json = fs::read_to_string(history_path()).unwrap();

        serde_json::from_str(&json).unwrap()
    }

    fn exists() -> bool {
        history_path().exists()
    }
}
