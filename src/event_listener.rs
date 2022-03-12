use crate::{context::Context, error::Error};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum EventType {
    Message,
    FriendMessage,
    GroupMessage,

    Invalid(Error),
}

// pub type EventHandler = dyn Fn(Context) -> dyn Future<Output = Result<()>>;
pub type EventHandler = dyn Fn(&Context);

pub struct EventListener {
    event_type: EventType,
    handler: Box<EventHandler>,
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
                let msg = format!("Invalid message type: received `{}`, expected `message`, `friendMessage` or `groupMessage`.", event_type);
                EventType::Invalid(Error::new(msg))
            }
        }
    }
}

impl EventListener {
    pub fn new(event_type: EventType, handler: &'static EventHandler) -> Self {
        EventListener {
            event_type,
            handler: Box::new(handler),
        }
    }

    pub fn event_type(&self) -> String {
        format!("{}", self.event_type)
    }

    pub fn handle(&self, ctx: &Context) {
        (self.handler)(ctx);
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use crate::Result;

    #[derive(Clone)]
    struct Context {
        message: String,
    }

    type EventHandler =
        Box<dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<String>>>>>;

    struct EventListener {
        event_type: &'static str,
        handler: EventHandler,
    }

    async fn get_context_message(ctx: Context) -> Result<String> {
        Ok(ctx.message.to_string())
    }

    #[tokio::test]
    async fn store_async_function_in_vector() {
        let mut event_listeners: Vec<EventListener> = vec![];

        event_listeners.push(EventListener {
            event_type: "message",
            handler: Box::new(|ctx| Box::pin(get_context_message(ctx))),
        });

        let ctx = Context {
            message: "message".to_string(),
        };

        for listener in event_listeners {
            match listener.event_type {
                "message" => assert_eq!(
                    (listener.handler)(ctx.clone()).await.unwrap(),
                    String::from("message")
                ),
                _ => continue,
            }
        }
    }
}
