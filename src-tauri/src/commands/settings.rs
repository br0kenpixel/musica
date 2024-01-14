use super::{AsyncVoid, Infallible};
use crate::settings::Settings;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn get_settings(settings: State<'_, RwLock<Settings>>) -> Infallible<Settings> {
    Ok(*settings.read().await)
}

#[tauri::command]
pub async fn update_settings(settings: State<'_, RwLock<Settings>>, new: Settings) -> AsyncVoid {
    *settings.write().await = new;
    Ok(())
}

#[tauri::command]
pub async fn save_settings(settings: State<'_, RwLock<Settings>>) -> Infallible<bool> {
    Ok(settings.read().await.save().await)
}
