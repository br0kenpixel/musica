use crate::dialog;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    tween::Tween,
};
use std::path::Path;
use tokio::sync::RwLock;

pub struct GlobalPlayer {
    manager: RwLock<AudioManager<DefaultBackend>>,
    current: RwLock<Option<StaticSoundHandle>>,
}

impl GlobalPlayer {
    pub fn new() -> Self {
        let manager = match AudioManager::new(AudioManagerSettings::default()) {
            Ok(man) => man,
            Err(why) => {
                dialog::error(format!(
                    "Failed to initialize default audio backend. ({why})"
                ));
            }
        };

        Self {
            manager: RwLock::new(manager),
            current: RwLock::new(None),
        }
    }

    pub async fn play_file<P: AsRef<Path>>(&self, file: P) {
        let path = file.as_ref();
        let sound_data = StaticSoundData::from_file(path, StaticSoundSettings::default()).unwrap();

        self.stop().await;
        let handle = self.manager.write().await.play(sound_data.clone()).unwrap();

        *self.current.write().await = Some(handle);
    }

    pub async fn pause(&self) {
        self.manager.read().await.pause(Tween::default()).unwrap();
    }

    pub async fn resume(&self) {
        self.manager.read().await.resume(Tween::default()).unwrap();
    }

    #[allow(clippy::significant_drop_tightening)]
    pub async fn set_volume(&self, percent: u8) {
        let mut handleref = self.current.write().await;
        let Some(handle) = handleref.as_mut() else {
            return;
        };

        let value = (percent as f64) / 100.0;
        handle.set_volume(value, Tween::default()).unwrap();
    }

    pub async fn stop(&self) {
        let mut handleref = self.current.write().await;

        if let Some(handle) = handleref.as_mut() {
            handle.stop(Tween::default()).unwrap();
        }

        *handleref = None;
    }
}
