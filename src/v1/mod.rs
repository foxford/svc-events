use crate::Event;
use serde::{Deserialize, Serialize};

pub mod agent;
pub mod ban;
pub mod video_group;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[serde(rename(deserialize = "Event"))]
pub enum EventV1 {
    VideoGroup(video_group::VideoGroupEventV1),
    BanIntent(ban::BanIntentV1),
    BanAccepted(ban::BanAcceptedV1),
    BanRejected(ban::BanRejectedV1),
    BanVideoStreamingCompleted(ban::BanVideoStreamingCompletedV1),
    BanCollaborationCompleted(ban::BanCollaborationCompletedV1),
    BanCompleted(ban::BanCompletedV1),
    AgentEntered(agent::AgentEnteredV1),
    AgentLeft(agent::AgentLeftV1),
}

impl From<EventV1> for Event {
    fn from(event: EventV1) -> Self {
        Event::V1(event)
    }
}
