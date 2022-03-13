use wood::message::{create_plain_message_chain, ChatroomType};
use wood::Bot;

#[tokio::main]
async fn main() {
    let (config, session, base_url) = wood::init("config/config.yml").await;
    let mut bot = Bot::new(config, &session, &base_url);

    bot.command("echo", &|ctx| async move {
        ctx.reply(ctx.message_chain()).await?;

        Ok(())
    });

    // Start your bot with a callback.
    bot.start_with_callback(|bot| async {
        println!("Echo Bot start successfully!");

        // Send a start message to the master.
        let start_message = "Hello master, I'm an echo bot!";

        bot.send_message(
            ChatroomType::Friend,
            bot.master_qq(),
            create_plain_message_chain(start_message.to_string()),
            None,
        )
        .await?;

        Ok(())
    })
    .await;
}
