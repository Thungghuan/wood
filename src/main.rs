use wood::message::{ChatroomType, MessageChain, SingleMessage};
use wood::{Bot, Result};

#[tokio::main]
async fn main() {
    let (config, session, base_url) = wood::init("config/config.yml").await;
    let bot = Bot::new(config, &session, &base_url);

    bot.on("message");

    // This will see a error message.
    bot.on("msg");

    bot.start_with_callback(bot_init).await;

    // You can also start the bot directly.
    // bot.start().await;
}

async fn bot_init(bot: &Bot) -> Result<()> {
    println!("Bot qq is: {}", bot.qq());
    println!("Master qq is: {}", bot.master_qq());
    println!("Session key: {}", bot.session());

    // Send a start message to the master.
    let start_message = "Hello master, your bot start successfully!";
    let mut message_chain: MessageChain = vec![];
    message_chain.push(SingleMessage::Plain {
        text: start_message.to_string(),
    });

    bot.send_message(ChatroomType::Friend, &bot.master_qq(), message_chain)
        .await?;

    Ok(())
}
