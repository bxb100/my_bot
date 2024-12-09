mod api;
mod config;
mod dao;
mod db;
mod game;
mod handlers;
mod jobs;
mod route;
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
    let database = init_db().await;

    let handler = dptree::entry()
        .branch(Update::filter_message().branch(route::group_message::route()))
        .branch(Update::filter_callback_query().endpoint(handlers::callback_query::handler));

    Dispatcher::builder(bot, handler)
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        .dependencies(dptree::deps![database])
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn setup_me(bot: MyBot) -> anyhow::Result<()> {
    let me = bot.get_me().await?;
    info!("me: {:?}", me);
    BOT_ME.set(me).unwrap();
    BOT_STATIC.set(bot).unwrap();

    Ok(())
}

async fn init_db() -> Database {
    Database::new().await
}
