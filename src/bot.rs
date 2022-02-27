use std::{fmt::Debug, time::Duration};

use tokio::{io, time::sleep};

use crate::api::Api;

#[derive(Debug)]
pub struct BotConfig {
    qq: String,
    master_qq: String,
    pub setting_file: String,
}
impl crate::bot::BotConfig {
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

    pub async fn start(&self) -> io::Result<()> {
        self.api.link().await;

        println!("Bot qq is: {}", self.qq);
        println!("Master qq is: {}", self.master_qq);
        println!("Session key: {:#?}", self.session);

        use crate::message::{MessageChain, SingleMessage};

        let mut message_chain: MessageChain = vec![];
        message_chain.push(SingleMessage::Plain {
            text: "Hello master, your bot start successfully!".to_string(),
        });

        // Send a start message to the master.
        // TODO: If error occurred, the bot will not start.
        match self
            .api
            .send_friend_message(&self.master_qq, message_chain)
            .await
        {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
            }
        }

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
        self.api.release().await;
        println!("88");

        Ok(())
    }
}
