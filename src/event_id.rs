use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventId {
    entity_type: String,
    operation: String,
    sequence_id: i64,
}

impl EventId {
    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn sequence_id(&self) -> i64 {
        self.sequence_id
    }
}

impl From<(String, String, i64)> for EventId {
    fn from((entity_type, operation, sequence_id): (String, String, i64)) -> Self {
        Self {
            entity_type,
            operation,
            sequence_id,
        }
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}_{}_{}",
            self.entity_type, self.operation, self.sequence_id
        )
    }
}
