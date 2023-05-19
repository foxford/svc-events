use serde::{Deserialize, Serialize};
use svc_authn::AccountId;
use uuid::Uuid;

use crate::v1::EventV1;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanEvent"))]
pub enum BanEventV1 {
    Intent {
        ban: bool,
        classroom_id: Uuid,
        user_account: AccountId,
        op_id: Uuid,
    },
    Ban {
        ban: bool,
        classroom_id: Uuid,
        user_account: AccountId,
        op_id: Uuid,
    },
    Complete {
        ban: bool,
        classroom_id: Uuid,
        user_account: AccountId,
        op_id: Uuid,
    },
}

impl From<BanEventV1> for EventV1 {
    fn from(event: BanEventV1) -> Self {
        EventV1::Ban(event)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanVideoComplete"))]
pub struct BanVideoCompleteV1 {
    ban: bool,
    classroom_id: Uuid,
    user_account: AccountId,
    op_id: Uuid,
    // TODO: move EventId into this crate
    // parent: EventId
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "BanEventAccessComplete"))]
pub struct BanEventAccessCompleteV1 {
    ban: bool,
    classroom_id: Uuid,
    user_account: AccountId,
    op_id: Uuid,
    // TODO: move EventId into this crate
    // parent: EventId,
}
