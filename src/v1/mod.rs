use crate::Event;
use serde::{Deserialize, Serialize};

pub mod ban;
pub mod video_group;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "entity_type", rename_all = "snake_case")]
#[serde(rename(deserialize = "Event"))]
pub enum EventV1 {
    VideoGroup(video_group::VideoGroupEventV1),
    BanIntent(ban::BanIntentEventV1),
    BanVideoComplete(ban::BanVideoCompleteV1),
    BanEventAccessComplete(ban::BanEventAccessCompleteV1),
    Ban(ban::BanEventV1),
    BanComplete(ban::BanCompleteEventV1),
}

impl From<EventV1> for Event {
    fn from(event: EventV1) -> Self {
        Event::V1(event)
    }
}
