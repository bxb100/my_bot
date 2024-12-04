mod config;
mod db;
mod dice;
mod domain;
mod handlers;
mod routes;
mod types;
mod utils;

use crate::config::{BOT_ME, BOT_STATIC};
use crate::db::Database;
use crate::types::MyBot;
use log::info;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    info!("Starting throw dice bot...");

    let bot = Bot::from_env().parse_mode(ParseMode::Html);
    setup_me(bot.clone()).await?;
    init_db().await;

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;

    Ok(())
}

async fn setup_me(bot: MyBot) -> anyhow::Result<()> {
    let me = bot.get_me().await?;
    BOT_ME.set(me).unwrap();
    BOT_STATIC.set(bot).unwrap();

    Ok(())
}

async fn init_db() {
    let _ = Database::new().await;
}
