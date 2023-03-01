use serde::{Deserialize, Serialize};

pub use crate::video_group::{VideoGroupEvent, VideoGroupOperation, VideoGroupPayload};

pub mod video_group;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "entity_type", rename_all = "snake_case")]
pub enum Event {
    VideoGroup(VideoGroupEvent),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Operation {
    VideoGroup(VideoGroupOperation),
}

impl Operation {
    pub fn as_label(&self) -> &str {
        match self {
            Operation::VideoGroup(op) => match op {
                VideoGroupOperation::Create(_) => "created",
                VideoGroupOperation::Update(_) => "updated",
                VideoGroupOperation::Delete(_) => "deleted",
            },
        }
    }
}

impl From<Event> for Operation {
    fn from(event: Event) -> Self {
        match event {
            Event::VideoGroup(e) => match e {
                VideoGroupEvent::Created(payload) => {
                    Operation::VideoGroup(VideoGroupOperation::Create(payload.created_at))
                }
                VideoGroupEvent::Updated(payload) => {
                    Operation::VideoGroup(VideoGroupOperation::Update(payload.created_at))
                }
                VideoGroupEvent::Deleted(payload) => {
                    Operation::VideoGroup(VideoGroupOperation::Delete(payload.created_at))
                }
            },
        }
    }
}

impl From<Operation> for Event {
    fn from(operation: Operation) -> Self {
        match operation {
            Operation::VideoGroup(op) => {
                let event = match op {
                    VideoGroupOperation::Create(created_at) => {
                        VideoGroupEvent::Created(VideoGroupPayload { created_at })
                    }
                    VideoGroupOperation::Update(created_at) => {
                        VideoGroupEvent::Updated(VideoGroupPayload { created_at })
                    }
                    VideoGroupOperation::Delete(created_at) => {
                        VideoGroupEvent::Deleted(VideoGroupPayload { created_at })
                    }
                };

                Event::VideoGroup(event)
            }
        }
    }
}
