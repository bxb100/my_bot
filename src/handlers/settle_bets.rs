use crate::api::telegram::send_message;
use crate::dao::{gambles, users};
use crate::game::games;
use crate::jobs::Job;
use crate::types::{Context, MyError, MyResult};
use crate::utils::telegram_message_url;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::{ChatId, MessageId, ParseMode};

pub struct SettleBetsJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettleBetsMetadata {
    pub chat_id: i64,
    pub message_id: i32,
    pub chat_username: Option<String>,
    pub serial_id: String,
    pub game_name: String,
}

#[async_trait]
impl Job for SettleBetsJob {
    fn name(&self) -> &str {
        "settle_bets"
    }

    async fn run(&self, _id: &i64, ctx: &Context, metadata: &serde_json::Value) -> MyResult<()> {
        let metadata: SettleBetsMetadata = serde_json::from_value(metadata.clone())?;
        let bot = ctx.bot.inner();

        bot.send_message(
            ChatId(metadata.chat_id),
            format!(
                r#"<a href="{}">{}</a> 期准备开奖了"#,
                telegram_message_url(
                    metadata.chat_id,
                    metadata.chat_username.as_deref(),
                    metadata.message_id
                ),
                metadata.serial_id
            ),
        )
        .parse_mode(ParseMode::Html)
        .reply_to(MessageId(metadata.message_id))
        .await?;

        for game in games() {
            if game.name() == metadata.game_name {
                let execution_result = game.execute(&ctx.bot, ChatId(metadata.chat_id)).await?;
                let bet_gambles =
                    gambles::get_by_serial_id(ctx.database.pool, &metadata.serial_id).await?;

                let mut summary = String::new();
                let mut lost_sum = 0;

                for gamble in bet_gambles {
                    let handle_action = execution_result.get(&*gamble.action);
                    if handle_action.is_none() {
                        return Err(MyError::Unknown(format!(
                            "gamble {} can't solve {}",
                            metadata.game_name, gamble.action
                        )));
                    }

                    let amount = handle_action.unwrap()(gamble.amount.unwrap_or(0) as isize);
                    lost_sum += amount;

                    if amount > 0 {
                        summary += &format!(
                            "恭喜 <a href=\"tg://user?id={}\">{}</a> 赢了 {} 积分\n",
                            gamble.user_id,
                            gamble
                                .user_name
                                .as_ref()
                                .unwrap_or(&"anonymous".to_string()),
                            amount
                        );
                        // fixme: should using tx
                        users::increase_amount(ctx.database.pool, gamble.user_id, amount as i32)
                            .await?;
                    } else {
                        summary += &format!(
                            "<a href=\"tg://user?id={}\">{}</a> 失去了 {} 积分\n",
                            gamble.user_id,
                            gamble
                                .user_name
                                .as_ref()
                                .unwrap_or(&"anonymous".to_string()),
                            -amount
                        )
                    }
                }

                summary += &format!(
                    "庄家{}了 {} 积分",
                    if lost_sum < 0 { "赢" } else { "输" },
                    lost_sum.abs()
                );

                gambles::delete_by_serial_id(ctx.database.pool, &metadata.serial_id).await?;

                send_message(ChatId(metadata.chat_id), &summary).await?;

                return Ok(());
            }
        }

        Err(MyError::Unknown(format!(
            "game {} not found",
            metadata.game_name
        )))
    }
}
