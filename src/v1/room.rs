use crate::{
    serde::{segments::Segments, time::Time},
    EventV1,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use svc_agent::AgentId;
use svc_authn::AccountId;
use svc_error::Error as SvcError;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "RoomEvent"))]
pub enum RoomEventV1 {
    DumpedEvents {
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Value>,
        result: EventsDumpResult,
    },
    Created(Room),
    Updated(Room),
    Closed(Room),
    Entered {
        id: Uuid,
        agent_id: AgentId,
        banned: bool,
        agent: AgentWithBan,
    },
    Adjusted {
        room_id: Uuid,
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Value>,
        #[serde(flatten)]
        result: RoomAdjustResult,
    },
    Left {
        id: Uuid,
        agent_id: AgentId,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Room {
    id: Uuid,
    audience: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_room_id: Option<Uuid>,
    #[serde(with = "crate::serde::time")]
    time: Time,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Value>,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    preserve_history: bool,
    classroom_id: Uuid,
    #[serde(default)]
    locked_types: HashMap<String, bool>,
    #[serde(default)]
    validate_whiteboard_access: bool,
    #[serde(default)]
    whiteboard_access: HashMap<AccountId, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum EventsDumpResult {
    Success { room_id: Uuid, s3_uri: String },
    Error { error: SvcError },
}

impl From<RoomEventV1> for EventV1 {
    fn from(event: RoomEventV1) -> Self {
        EventV1::Room(event)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Eq, PartialEq)]
pub struct AgentWithBan {
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    id: Uuid,
    agent_id: AgentId,
    room_id: Uuid,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    status: Status,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "agent_status")]
pub enum Status {
    #[sqlx(rename = "in_progress")]
    InProgress,
    #[sqlx(rename = "ready")]
    Ready,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum RoomAdjustResult {
    Success {
        original_room_id: Uuid,
        modified_room_id: Uuid,
        #[serde(with = "crate::serde::segments")]
        modified_segments: Segments,
        #[serde(with = "crate::serde::segments")]
        cut_original_segments: Segments,
    },
    Error {
        error: SvcError,
    },
}
