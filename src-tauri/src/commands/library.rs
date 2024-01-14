use super::{AsyncVoid, Infallible};
use crate::{db::LibraryDb, types::Track};
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn get_library(library: State<'_, RwLock<LibraryDb>>) -> Infallible<Vec<Track>> {
    Ok(library.read().await.tracks.clone())
}

#[tauri::command]
pub async fn reload_library(library: State<'_, RwLock<LibraryDb>>) -> AsyncVoid {
    library.write().await.rescan().await;
    Ok(())
}
