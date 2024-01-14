use super::Format;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr, DurationSeconds};
use std::{path::PathBuf, time::Duration};

#[serde_as]
#[derive(Serialize, Clone)]
pub struct Track {
    pub id: usize,
    pub title: String,
    pub album: String,
    pub artist: String,
    #[serde_as(as = "DisplayFromStr")]
    pub format: Format,
    #[serde_as(as = "DurationSeconds<f64>")]
    pub length: Duration,
    #[serde(skip)]
    pub path: PathBuf,
}
