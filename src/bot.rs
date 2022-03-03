use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

use crate::api::Api;
use crate::context::Context;
use crate::message::{ChatroomType, MessageChain, ReceivedMessage};
use crate::Result;

pub struct BotConfig {
    qq: String,
    master_qq: String,
    pub setting_file: String,
}
impl BotConfig {
    pub fn new(qq: String, master_qq: String, setting_file: String) -> Self {
        BotConfig {
            qq,
            master_qq,
            setting_file,
        }
    }
}

pub struct Bot {
    qq: String,
    master_qq: String,
    session: String,
    api: Api,
}

impl Bot {
    pub fn new(config: BotConfig, session: &str, base_url: &str) -> Self {
        Bot {
            qq: config.qq.clone(),
            master_qq: config.master_qq.clone(),
            session: session.to_string(),
            api: Api::new(&config.qq, base_url, session),
        }
    }

    pub fn qq(&self) -> String {
        self.qq.clone()
    }

    pub fn master_qq(&self) -> String {
        self.master_qq.clone()
    }

    pub fn session(&self) -> String {
        self.session.clone()
    }

    pub async fn start_with_callback<'a, F, T>(&'a self, cb: F)
    where
        F: FnOnce(&'a Bot) -> T,
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

    async fn handler(&self, message: ReceivedMessage) -> Result<()> {
        let ctx = match message {
            ReceivedMessage::FriendMessage {
                sender,
                message_chain,
            } => Context::new(self, sender, &message_chain)?,

            ReceivedMessage::GroupMessage {
                sender,
                message_chain,
            } => Context::new(self, sender, &message_chain)?,
        };

        ctx.reply(ctx.message_chain()).await?;

        match ctx.chatroom_type() {
            ChatroomType::Friend => println!(
                "Received friend message from {}({})",
                ctx.sender_nickname(),
                ctx.sender_id()
            ),

            ChatroomType::Group => println!(
                "Received group message from {}[{}] in group: {}[{}]",
                ctx.sender_nickname(),
                ctx.sender_id(),
                ctx.chatroom_name(),
                ctx.chatroom_id()
            ),
        }

        Ok(())
    }

    pub async fn send_message(
        &self,
        chatroom_type: ChatroomType,
        target: &str,
        message_chain: MessageChain,
    ) -> Result<()> {
        self.api
            .send_message(chatroom_type, &target, message_chain)
            .await?;
        Ok(())
    }
}
