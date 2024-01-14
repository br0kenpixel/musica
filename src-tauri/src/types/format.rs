use serde::Serialize;
use std::{ffi::OsStr, fmt::Display};

#[derive(Serialize, Clone)]
pub enum FileFormat {
    Flac,
    Wav,
    Mp3,
    Ogg,
    Unknown(Box<str>),
}

#[derive(Serialize, Clone)]
pub struct Format {
    pub file_format: FileFormat,
    pub sample_rate: u32,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{:.02}kHz",
            self.file_format,
            self.sample_rate as f32 / 1000.0
        )
    }
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flac => write!(f, "FLAC"),
            Self::Wav => write!(f, "WAV"),
            Self::Mp3 => write!(f, "MP3"),
            Self::Ogg => write!(f, "OGG"),
            Self::Unknown(other) => write!(f, "{other}"),
        }
    }
}

impl From<&str> for FileFormat {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "flac" => Self::Flac,
            "wav" => Self::Wav,
            "mp3" => Self::Mp3,
            "ogg" => Self::Ogg,
            other => Self::Unknown(other.into()),
        }
    }
}

impl From<&OsStr> for FileFormat {
    fn from(value: &OsStr) -> Self {
        value.to_string_lossy().as_ref().into()
    }
}
