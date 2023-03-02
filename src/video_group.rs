use crate::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
pub enum VideoGroupEvent {
    Created { created_at: i64 },
    Updated { created_at: i64 },
    Deleted { created_at: i64 },
}

impl From<VideoGroupEvent> for Event {
    fn from(event: VideoGroupEvent) -> Self {
        Event::VideoGroup(event)
    }
}

impl VideoGroupEvent {
    pub fn created_at(&self) -> i64 {
        match *self {
            VideoGroupEvent::Created { created_at } => created_at,
            VideoGroupEvent::Updated { created_at } => created_at,
            VideoGroupEvent::Deleted { created_at } => created_at,
        }
    }

    pub fn as_label(&self) -> String {
        let label = match self {
            VideoGroupEvent::Created { .. } => "created",
            VideoGroupEvent::Updated { .. } => "updated",
            VideoGroupEvent::Deleted { .. } => "deleted",
        };

        String::from(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Event;

    mod video_group {
        use super::*;
        use serde_json::json;

        #[test]
        fn serialize_test() {
            let video_group = VideoGroupEvent::Created {
                created_at: 1673955105514,
            };
            let event = Event::VideoGroup(video_group);

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
            let event1 = serde_json::from_str::<Event>(&json).unwrap();

            let video_group = VideoGroupEvent::Updated {
                created_at: 1673955105514,
            };
            let event2 = Event::VideoGroup(video_group);

            assert_eq!(event1, event2);
        }
    }
}
