use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use svc_agent::AgentId;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "EventEvent"))]
pub enum EventEventV1 {
    #[serde(rename = "created")]
    TenantClaimCreated {
        #[serde(flatten)]
        event: Event,
        classroom_id: Uuid,
    },
    Created(Event),
}

impl From<EventEventV1> for crate::EventV1 {
    fn from(event: EventEventV1) -> Self {
        crate::EventV1::Event(event)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Eq, PartialEq)]
pub struct Event {
    id: Uuid,
    room_id: Uuid,
    #[serde(rename = "type")]
    kind: String,
    set: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    attribute: Option<String>,
    data: Value,
    occurred_at: i64,
    created_by: AgentId,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none",
        skip_deserializing,
        default
    )]
    deleted_at: Option<DateTime<Utc>>,
    original_occurred_at: i64,
    original_created_by: AgentId,
    removed: bool,
}
