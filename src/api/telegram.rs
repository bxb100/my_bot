use crate::config::BOT_CONFIG;
use crate::types::MyResult;
use backon::{ExponentialBuilder, Retryable};
use teloxide::requests::Requester;
use teloxide::types::{ChatId, Message};

// todo: rewrite all bot call with retry
pub async fn send_message(chat_id: ChatId, message: &str) -> MyResult<Message> {
    let message = (|| async { BOT_CONFIG.bot.send_message(chat_id, message).await })
        .retry(ExponentialBuilder::default())
        .await?;

    Ok(message)
}
