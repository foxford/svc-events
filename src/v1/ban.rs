use serde::{Deserialize, Serialize};
use svc_authn::AccountId;
use uuid::Uuid;

use crate::{Event, EventV1};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanIntentEvent"))]
pub struct BanIntentEventV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub user_account: AccountId,
    pub op_id: Uuid,
}

impl From<BanIntentEventV1> for Event {
    fn from(value: BanIntentEventV1) -> Self {
        Event::V1(EventV1::BanIntent(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanEvent"))]
pub struct BanEventV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub user_account: AccountId,
    pub op_id: Uuid,
}

impl From<BanEventV1> for Event {
    fn from(value: BanEventV1) -> Self {
        Event::V1(EventV1::Ban(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanCompleteEvent"))]
pub struct BanCompleteEventV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub user_account: AccountId,
    pub op_id: Uuid,
}

impl From<BanCompleteEventV1> for Event {
    fn from(value: BanCompleteEventV1) -> Self {
        Event::V1(EventV1::BanComplete(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanVideoComplete"))]
pub struct BanVideoCompleteV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub user_account: AccountId,
    pub op_id: Uuid,
    // TODO: move EventId into this crate
    // parent: EventId
}

impl From<BanVideoCompleteV1> for Event {
    fn from(value: BanVideoCompleteV1) -> Self {
        Event::V1(EventV1::BanVideoComplete(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanEventAccessComplete"))]
pub struct BanEventAccessCompleteV1 {
    pub ban: bool,
    pub classroom_id: Uuid,
    pub user_account: AccountId,
    pub op_id: Uuid,
    // TODO: move EventId into this crate
    // parent: EventId,
}

impl From<BanEventAccessCompleteV1> for Event {
    fn from(value: BanEventAccessCompleteV1) -> Self {
        Event::V1(EventV1::BanEventAccessComplete(value))
    }
}
