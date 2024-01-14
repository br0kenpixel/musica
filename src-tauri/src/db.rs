use crate::{
    dialog, storage,
    types::{Format, Track},
};
use sndfile::{ReadOptions, TagType};
use std::{fs, path::PathBuf, time::Duration};

pub struct LibraryDb {
    pub tracks: Vec<Track>,
}

impl LibraryDb {
    pub fn new() -> Self {
        let files = match fs::read_dir(storage::library_path()) {
            Ok(list) => list,
            Err(why) => {
                dialog::error(format!("Failed to open library: {why}"));
            }
        };

        let paths: Box<[PathBuf]> = files
            .into_iter()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect();

        let tracks = Self::get_tracks(&paths);

        Self { tracks }
    }

    pub async fn rescan(&mut self) {
        let result = tokio::task::spawn_blocking(Self::new).await.unwrap();
        *self = result;
    }

    fn get_tracks(files: &[PathBuf]) -> Vec<Track> {
        let mut tracklist = Vec::new();

        let unknown_creator = || "Unknown".to_string();

        for (id, file) in files.iter().enumerate() {
            let Ok(mut info) = sndfile::OpenOptions::ReadOnly(ReadOptions::Auto).from_path(file)
            else {
                continue;
            };

            let title = info
                .get_tag(TagType::Title)
                .unwrap_or_else(|| file.with_extension("").display().to_string());
            let album = info.get_tag(TagType::Album).unwrap_or_else(unknown_creator);
            let artist = info
                .get_tag(TagType::Artist)
                .unwrap_or_else(unknown_creator);

            let samplerate = info.get_samplerate();
            let n_frames = info.len().unwrap();
            let length = n_frames as f64 / samplerate as f64;
            let format = Format {
                file_format: file.extension().unwrap().into(),
                sample_rate: info.get_samplerate() as u32,
            };

            tracklist.push(Track {
                id,
                title,
                album,
                artist,
                format,
                length: Duration::from_secs_f64(length),
                path: file.clone(),
            });
        }

        tracklist
    }
}
