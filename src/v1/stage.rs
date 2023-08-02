use serde::{Deserialize, Serialize};
use svc_agent::AgentId;

use crate::{v1::video_group::VideoGroupEventV1, Event, EventV1};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "UpdateJanusConfigAndSendNotificationStage"))]
pub struct UpdateJanusConfigAndSendNotificationStageV1 {
    pub backend_id: AgentId,
    pub event: VideoGroupEventV1,
}

impl From<UpdateJanusConfigAndSendNotificationStageV1> for Event {
    fn from(value: UpdateJanusConfigAndSendNotificationStageV1) -> Self {
        Event::V1(EventV1::UpdateJanusConfigAndSendNotificationStage(value))
    }
}
