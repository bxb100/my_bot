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
use crate::jobs::default_job_schedules;
use crate::types::{Context, MyBot};
use crate::utils::get_chat_kind;
use log::{error, info};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use tokio::{task, time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    info!("BOT Starting...");

    let bot = Bot::from_env().parse_mode(ParseMode::Html);
    setup_me(bot.clone()).await?;

    let database = Database::new().await;
    let ctx = Arc::new(Context {
        bot: bot.clone(),
        database: database.clone(),
    });

    spawn_job_scheduler();
    spawn_job_runner(ctx.clone());

    let handler = dptree::entry()
        .branch(Update::filter_message().branch(route::group_message::route()))
        .branch(Update::filter_callback_query().endpoint(handlers::callback_query::handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![database])
        .default_handler(default_log_handler)
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

fn spawn_job_scheduler() {
    task::spawn(async move {
        loop {
            let res = task::spawn(async move {
                let database = Database::new().await;
                let mut interval = time::interval(time::Duration::from_secs(180));

                loop {
                    interval.tick().await;

                    db::schedule_jobs(&database, default_job_schedules())
                        .await
                        .unwrap();
                }
            });

            match res.await {
                Err(err) if err.is_panic() => {
                    error!("schedule_jobs task died (error={err})");
                    tokio::time::sleep(std::time::Duration::new(5, 0)).await;
                }
                _ => unreachable!(),
            }
        }
    });
}

fn spawn_job_runner(ctx: Arc<Context>) {
    task::spawn(async move {
        loop {
            let ctx = ctx.clone();
            let res = task::spawn(async move {
                let mut interval = time::interval(time::Duration::from_secs(30));

                loop {
                    interval.tick().await;

                    db::run_scheduled_jobs(&ctx, &ctx.database).await.unwrap();
                }
            });

            match res.await {
                Err(err) if err.is_panic() => {
                    error!("spawn_job_runner task died (error={err})");
                    tokio::time::sleep(std::time::Duration::new(5, 0)).await;
                }
                _ => unreachable!(),
            }
        }
    });
}

async fn default_log_handler(upd: Arc<Update>) {
    let update_id = upd.id.0;
    if let Some(user) = upd.from() {
        let user_id = user.id;
        if let Some(chat) = upd.chat() {
            let chat_id = chat.id;
            let chat_kind = get_chat_kind(&chat.kind);
            log::info!(
                "Unhandled update [{update_id}]: user: [{user_id}] chat: [{chat_kind}:{chat_id}]"
            );
        } else {
            log::info!("Unhandled update [{update_id}]: user: [{user_id}] ");
        };
    } else if let Some(chat) = upd.chat() {
        let chat_id = chat.id;
        let chat_kind = get_chat_kind(&chat.kind);
        log::info!("Unhandled update [{update_id}]: chat: [{chat_kind}:{chat_id}]");
    } else {
        log::info!("Unhandled update [{update_id}]: kind: {:?}", upd.kind);
    }
}
