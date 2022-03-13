use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

use crate::api::Api;
use crate::context::Context;
use crate::event_listener::{EventListener, EventType};
use crate::message::{ChatroomType, MessageChain, ReceivedMessage};
use crate::utils::BotConfig;
use crate::Result;

pub struct Bot {
    qq: i64,
    master_qq: i64,
    session: String,
    api: Api,

    event_listeners: Vec<EventListener>,
}

impl Bot {
    pub fn new(config: BotConfig, session: &str, base_url: &str) -> Self {
        Bot {
            qq: config.qq.clone(),
            master_qq: config.master_qq.clone(),
            session: session.to_string(),
            api: Api::new(config.qq, base_url, session),

            event_listeners: vec![],
        }
    }

    pub fn clone(&self) -> Self {
        Bot {
            qq: self.qq.clone(),
            master_qq: self.master_qq.clone(),
            session: self.session.clone(),
            api: self.api.clone(),

            event_listeners: vec![],
        }
    }

    pub fn qq(&self) -> i64 {
        self.qq
    }

    pub fn master_qq(&self) -> i64 {
        self.master_qq
    }

    pub fn session(&self) -> String {
        self.session.clone()
    }

    pub async fn start_with_callback<'a, F, T>(&'a self, cb: F)
    where
        F: Fn(&'a Bot) -> T,
        T: Future<Output = Result<()>>,
    {
        // If error occurred, the bot will not start.
        let mut will_bot_start = match self.api.link().await {
            Ok(_) => true,
            Err(e) => {
                println!(
                    "[Error] Linking session to qq.\n{}\nThe bot won't start.",
                    e
                );
                false
            }
        };

        if will_bot_start {
            will_bot_start = match cb(self).await {
                Ok(_) => true,
                Err(e) => {
                    println!(
                        "[Error] Executing bot start callback.\n{}\nThe bot won't start.",
                        e
                    );
                    false
                }
            };
        }

        if will_bot_start {
            tokio::select! {
                _ = async {
                    self.listen().await;
                } => {}
                _ = tokio::signal::ctrl_c() => {
                    println!("\nCtrl+C received.\nReleasing session...");
                }
            }
        }

        match self.api.release().await {
            Ok(_) => println!("88"),
            Err(e) => {
                eprintln!("[Error] Releasing bot session.\n{}", e);
            }
        }
    }

    pub async fn start(&self) {
        async fn basic_start_callback(_bot: &Bot) -> Result<()> {
            Ok(())
        }
        self.start_with_callback(basic_start_callback).await;
    }

    async fn listen(&self) {
        println!("The bot is running...");

        loop {
            let messages = match self.api.fetch_messages().await {
                Ok(messages) => messages,
                Err(e) => {
                    eprintln!("[Error] Fetching message.\n{}", e);
                    vec![]
                }
            };

            for message in messages {
                if let Err(e) = self.handler(message).await {
                    eprintln!("[Error] Handling message.\n{}", e);
                }
            }

            // fetch messages for every second.
            sleep(Duration::from_secs(1)).await;
        }
    }

    fn will_handle(&self, ctx: &Context, listener: &EventListener) -> bool {
        match listener.event_type() {
            EventType::FriendMessage => ctx.chatroom_type() == ChatroomType::Friend,
            EventType::GroupMessage => ctx.chatroom_type() == ChatroomType::Group,
            EventType::Message => true,
            EventType::Command => ctx.is_command(),
            _ => false,
        }
    }

    async fn handler(&self, message: ReceivedMessage) -> Result<()> {
        // Fix the f**king lifetime error by just cloning it
        // instead of borrowing it.
        let ctx = match message {
            ReceivedMessage::FriendMessage {
                sender,
                message_chain,
            } => Context::new(self.clone(), sender, &message_chain)?,

            ReceivedMessage::GroupMessage {
                sender,
                message_chain,
            } => Context::new(self.clone(), sender, &message_chain)?,
        };

        for listener in &self.event_listeners {
            let will_handle = self.will_handle(&ctx, listener);

            if will_handle {
                listener.handle(ctx.clone()).await.unwrap()
            }
        }

        Ok(())
    }

    pub async fn send_message(
        &self,
        chatroom_type: ChatroomType,
        target: i64,
        message_chain: MessageChain,
        quote: Option<i64>,
    ) -> Result<()> {
        self.api
            .send_message(chatroom_type, target, message_chain, quote)
            .await?;
        Ok(())
    }

    pub fn on<F, Fut>(&mut self, event_type: &str, handler: &'static F)
    where
        F: Fn(Context) -> Fut,
        Fut: Future<Output = Result<()>> + 'static,
    {
        let event_type = EventType::from(event_type);

        if let EventType::Invalid(e) = event_type {
            eprintln!("[Error] Adding event handler.\n{}", e);
            return;
        }

        self.event_listeners
            .push(EventListener::new(event_type, handler));
    }
}
