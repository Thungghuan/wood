mod bot;
mod utils;

use bot::Bot;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = utils::load_bot_config("config/config.yml").await?;

    println!("{:#?}", config);
    let bot = Bot::new(config);

    bot.start();

    Ok(())
}
