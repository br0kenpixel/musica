use super::AsyncVoid;
use crate::{db::LibraryDb, events::PlayerEvent, player::GlobalPlayer};
use tauri::{State, Window};
use tokio::sync::RwLock;

#[tauri::command]
pub async fn play_track(
    window: Window,
    player: State<'_, GlobalPlayer>,
    library: State<'_, RwLock<LibraryDb>>,
    id: usize,
) -> AsyncVoid {
    let track = &library.read().await.tracks[id];

    window.emit("player", PlayerEvent::Loading).unwrap();
    // Init track
    player.play_file(&track.path).await;

    window
        .emit("player", PlayerEvent::Started(track.clone()))
        .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn pause_playback(window: Window, player: State<'_, GlobalPlayer>) -> AsyncVoid {
    player.pause().await;
    window.emit("player", PlayerEvent::Paused).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn resume_playback(window: Window, player: State<'_, GlobalPlayer>) -> AsyncVoid {
    player.resume().await;
    window.emit("player", PlayerEvent::Resumed).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn stop_playback(window: Window, player: State<'_, GlobalPlayer>) -> AsyncVoid {
    player.stop().await;
    window.emit("player", PlayerEvent::Stopped).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn set_volume(player: State<'_, GlobalPlayer>, value: u8) -> AsyncVoid {
    player.set_volume(value.clamp(0, 100)).await;

    Ok(())
}
