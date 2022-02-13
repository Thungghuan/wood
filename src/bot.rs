use std::fmt::Debug;

use crate::api::Api;
use crate::utils::load_bot_settings;

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

pub struct BotSettingsAdapterHttp {
    pub host: String,
    pub port: u32,
}
impl BotSettingsAdapterHttp {
    pub fn new(host: String, port: u32) -> Self {
        BotSettingsAdapterHttp { host, port }
    }
}

pub struct BotSettingsAdapter {
    pub http: BotSettingsAdapterHttp,
}
impl BotSettingsAdapter {
    pub fn new(http: BotSettingsAdapterHttp) -> Self {
        BotSettingsAdapter { http }
    }
}

pub struct BotSettings {
    verify_key: String,
    pub adapter_settings: BotSettingsAdapter,
}
impl BotSettings {
    pub fn new(verify_key: String, adapter_settings: BotSettingsAdapter) -> Self {
        BotSettings {
            verify_key,
            adapter_settings,
        }
    }
}


pub struct Bot {
    qq: String,
    config: BotConfig,
    api: Api,
}
impl Bot {
    pub fn new(config: BotConfig) -> Self {
        let settings = load_bot_settings(&config.setting_file).unwrap();

        Bot {
            qq: config.qq.clone(),
            api: Api::new(&config.qq, settings),
            config,
        }
    }

    pub fn start(&self) {
        println!("Bot qq is: {}", self.qq);
        println!("Bot config is {:#?}", self.config);
        println!("Bot api is {:#?}", self.api);
    }
}
