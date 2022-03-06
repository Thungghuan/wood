use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{error::Error, message::ChatroomType};

pub(crate) enum EventType {
    Message,
    FriendMessage,
    GroupMessage,

    Invalid(Error),
}

pub struct EventListener {
    chatroom_type: ChatroomType,
    event_type: EventType,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let event_type = match *self {
            EventType::Message => "message",
            EventType::FriendMessage => "friendMessage",
            EventType::GroupMessage => "groupMessage",
            EventType::Invalid(_) => "invalidEvent",
        };

        write!(f, "{}", event_type)
    }
}

impl From<&str> for EventType {
    fn from(event_type: &str) -> Self {
        // match the &str to the enum variant
        match event_type {
            "message" => EventType::Message,
            "friendMessage" => EventType::FriendMessage,
            "groupMessage" => EventType::GroupMessage,
            _ => {
                let msg = format!("Invalid message type, received `{}`, expected `message`, `friendMessage` or `groupMessage`.", event_type);
                EventType::Invalid(Error::new(msg))
            }
        }
    }
}
