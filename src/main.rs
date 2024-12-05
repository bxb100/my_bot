mod config;
mod db;
mod domain;
mod game;
mod handlers;
mod types;
mod utils;

use crate::config::{BOT_CONFIG, BOT_ME, BOT_STATIC};
use crate::db::Database;
use crate::game::dice::Dice;
use crate::handlers::handler_group_filter_message;
use crate::types::{MyBot, MyResult};
use chrono::{Duration, Utc};
use chrono_tz::Asia::Shanghai;
use chrono_tz::Tz::Asia__Shanghai;
use cron::Schedule;
use log::info;
use std::ops::Sub;
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    info!("Starting throw dice bot...");

    let bot = Bot::from_env().parse_mode(ParseMode::Html);
    setup_me(bot.clone()).await?;
    let database = init_db().await;

    let schedule = Schedule::from_str("0 0/10 08-23 * * ?")?;
    tokio::spawn(async move {
        cron_dispatch(schedule).await.expect("Failed to run");
    });
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter(|msg: Message| msg.chat.id.0 == BOT_CONFIG.chat_id)
                .endpoint(handler_group_filter_message::handler),
        )
        .branch(
            Update::filter_callback_query().endpoint(handlers::handler_callback_query::handler),
        );

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

async fn cron_dispatch(schedule: Schedule) -> MyResult<()> {
    let database = Database::new().await;
    loop {
        let time = schedule.upcoming(Shanghai).take(1).next().unwrap();
        // BOT_CONFIG
        //     .bot
        //     .send_message(ChatId(chat_id), format!("Next game in {}", time))
        //     .await?;

        // todo
        Dice::new(database.clone()).play().await?;

        let now = Utc::now().with_timezone(&Asia__Shanghai);
        let diff = time.sub(now);
        tokio::time::sleep(Duration::from(diff).to_std().unwrap()).await;
    }
}
