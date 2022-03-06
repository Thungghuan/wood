use crate::{context::Context, error::Error};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum EventType {
    Message,
    FriendMessage,
    GroupMessage,

    Invalid(Error),
}

// pub type EventHandler = dyn FnOnce(Context) -> dyn Future<Output = Result<()>>;
pub type EventHandler = Box<dyn FnOnce(Context)>;

pub struct EventListener {
    event_type: EventType,
    pub handler: EventHandler,
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

impl EventListener {
    pub fn new(event_type: EventType, handler: EventHandler) -> Self {
        EventListener {
            event_type,
            handler,
        }
    }

    pub fn event_type(&self) -> String {
        format!("{}", self.event_type)
    }
}
