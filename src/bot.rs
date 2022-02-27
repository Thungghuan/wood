use std::time::Duration;

use tokio::time::sleep;

use crate::api::Api;
use crate::message::{MessageChain, SingleMessage};
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

    pub async fn init(&self) -> Result<()> {
        println!("Bot qq is: {}", self.qq);
        println!("Master qq is: {}", self.master_qq);
        println!("Session key: {:#?}", self.session);

        // Send a start message to the master.
        let start_message = "Hello master, your bot start successfully!";
        let mut message_chain: MessageChain = vec![];
        message_chain.push(SingleMessage::Plain {
            text: start_message.to_string(),
        });

        self.api
            .send_friend_message(&self.master_qq, message_chain)
            .await?;

        Ok(())
    }

    pub async fn start(&self) {
        self.api.link().await;

        // If error occurred, the bot will not start.
        let will_bot_start = match self.init().await {
            Ok(()) => true,
            Err(e) => {
                println!("{}\nThe bot will stop.", e);
                false
            }
        };

        if will_bot_start {
            tokio::select! {
                _ = async {
                    loop {
                        println!("The bot is running...");
                        sleep(Duration::from_secs(1)).await;
                    }
                } => {}
                _ = tokio::signal::ctrl_c() => {
                    println!("\nCtrl+C received.\nReleasing session...");
                }
            }
        }

        self.api.release().await;
        println!("88");
    }
}
