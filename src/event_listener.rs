use crate::{context::Context, error::Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub enum EventType {
    Message,
    FriendMessage,
    GroupMessage,

    Command,

    Invalid(Error),
}

pub type EventHandler = dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<()>>>>;

pub struct EventListener {
    event_type: EventType,
    handler: Box<EventHandler>,

    // this will be `None` if the event_type is not `Command`
    command_name: Option<String>,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let event_type = match *self {
            EventType::Message => "message",
            EventType::FriendMessage => "friendMessage",
            EventType::GroupMessage => "groupMessage",
            EventType::Command => "command",
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
            "command" => EventType::Command,
            _ => {
                let msg = format!("Invalid message type: received `{}`, expected `message`, `friendMessage` or `groupMessage`.", event_type);
                EventType::Invalid(Error::new(&msg))
            }
        }
    }
}

impl EventListener {
    pub fn new<F, Fut>(
        event_type: EventType,
        handler: &'static F,
        command_name: Option<String>,
    ) -> Self
    where
        F: Fn(Context) -> Fut,
        Fut: Future<Output = Result<()>> + 'static,
    {
        EventListener {
            event_type,
            handler: Box::new(|ctx| Box::pin(handler(ctx))),
            command_name,
        }
    }

    pub fn event_type(&self) -> EventType {
        self.event_type.clone()
    }

    pub async fn handle(&self, ctx: Context) -> Result<()> {
        (self.handler)(ctx).await
    }

    pub fn command_name(&self) -> Option<String> {
        self.command_name.clone()
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

    type EventHandler = dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<String>>>>;
    struct EventListener {
        handler: Box<EventHandler>,
    }

    impl EventListener {
        fn new<F, Fut>(handler: &'static F) -> Self
        where
            F: Fn(Context) -> Fut,
            Fut: Future<Output = Result<String>> + 'static,
        {
            EventListener {
                handler: Box::new(|ctx| Box::pin(handler(ctx))),
            }
        }
    }

    fn store_async_function<F, Fut>(event_listeners: &mut Vec<EventListener>, handler: &'static F)
    where
        F: Fn(Context) -> Fut,
        Fut: Future<Output = Result<String>> + 'static,
    {
        event_listeners.push(EventListener::new(handler));
    }

    #[tokio::test]
    async fn store_async_function_in_vector() {
        let mut event_listeners: Vec<EventListener> = vec![];

        async fn get_context_message(ctx: Context) -> Result<String> {
            Ok(ctx.message.to_string())
        }

        store_async_function(&mut event_listeners, &get_context_message);
        store_async_function(&mut event_listeners, &|ctx| async move {
            Ok(ctx.message.to_string())
        });

        let ctx = Context {
            message: "message".to_string(),
        };

        for listener in event_listeners {
            assert_eq!(
                (listener.handler)(ctx.clone()).await.unwrap(),
                String::from("message")
            );
        }
    }
}
