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
    commands: Vec<String>,
}

impl Bot {
    pub fn new(config: BotConfig, session: &str, base_url: &str) -> Self {
        Bot {
            qq: config.qq.clone(),
            master_qq: config.master_qq.clone(),
            session: session.to_string(),
            api: Api::new(config.qq, base_url, session),

            event_listeners: vec![],
            commands: vec![],
        }
    }

    pub fn clone(&self) -> Self {
        Bot {
            qq: self.qq.clone(),
            master_qq: self.master_qq.clone(),
            session: self.session.clone(),
            api: self.api.clone(),

            event_listeners: vec![],
            commands: vec![],
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
        // Distinguish between common message and command
        match listener.event_type() {
            EventType::FriendMessage => {
                !ctx.is_command() && ctx.chatroom_type() == ChatroomType::Friend
            }
            EventType::GroupMessage => {
                !ctx.is_command() && ctx.chatroom_type() == ChatroomType::Group
            }
            EventType::Message => !ctx.is_command(),
            EventType::Command => {
                ctx.is_command()
                    && ctx.command_name() != ""
                    // use `bot.on("command", handler)` to handle all command
                    && ((listener.command_name() == None
                        && !self.commands.contains(&ctx.command_name().to_string()))
                        // use `bot.command("command_name", handler)` to handle specific command
                        || ctx.command_name() == listener.command_name().unwrap_or("".to_string()))
            }
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
            } => Context::new(self.clone(), sender, message_chain)?,

            ReceivedMessage::GroupMessage {
                sender,
                message_chain,
            } => Context::new(self.clone(), sender, message_chain)?,
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
            .push(EventListener::new(event_type, handler, None));
    }

    pub fn command<F, Fut>(&mut self, command_name: &str, handler: &'static F)
    where
        F: Fn(Context) -> Fut,
        Fut: Future<Output = Result<()>> + 'static,
    {
        let command_name = command_name.to_string();

        if command_name == "" {
            eprintln!("[Error] Adding an empty command.");
            return;
        }

        if !self.commands.contains(&command_name) {
            self.commands.push(command_name.clone())
        } else {
            eprintln!("[Error] Adding a duplicate command {}.", command_name);
            return;
        }

        let event_type = EventType::from("command");

        self.event_listeners
            .push(EventListener::new(event_type, handler, Some(command_name)));
    }
}
