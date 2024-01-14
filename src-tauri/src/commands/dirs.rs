use crate::storage;
use std::path::PathBuf;

#[tauri::command]
pub fn get_app_dir() -> PathBuf {
    storage::app_dir()
}

#[tauri::command]
pub fn get_library_path() -> PathBuf {
    storage::library_path()
}

#[tauri::command]
pub fn get_config_path() -> PathBuf {
    storage::config_path()
}
