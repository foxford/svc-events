use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
pub enum VideoGroupEvent {
    Created(VideoGroupPayload),
    Updated(VideoGroupPayload),
    Deleted(VideoGroupPayload),
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct VideoGroupPayload {
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum VideoGroupOperation {
    Create(i64),
    Update(i64),
    Delete(i64),
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
            let payload = VideoGroupPayload {
                created_at: 1673955105514,
            };
            let video_group = VideoGroupEvent::Created(payload);
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

            let payload = VideoGroupPayload {
                created_at: 1673955105514,
            };
            let video_group = VideoGroupEvent::Updated(payload);
            let event2 = Event::VideoGroup(video_group);

            assert_eq!(event1, event2);
        }
    }
}
