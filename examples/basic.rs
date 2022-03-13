use wood::message::{ChatroomType, MessageChain, SingleMessage};
use wood::Bot;

#[tokio::main]
async fn main() {
    let (config, session, base_url) = wood::init("config/config.yml").await;
    let mut bot = Bot::new(config, &session, &base_url);

    bot.on("message", &|ctx| async move {
        match ctx.chatroom_type() {
            ChatroomType::Friend => println!(
                "Received friend message from {}({})",
                ctx.sender_nickname(),
                ctx.sender_id()
            ),

            ChatroomType::Group => println!(
                "Received group message from {}[{}] in group: {}[{}]",
                ctx.sender_nickname(),
                ctx.sender_id(),
                ctx.chatroom_name(),
                ctx.chatroom_id()
            ),
        }

        Ok(())
    });

    bot.on("message", &|ctx| async move {
        println!("{:#?}", ctx.message_chain());

        Ok(())
    });

    // Only echo message when received friend message
    bot.on("friendMessage", &|ctx| async move {
        ctx.reply(ctx.message_chain()).await?;

        Ok(())
    });

    // No need to use `move` when regardless of context
    bot.on("groupMessage", &|_| async { Ok(()) });

    // You'll see a error message that tells that
    // you are listening a `InvalidEvent`.
    bot.on("msg", &|_| async { Ok(()) });

    // Start your bot with a callback.
    bot.start_with_callback(|bot| async {
        println!("Bot qq is: {}", bot.qq());
        println!("Master qq is: {}", bot.master_qq());
        println!("Session key: {}", bot.session());

        // Send a start message to the master.
        let start_message = "Hello master, your bot start successfully!";
        let mut message_chain: MessageChain = vec![];
        message_chain.push(SingleMessage::Plain {
            text: start_message.to_string(),
        });

        bot.send_message(ChatroomType::Friend, bot.master_qq(), message_chain)
            .await?;

        Ok(())
    })
    .await;

    // You can also start the bot directly.
    // bot.start().await;
}
