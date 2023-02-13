use crate::{Event, EventError, Parse};
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
    pub entity_event_id: i64,
}

impl Parse for VideoGroupEvent {
    type Item = (&'static str, VideoGroupPayload);

    fn parse(value: &[u8]) -> Result<Self::Item, EventError> {
        let event = serde_json::from_slice::<Event>(value).map_err(EventError::DeserializeError)?;

        let (label, payload) = match event {
            Event::VideoGroup(VideoGroupEvent::Created(payload)) => ("created", payload),
            Event::VideoGroup(VideoGroupEvent::Updated(payload)) => ("updated", payload),
            Event::VideoGroup(VideoGroupEvent::Deleted(payload)) => ("deleted", payload),
        };

        Ok((label, payload))
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
            let payload = VideoGroupPayload {
                created_at: 1673955105514,
                entity_event_id: 1,
            };
            let video_group = VideoGroupEvent::Created(payload);
            let event = Event::VideoGroup(video_group);

            let json = serde_json::to_string(&event).expect("serialization to string");

            assert_eq!(
                json,
                "{\"entity_type\":\"video_group\",\"label\":\"created\",\"created_at\":1673955105514,\"entity_event_id\":1}"
            )
        }

        #[test]
        fn deserialize_test() {
            let json = json!(
                {
                    "entity_type": "video_group",
                    "label": "updated",
                    "created_at": 1673955105514 as i64,
                    "entity_event_id": 1 as i64
                }
            );
            let json = serde_json::to_string(&json).expect("serialization to string");

            let event1 = serde_json::from_str::<Event>(&json).unwrap();

            let payload = VideoGroupPayload {
                created_at: 1673955105514,
                entity_event_id: 1,
            };
            let video_group = VideoGroupEvent::Updated(payload);
            let event2 = Event::VideoGroup(video_group);

            assert_eq!(event1, event2);
        }
    }
}
