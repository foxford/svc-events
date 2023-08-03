use crate::{v1::EventV1, Event};
use serde::{Deserialize, Serialize};
use svc_agent::AgentId;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "VideoGroupEvent"))]
pub enum VideoGroupEventV1 {
    Created { created_at: i64 },
    Updated { created_at: i64 },
    Deleted { created_at: i64 },
}

impl From<VideoGroupEventV1> for EventV1 {
    fn from(event: VideoGroupEventV1) -> Self {
        EventV1::VideoGroup(event)
    }
}

impl From<VideoGroupEventV1> for Event {
    fn from(value: VideoGroupEventV1) -> Self {
        Event::V1(EventV1::VideoGroup(value))
    }
}

impl From<VideoGroupIntentEventV1> for VideoGroupEventV1 {
    fn from(value: VideoGroupIntentEventV1) -> Self {
        match value {
            VideoGroupIntentEventV1::CreateIntent { created_at, .. } => {
                VideoGroupEventV1::Created { created_at }
            }
            VideoGroupIntentEventV1::UpdateIntent { created_at, .. } => {
                VideoGroupEventV1::Updated { created_at }
            }
            VideoGroupIntentEventV1::DeleteIntent { created_at, .. } => {
                VideoGroupEventV1::Deleted { created_at }
            }
        }
    }
}

impl VideoGroupEventV1 {
    pub fn created_at(&self) -> i64 {
        match *self {
            VideoGroupEventV1::Created { created_at } => created_at,
            VideoGroupEventV1::Updated { created_at } => created_at,
            VideoGroupEventV1::Deleted { created_at } => created_at,
        }
    }

    pub fn as_label(&self) -> &str {
        match self {
            VideoGroupEventV1::Created { .. } => "created",
            VideoGroupEventV1::Updated { .. } => "updated",
            VideoGroupEventV1::Deleted { .. } => "deleted",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "VideoGroupIntentEvent"))]
pub enum VideoGroupIntentEventV1 {
    CreateIntent {
        backend_id: AgentId,
        created_at: i64,
    },
    UpdateIntent {
        backend_id: AgentId,
        created_at: i64,
    },
    DeleteIntent {
        backend_id: AgentId,
        created_at: i64,
    },
}

impl VideoGroupIntentEventV1 {
    pub fn backend_id(&self) -> AgentId {
        match self {
            VideoGroupIntentEventV1::CreateIntent { backend_id, .. } => backend_id.clone(),
            VideoGroupIntentEventV1::UpdateIntent { backend_id, .. } => backend_id.clone(),
            VideoGroupIntentEventV1::DeleteIntent { backend_id, .. } => backend_id.clone(),
        }
    }
}

impl From<VideoGroupIntentEventV1> for EventV1 {
    fn from(event: VideoGroupIntentEventV1) -> Self {
        EventV1::VideoGroupIntent(event)
    }
}

impl From<VideoGroupIntentEventV1> for Event {
    fn from(value: VideoGroupIntentEventV1) -> Self {
        Event::V1(EventV1::VideoGroupIntent(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod video_group {
        use super::*;
        use serde_json::json;

        #[test]
        fn serialize_test() {
            let video_group = VideoGroupEventV1::Created {
                created_at: 1673955105514,
            };
            let event = EventV1::VideoGroup(video_group);

            let json = serde_json::to_string(&event).expect("serialization to string");

            assert_eq!(
                json,
                "{\"entity_type\":\"video_group\",\"label\":\"created\",\"created_at\":1673955105514}"
            )
        }

        #[test]
        fn deserialize_test() {
            let json = json!(
                {
                    "entity_type": "video_group",
                    "label": "updated",
                    "created_at": 1673955105514 as i64,
                }
            );
            let json = serde_json::to_string(&json).expect("serialization to string");
            let event1 = serde_json::from_str::<EventV1>(&json).unwrap();

            let video_group = VideoGroupEventV1::Updated {
                created_at: 1673955105514,
            };
            let event2 = EventV1::VideoGroup(video_group);

            assert_eq!(event1, event2);
        }
    }
}
