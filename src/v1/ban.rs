use serde::{Deserialize, Serialize};
use svc_agent::AgentId;
use svc_authn::AccountId;
use uuid::Uuid;

use crate::{Event, EventId, EventV1};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanIntentEvent"))]
pub struct BanIntentV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub sender: AgentId,
    pub target_account: AccountId,
    pub last_operation_id: i64,
}

impl From<BanIntentV1> for Event {
    fn from(value: BanIntentV1) -> Self {
        Event::V1(EventV1::BanIntent(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanAccepted"))]
pub struct BanAcceptedV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub target_account: AccountId,
    pub operation_id: i64,
}

impl From<BanAcceptedV1> for Event {
    fn from(value: BanAcceptedV1) -> Self {
        Event::V1(EventV1::BanAccepted(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanRejected"))]
pub struct BanRejectedV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub target_account: AccountId,
    pub operation_id: i64,
}

impl From<BanRejectedV1> for Event {
    fn from(value: BanRejectedV1) -> Self {
        Event::V1(EventV1::BanRejected(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanVideoStreamingCompleted"))]
pub struct BanVideoStreamingCompletedV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub target_account: AccountId,
    pub operation_id: i64,
    pub parent: EventId,
}

impl BanVideoStreamingCompletedV1 {
    pub fn new_from_accepted(e: BanAcceptedV1, id: EventId) -> Self {
        Self {
            ban: e.ban,
            classroom_id: e.classroom_id,
            target_account: e.target_account,
            operation_id: e.operation_id,
            parent: id,
        }
    }
}

impl From<BanVideoStreamingCompletedV1> for Event {
    fn from(value: BanVideoStreamingCompletedV1) -> Self {
        Event::V1(EventV1::BanVideoStreamingCompleted(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanCollaborationCompleted"))]
pub struct BanCollaborationCompletedV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub target_account: AccountId,
    pub operation_id: i64,
    pub parent: EventId,
}

impl BanCollaborationCompletedV1 {
    pub fn new_from_accepted(e: BanAcceptedV1, id: EventId) -> Self {
        Self {
            ban: e.ban,
            classroom_id: e.classroom_id,
            target_account: e.target_account,
            operation_id: e.operation_id,
            parent: id,
        }
    }
}

impl From<BanCollaborationCompletedV1> for Event {
    fn from(value: BanCollaborationCompletedV1) -> Self {
        Event::V1(EventV1::BanCollaborationCompleted(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "BanCompleted"))]
pub struct BanCompletedV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub target_account: AccountId,
    pub operation_id: i64,
}

impl From<BanCompletedV1> for Event {
    fn from(value: BanCompletedV1) -> Self {
        Event::V1(EventV1::BanCompleted(value))
    }
}

impl From<BanVideoStreamingCompletedV1> for BanCompletedV1 {
    fn from(b: BanVideoStreamingCompletedV1) -> Self {
        Self {
            ban: b.ban,
            classroom_id: b.classroom_id,
            target_account: b.target_account,
            operation_id: b.operation_id,
        }
    }
}

impl From<BanCollaborationCompletedV1> for BanCompletedV1 {
    fn from(b: BanCollaborationCompletedV1) -> Self {
        Self {
            ban: b.ban,
            classroom_id: b.classroom_id,
            target_account: b.target_account,
            operation_id: b.operation_id,
        }
    }
}
