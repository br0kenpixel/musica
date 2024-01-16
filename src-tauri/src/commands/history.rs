use super::{AsyncVoid, Infallible};
use crate::{db::LibraryDb, history::HistoryDb, types::Track};
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn get_history(
    history: State<'_, RwLock<HistoryDb>>,
    library: State<'_, RwLock<LibraryDb>>,
) -> Infallible<Vec<Track>> {
    let historyref = history.read().await;
    let libraryref = library.read().await;

    Ok(historyref.get(&libraryref))
}

#[tauri::command]
pub async fn update_history(
    history: State<'_, RwLock<HistoryDb>>,
    library: State<'_, RwLock<LibraryDb>>,
    with: usize,
) -> AsyncVoid {
    let mut historyref = history.write().await;
    let libraryref = library.read().await;

    let path = &libraryref
        .tracks
        .iter()
        .find(|track| track.id == with)
        .unwrap()
        .path;

    historyref.push(path);
    historyref.save().await;

    Ok(())
}

#[tauri::command]
pub async fn clear_history(history: State<'_, RwLock<HistoryDb>>) -> AsyncVoid {
    let mut historyref = history.write().await;
    historyref.clear().await;

    Ok(())
}
