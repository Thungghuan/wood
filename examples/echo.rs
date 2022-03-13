use wood::message::{ChatroomType, MessageChain, SingleMessage};
use wood::Bot;

#[tokio::main]
async fn main() {
    let (config, session, base_url) = wood::init("config/config.yml").await;
    let mut bot = Bot::new(config, &session, &base_url);

    bot.on("message", &|ctx| async move {
        ctx.quote_reply(ctx.message_chain()).await?;

        Ok(())
    });

    // Start your bot with a callback.
    bot.start_with_callback(|bot| async {
        println!("echo qq start successfully!");

        // Send a start message to the master.
        let start_message = "Hello master, I'm echo bot!";
        let mut message_chain: MessageChain = vec![];
        message_chain.push(SingleMessage::Plain {
            text: start_message.to_string(),
        });
        bot.send_message(ChatroomType::Friend, bot.master_qq(), message_chain, None)
            .await?;

        Ok(())
    })
    .await;
}
