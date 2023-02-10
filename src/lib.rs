use serde::{Deserialize, Serialize};

pub use crate::video_group::VideoGroupEvent;

pub mod video_group;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "entity_type", rename_all = "snake_case")]
pub enum Event {
    VideoGroup(VideoGroupEvent),
}

#[derive(thiserror::Error, Debug)]
pub enum EventError {
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
}
