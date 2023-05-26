use crate::EventV1;
use serde::{Deserialize, Serialize};
use svc_agent::AgentId;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "label", rename_all = "snake_case")]
#[serde(rename(deserialize = "AgentEvent"))]
pub enum AgentEventV1 {
    Enter { agent_id: AgentId },
    Leave { agent_id: AgentId },
}

impl From<AgentEventV1> for EventV1 {
    fn from(event: AgentEventV1) -> Self {
        EventV1::Agent(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod agent {
        use super::*;
        use serde_json::json;
        use std::str::FromStr;

        #[test]
        fn serialize_test() {
            let agent_id = AgentId::from_str("instance01.service_name.svc.example.org")
                .expect("parse agent_id");
            let agent = AgentEventV1::Enter { agent_id };
            let event = EventV1::Agent(agent);

            let json = serde_json::to_string(&event).expect("serialization to string");

            assert_eq!(
                json,
                "{\"entity_type\":\"agent\",\"label\":\"enter\",\"agent_id\":\"instance01.service_name.svc.example.org\"}"
            )
        }

        #[test]
        fn deserialize_test() {
            let agent_id = "instance01.service_name.svc.example.org";
            let json = json!(
                {
                    "entity_type": "agent",
                    "label": "leave",
                    "agent_id": agent_id,
                }
            );
            let json = serde_json::to_string(&json).expect("serialization to string");
            let event1 = serde_json::from_str::<EventV1>(&json).unwrap();

            let agent_id = AgentId::from_str(agent_id).expect("parse agent_id");
            let agent = AgentEventV1::Leave { agent_id };
            let event2 = EventV1::Agent(agent);

            assert_eq!(event1, event2);
        }
    }
}
