use std::fmt::Debug;

#[derive(Debug)]
pub struct BotConfig {
    qq: String,
    master_qq: String,
    setting_file: String,
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
    config: BotConfig,
}

impl Bot {
    pub fn new(config: BotConfig) -> Self {
        Bot {
            qq: config.qq.clone(),
            config,
        }
    }

    pub fn start(&self) {
        println!("Bot qq is: {}", self.qq)
    }
}
