use crate::jobs::Job;
use crate::types::{Context, MyResult};
use crate::utils::{deserialize_metadata, telegram_message_url};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use teloxide::payloads::UnpinChatMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, MessageId};

pub struct StopBettingJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopBettingMetadata {
    pub chat_id: i64,
    pub message_id: i32,
    pub serial_id: String,
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

impl Job for StopBettingJob {
    fn name(&self) -> &'static str {
        "stop_betting"
    }

    async fn run(&self, ctx: &Context, metadata: &Value) -> MyResult<()> {
        info!("stop_betting handled, metadata: {}", metadata);

        let bot = &ctx.bot;
        let metadata: StopBettingMetadata = deserialize_metadata(metadata)?;
        let chat_id = ChatId(metadata.chat_id);
        let message_id = MessageId(metadata.message_id);

        bot.unpin_chat_message(chat_id)
            .message_id(message_id)
            .await?;

        let url = telegram_message_url(
            metadata.chat_id,
            metadata.chat_username,
            metadata.message_id,
        );

        let text = metadata
            .text
            .replace("javascript:;", url.as_str())
            .add("(已停止下注)");

        bot.edit_message_text(chat_id, message_id, text).await?;

        todo!("add settle job")
    }
}
