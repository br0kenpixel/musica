use crate::{
    dialog, storage,
    types::{Format, Track},
};
use lofty::{Accessor, AudioFile, Probe, Tag, TaggedFileExt};
use std::{fs, path::PathBuf};

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

        for (id, file) in files.iter().enumerate() {
            let Ok(probe) = Probe::open(file) else {
                continue;
            };

            let Ok(info) = probe.read() else {
                continue;
            };

            let (title, album, artist) = Self::get_tag_info(info.primary_tag());
            let properties = info.properties();

            let samplerate = properties.sample_rate().unwrap();
            let length = properties.duration();
            let format = Format {
                file_format: file.extension().unwrap().into(),
                sample_rate: samplerate,
            };

            tracklist.push(Track {
                id,
                title,
                album,
                artist,
                format,
                length,
                path: file.clone(),
            });
        }

        tracklist
    }

    fn get_tag_info(tag: Option<&Tag>) -> (String, String, String) {
        let Some(tag) = tag else {
            return (String::new(), String::new(), String::new());
        };

        let unknown_creator = || "Unknown".to_string();

        let title = tag.title().map_or_else(unknown_creator, Into::into);
        let album = tag.album().map_or_else(unknown_creator, Into::into);
        let artist = tag.artist().map_or_else(unknown_creator, Into::into);

        (title, album, artist)
    }
}
