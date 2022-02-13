pub mod http;

use crate::bot::BotSettings;

#[derive(Debug)]
pub struct Api {
    qq: String,
    host: String,
    port: u32,
}

impl Api {
    pub fn new(qq: &str, bot_settings: BotSettings) -> Self {
        let http_adapter = bot_settings.adapter_settings.http;
        let host = &http_adapter.host;
        let port = &http_adapter.port;

        Api {
            qq: qq.to_string(),
            host: host.to_string(),
            port: u32::try_from(*port).unwrap(),
        }
    }
}
