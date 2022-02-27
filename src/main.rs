use tokio::io;
use wood::bot::Bot;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = wood::load_bot_config("config/config.yml").unwrap();
    let settings = wood::load_bot_settings(&config.setting_file);

    let base_url = String::from("http://") + &settings.host + ":" + &settings.port;
    let session = wood::get_session(&base_url, &settings.verify_key).await;

    let bot = Bot::new(config, &session, &base_url);

    bot.start().await?;

    Ok(())
}
