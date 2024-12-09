use crate::jobs::Job;
use crate::types::{Context, MyResult};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use teloxide::payloads::UnpinChatMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, Message, MessageId};

pub struct StopBettingJob;

#[derive(Serialize, Deserialize)]
pub struct StopBettingMetadata {
    pub chat_id: i64,
    pub message_id: i32,
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
        let metadata: StopBettingMetadata = serde_json::from_value(metadata.clone())?;
        let chat_id = ChatId(metadata.chat_id);
        let message_id = MessageId(metadata.message_id);

        bot.unpin_chat_message(chat_id)
            .message_id(message_id)
            .await?;

        let url = Message::url_of(chat_id, metadata.chat_username.as_deref(), message_id);

        let text = if let Some(url) = url {
            metadata
                .text
                .replace("javascript:;", url.as_str())
                .add("(已停止下注)")
        } else {
            metadata.text
        };

        bot.edit_message_text(chat_id, message_id, text).await?;

        todo!("add settle job")
    }
}
