use crate::Event;
use serde::{Deserialize, Serialize};

use crate::v1::video_group::VideoGroupEventV1;

pub mod video_group;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "entity_type", rename_all = "snake_case")]
#[serde(rename(deserialize = "Event"))]
pub enum EventV1 {
    VideoGroup(VideoGroupEventV1),
}

impl From<EventV1> for Event {
    fn from(event: EventV1) -> Self {
        Event::V1(event)
    }
}
