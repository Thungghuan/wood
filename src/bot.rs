use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

use crate::api::Api;
use crate::message::MessageChain;
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

        match self.api.release().await {
            Ok(_) => println!("88"),
            Err(e) => {
                eprintln!(
                    "[Error] Releasing bot session.\n{}",
                    e
                );
            }
        }
    }

    pub async fn start(&self) {
        async fn basic_start_callback(_bot: &Bot) -> Result<()> {
            Ok(())
        }
        self.start_with_callback(basic_start_callback).await;
    }

    pub async fn send_friend_message(
        &self,
        target: &String,
        message_chain: MessageChain,
    ) -> Result<()> {
        self.api.send_friend_message(&target, message_chain).await?;
        Ok(())
    }
}
