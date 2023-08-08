use crate::EventV1;
use serde::{Deserialize, Serialize};
use svc_agent::AgentId;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "AgentEntered"))]
pub struct AgentEnteredV1 {
    agent_id: AgentId,
}

impl From<AgentEnteredV1> for EventV1 {
    fn from(ev: AgentEnteredV1) -> Self {
        EventV1::AgentEntered(ev)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename(deserialize = "AgentLeft"))]
pub struct AgentLeftV1 {
    agent_id: AgentId,
}

impl From<AgentLeftV1> for EventV1 {
    fn from(ev: AgentLeftV1) -> Self {
        EventV1::AgentLeft(ev)
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
            let agent = AgentEnteredV1 { agent_id };
            let event = EventV1::AgentEntered(agent);

            let json = serde_json::to_string(&event).expect("serialization to string");

            assert_eq!(
                json,
                "{\"type\":\"agent_entered\",\"agent_id\":\"instance01.service_name.svc.example.org\"}"
            )
        }

        #[test]
        fn deserialize_test() {
            let agent_id = "instance01.service_name.svc.example.org";
            let json = json!(
                {
                    "type": "agent_left",
                    "agent_id": agent_id,
                }
            );
            let json = serde_json::to_string(&json).expect("serialization to string");
            let event1 = serde_json::from_str::<EventV1>(&json).unwrap();

            let agent_id = AgentId::from_str(agent_id).expect("parse agent_id");
            let agent = AgentLeftV1 { agent_id };
            let event2 = EventV1::AgentLeft(agent);

            assert_eq!(event1, event2);
        }
    }
}
