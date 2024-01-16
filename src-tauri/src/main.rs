// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(
    clippy::module_name_repetitions,
    clippy::used_underscore_binding,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::future_not_send,
    clippy::cast_lossless,
    clippy::significant_drop_tightening
)]

use db::LibraryDb;
use history::HistoryDb;
use player::GlobalPlayer;
use settings::Settings;
use std::{fs, io::ErrorKind};
use tokio::sync::RwLock;

mod commands;
mod db;
mod dialog;
mod events;
mod history;
mod player;
mod settings;
mod storage;
mod types;

fn main() {
    ensure_dirs();
    let settings = RwLock::new(Settings::new_or_load());
    let player = GlobalPlayer::new();

    let library = LibraryDb::new();
    let mut history = HistoryDb::new();
    history.cleanup_with(&library);

    let library = RwLock::new(library);
    let history = RwLock::new(history);

    tauri::Builder::default()
        .manage(player)
        .manage(settings)
        .manage(library)
        .manage(history)
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::save_settings,
            commands::dirs::get_app_dir,
            commands::dirs::get_library_path,
            commands::dirs::get_config_path,
            commands::library::get_library,
            commands::library::reload_library,
            commands::playback::play_track,
            commands::playback::pause_playback,
            commands::playback::resume_playback,
            commands::playback::stop_playback,
            commands::playback::set_volume,
            commands::exec::build_type,
            commands::history::get_history,
            commands::history::update_history,
            commands::history::clear_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn ensure_dirs() {
    let results = [
        fs::create_dir(storage::app_dir()),
        fs::create_dir(storage::library_path()),
    ];

    for result in results {
        let Err(why) = result else {
            continue;
        };

        if why.kind() == ErrorKind::AlreadyExists {
            continue;
        }

        dialog::error(format!(
            "Failed to create application data directory structure: {why}"
        ));
    }
}
