use std::path::PathBuf;

use teloxide::{prelude::*, types::InputFile};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();
    let chat: ChatId = teloxide::prelude::ChatId(-1001677537934);
    let path = PathBuf::from(r"C:\Users\Integro\Desktop\file.mp3");

    let file = InputFile::file(path);

    let _ = bot.send_message(chat, "text11").await;
    let _ = bot.send_audio(chat, file).await;

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;
}
