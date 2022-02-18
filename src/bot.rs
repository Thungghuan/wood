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
    master_qq: String,
    base_url: String,
    session: String,
    api: Api,
}
impl Bot {
    pub fn new(config: BotConfig, session: &str, base_url: &str) -> Self {
        Bot {
            qq: config.qq.clone(),
            master_qq: config.master_qq.clone(),
            session: session.to_string(),
            base_url: base_url.to_string(),
            api: Api::new(&config.qq, base_url, session),
        }
    }

    pub async fn start(&self) {
        println!("Bot qq is: {}", self.qq);
        println!("Session key: {:#?}", self.session);
    }
}
