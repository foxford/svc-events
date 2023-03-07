use crate::v1::EventV1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
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

impl VideoGroupEventV1 {
    pub fn created_at(&self) -> i64 {
        match *self {
            VideoGroupEventV1::Created { created_at } => created_at,
            VideoGroupEventV1::Updated { created_at } => created_at,
            VideoGroupEventV1::Deleted { created_at } => created_at,
        }
    }

    pub fn as_label(&self) -> String {
        let label = match self {
            VideoGroupEventV1::Created { .. } => "created",
            VideoGroupEventV1::Updated { .. } => "updated",
            VideoGroupEventV1::Deleted { .. } => "deleted",
        };

        String::from(label)
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
