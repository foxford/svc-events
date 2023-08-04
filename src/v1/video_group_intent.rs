use crate::{v1::EventV1, Event};
use serde::{Deserialize, Serialize};
use svc_agent::AgentId;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "VideoGroupCreateIntentEvent"))]
pub struct VideoGroupCreateIntentEventV1 {
    pub backend_id: AgentId,
    pub created_at: i64,
}

impl From<VideoGroupCreateIntentEventV1> for EventV1 {
    fn from(event: VideoGroupCreateIntentEventV1) -> Self {
        EventV1::VideoGroupCreateIntent(event)
    }
}

impl From<VideoGroupCreateIntentEventV1> for Event {
    fn from(value: VideoGroupCreateIntentEventV1) -> Self {
        Event::V1(EventV1::VideoGroupCreateIntent(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "VideoGroupUpdateIntentEvent"))]
pub struct VideoGroupUpdateIntentEventV1 {
    pub backend_id: AgentId,
    pub created_at: i64,
}

impl From<VideoGroupUpdateIntentEventV1> for EventV1 {
    fn from(event: VideoGroupUpdateIntentEventV1) -> Self {
        EventV1::VideoGroupUpdateIntent(event)
    }
}

impl From<VideoGroupUpdateIntentEventV1> for Event {
    fn from(value: VideoGroupUpdateIntentEventV1) -> Self {
        Event::V1(EventV1::VideoGroupUpdateIntent(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "VideoGroupDeleteIntentEvent"))]
pub struct VideoGroupDeleteIntentEventV1 {
    pub backend_id: AgentId,
    pub created_at: i64,
}

impl From<VideoGroupDeleteIntentEventV1> for EventV1 {
    fn from(event: VideoGroupDeleteIntentEventV1) -> Self {
        EventV1::VideoGroupDeleteIntent(event)
    }
}

impl From<VideoGroupDeleteIntentEventV1> for Event {
    fn from(value: VideoGroupDeleteIntentEventV1) -> Self {
        Event::V1(EventV1::VideoGroupDeleteIntent(value))
    }
}
