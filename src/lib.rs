use serde::{Deserialize, Serialize};

pub use crate::v1::{video_group::VideoGroupEventV1, EventV1};

mod v1;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "version", rename_all = "snake_case")]
pub enum Event {
    V1(EventV1),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod video_group {
        use super::*;
        use serde_json::json;

        #[test]
        fn serialize_test() {
            let video_group_v1 = VideoGroupEventV1::Created {
                created_at: 1673955105514,
            };
            let event_v1 = EventV1::VideoGroup(video_group_v1);
            let event = Event::V1(event_v1);

            let json = serde_json::to_string(&event).expect("serialization to string");

            assert_eq!(
                json,
                "{\"version\":\"v1\",\"entity_type\":\"video_group\",\"label\":\"created\",\"created_at\":1673955105514}"
            )
        }

        #[test]
        fn deserialize_test() {
            let json = json!(
                {
                    "version": "v1",
                    "entity_type": "video_group",
                    "label": "updated",
                    "created_at": 1673955105514 as i64,
                }
            );
            let json = serde_json::to_string(&json).expect("serialization to string");
            let event1 = serde_json::from_str::<Event>(&json).unwrap();

            let video_group_v1 = VideoGroupEventV1::Updated {
                created_at: 1673955105514,
            };
            let event_v1 = EventV1::VideoGroup(video_group_v1);
            let event2 = Event::V1(event_v1);

            assert_eq!(event1, event2);
        }
    }
}
