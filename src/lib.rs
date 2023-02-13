use serde::{Deserialize, Serialize};

pub use crate::video_group::VideoGroupEvent;
use crate::video_group::VideoGroupPayload;

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

impl Event {
    pub fn parse(value: &[u8]) -> Result<Self, EventError> {
        serde_json::from_slice::<Event>(value).map_err(EventError::DeserializeError)
    }

    pub fn payload(&self) -> (&str, &VideoGroupPayload) {
        match self {
            Event::VideoGroup(e) => e.payload(),
        }
    }
}
