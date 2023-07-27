use serde::{Deserialize, Serialize};
use svc_agent::AgentId;
use svc_authn::AccountId;

use crate::{Event, EventV1};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "UpdateJanusConfigStage"))]
pub struct UpdateJanusConfigStageV1 {
    pub backend_id: AgentId,
    pub target_account: AccountId,
}

impl From<UpdateJanusConfigStageV1> for Event {
    fn from(value: UpdateJanusConfigStageV1) -> Self {
        Event::V1(EventV1::UpdateJanusConfigStage(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "SendNotificationStage"))]
pub struct SendNotificationStageV1 {}

impl From<SendNotificationStageV1> for Event {
    fn from(value: SendNotificationStageV1) -> Self {
        Event::V1(EventV1::SendNotificationStage(value))
    }
}
