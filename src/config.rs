use crate::types::MyBot;
use std::env;
use std::str::FromStr;
use std::sync::{LazyLock, OnceLock};
use teloxide::types::Me;
use url::Url;

#[allow(dead_code)]
pub struct Config {
    pub me: &'static Me,
    pub bot: &'static MyBot,

    pub database_url: String,

    pub webhook_url: Url,
    pub webhook_port: u16,

    pub maintainer_id: u64,
    pub chat_id: i64,
}

fn _from_env<T: FromStr>(name: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    env::var(name)
        .unwrap_or_else(|_| panic!("{} is not defined!", name))
        .parse::<T>()
        .unwrap_or_else(|_| panic!("{} is not valid!", name))
}

pub static BOT_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    me: BOT_ME.get().expect("BOT_ME not set"),
    bot: BOT_STATIC.get().expect("BOT_STATIC not set"),
    database_url: _from_env("DATABASE_URL"),
    webhook_url: _from_env("WEBHOOK_URL"),
    webhook_port: _from_env("WEBHOOK_PORT"),
    maintainer_id: _from_env("MAINTAINER_ID"),
    chat_id: _from_env("CHAT_ID"),
});

pub static BOT_ME: OnceLock<Me> = OnceLock::new();
pub static BOT_STATIC: OnceLock<MyBot> = OnceLock::new();
