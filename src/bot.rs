use std::fmt::Debug;

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

    pub async fn start(&self) {
        self.api.link().await;

        println!("Bot qq is: {}", self.qq);
        println!("Master qq is: {}", self.master_qq);
        println!("Session key: {:#?}", self.session);
    }
}
