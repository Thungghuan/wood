mod api;
mod bot;
mod utils;

use bot::Bot;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = utils::load_bot_config("config/config.yml").unwrap();
    let settings = utils::load_bot_settings(&config.setting_file);

    let base_url = String::from("http://") + &settings.host + ":" + &settings.port;
    let session = utils::get_session(&base_url, &settings.verify_key).await;

    let bot = Bot::new(config, &session, &base_url);
    
    bot.start().await;

    Ok(())
}
