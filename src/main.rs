use wood::Bot;

#[tokio::main]
async fn main() {
    let (config, session, base_url) = wood::init("config/config.yml").await;
    let bot = Bot::new(config, &session, &base_url);

    bot.start().await;
}
