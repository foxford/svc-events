use crate::{serde::segments::Segments, EventV1};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use svc_agent::AgentId;
use svc_error::Error as SvcError;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "EditionEvent"))]
pub enum EditionEventV1 {
    Created {
        id: Uuid,
        source_room_id: Uuid,
        created_by: AgentId,
        #[serde(with = "chrono::serde::ts_seconds")]
        created_at: DateTime<Utc>,
    },
    Committed {
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Value>,
        #[serde(flatten)]
        result: EditionCommitResult,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum EditionCommitResult {
    Success {
        source_room_id: Uuid,
        committed_room_id: Uuid,
        #[serde(with = "crate::serde::segments")]
        modified_segments: Segments,
    },
    Error {
        error: SvcError,
    },
}

impl From<EditionEventV1> for EventV1 {
    fn from(event: EditionEventV1) -> Self {
        EventV1::Edition(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn serialize_test() {
        let agent_id =
            AgentId::from_str("instance01.service_name.svc.example.org").expect("parse agent_id");
        let event = EditionEventV1::Created {
            id: Default::default(),
            source_room_id: Default::default(),
            created_by: agent_id,
            created_at: Default::default(),
        };
        let event = EventV1::Edition(event);

        let json = serde_json::to_string(&event).expect("serialization to string");
        let expected = r#"{"entity_type":"edition","label":"created","id":"00000000-0000-0000-0000-000000000000","source_room_id":"00000000-0000-0000-0000-000000000000","created_by":"instance01.service_name.svc.example.org","created_at":0}"#;

        assert_eq!(json, expected)
    }

    #[test]
    fn deserialize_test() {
        let agent_id = "instance01.service_name.svc.example.org";
        let json = json!(
            {
                "entity_type": "edition",
                "label": "created",
                "id": Uuid::default(),
                "source_room_id": Uuid::default(),
                "created_by": agent_id,
                "created_at": 0
            }
        );
        let json = serde_json::to_string(&json).expect("serialization to string");
        let event1 = serde_json::from_str::<EventV1>(&json).unwrap();

        let agent_id = AgentId::from_str(agent_id).expect("parse agent_id");
        let event = EditionEventV1::Created {
            id: Default::default(),
            source_room_id: Default::default(),
            created_by: agent_id,
            created_at: Default::default(),
        };
        let event2 = EventV1::Edition(event);

        assert_eq!(event1, event2);
    }
}
