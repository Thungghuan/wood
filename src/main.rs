mod bot;

use bot::Bot;

fn main() {
    println!("Hello, BOT!");

    let bot = Bot::new(String::from("1160000000"));
    bot.start();
}
