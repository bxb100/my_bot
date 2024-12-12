use crate::dao::jobs;
use crate::handlers::settle_bets::{SettleBetsJob, SettleBetsMetadata};
use crate::jobs::{Job, GAME_STOP_TO_END_BETTING_IN_SECS};
use crate::types::{Context, MyResult};
use crate::utils::telegram_message_url;
use async_trait::async_trait;
use chrono::Utc;
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;
use teloxide::payloads::UnpinChatMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, MessageId};

pub struct StopBettingJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopBettingMetadata {
    pub chat_id: i64,
    pub message_id: i32,
    pub serial_id: String,
    pub game_name: String,
    pub chat_username: Option<String>,
    pub text: String,
}

impl Display for StopBettingMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "chat_id: {}, message_id: {}, chat_username: {:?}",
            self.chat_id, self.message_id, self.chat_username
        ))
    }
}

#[async_trait]
impl Job for StopBettingJob {
    fn name(&self) -> &'static str {
        "stop_betting"
    }

    async fn run(&self, _id: &i64, ctx: &Context, metadata: &serde_json::Value) -> MyResult<()> {
        info!("stop_betting handled, metadata: {:?}", metadata);
        // fixme: should be set in metadata?
        let now = Utc::now();

        let bot = &ctx.bot;
        let metadata: StopBettingMetadata = serde_json::from_value(metadata.clone())?;
        let chat_id = ChatId(metadata.chat_id);
        let message_id = MessageId(metadata.message_id);

        bot.unpin_chat_message(chat_id)
            .message_id(message_id)
            .await?;

        let url = telegram_message_url(
            metadata.chat_id,
            metadata.chat_username.as_deref(),
            metadata.message_id,
        );

        let text = metadata
            .text
            .replace("javascript:;", url.as_str())
            .add("(已停止下注)");

        bot.edit_message_text(chat_id, message_id, text).await?;

        // add settle job
        let metadata = SettleBetsMetadata {
            chat_id: metadata.chat_id,
            message_id: metadata.message_id,
            chat_username: metadata.chat_username,
            serial_id: metadata.serial_id,
            game_name: metadata.game_name,
        };

        jobs::insert(
            ctx.database.pool,
            SettleBetsJob.name(),
            &now.add(Duration::from_secs(GAME_STOP_TO_END_BETTING_IN_SECS)),
            &serde_json::to_value(metadata)?,
        )
        .await?;

        Ok(())
    }
}
