use crate::types::Track;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum PlayerEvent {
    Loading,
    Started(Track),
    Paused,
    Resumed,
    Stopped,
}
