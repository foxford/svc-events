use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Event, EventId, EventV1};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "UpdateJanusConfigStage"))]
pub struct UpdateJanusConfigStageV1 {
    pub event_id: EventId,
    pub stage_state: Value,
}

impl From<UpdateJanusConfigStageV1> for Event {
    fn from(value: UpdateJanusConfigStageV1) -> Self {
        Event::V1(EventV1::UpdateJanusConfigStage(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "SendNatsNotificationStage"))]
pub struct SendNatsNotificationStageV1 {
    pub event_id: EventId,
    pub stage_state: Value,
}

impl From<SendNatsNotificationStageV1> for Event {
    fn from(value: SendNatsNotificationStageV1) -> Self {
        Event::V1(EventV1::SendNatsNotificationStage(value))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "SendMqttNotificationStage"))]
pub struct SendMqttNotificationStageV1 {
    pub event_id: EventId,
    pub stage_state: Value,
}

impl From<SendMqttNotificationStageV1> for Event {
    fn from(value: SendMqttNotificationStageV1) -> Self {
        Event::V1(EventV1::SendMqttNotificationStage(value))
    }
}
