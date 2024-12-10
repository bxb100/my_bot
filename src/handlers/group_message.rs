use crate::config::BOT_CONFIG;
use crate::dao::{gambles, users};
use crate::db::Database;
use crate::types::{MyBot, MyResult};
use log::{info, warn};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::ParseMode;
use teloxide::Bot;

pub async fn handler(bot: MyBot, msg: Message, text: String, database: Database) -> MyResult<()> {
    info!("group_message handler: {text}");
    if let Some(ref user) = msg.from {
        // fix
        let user_id = if user.is_bot {
            BOT_CONFIG.maintainer_id
        } else {
            user.id.0
        } as i64;

        if text == "梭哈" || text.parse::<i64>().is_ok() {
            if let Ok(gambles) =
                gambles::get_by_user_id_and_empty_amount(database.pool, user_id).await
            {
                let user = users::get_by_id(database.pool, user_id).await?.unwrap();

                let amount = if text == "梭哈" {
                    user.points
                } else {
                    text.parse::<i64>().unwrap()
                };

                if user.points < amount {
                    bot.inner()
                        .send_message(msg.chat.id, "积分不足")
                        .reply_to(msg.id)
                        .await?;
                    return Ok(());
                }

                users::update_amount(database.pool, user_id, user.points - amount).await?;

                gambles::update_amount(database.pool, gambles.id, amount).await?;

                send_success_msg(
                    bot.inner(),
                    msg,
                    &gambles.serial_id,
                    amount,
                    &gambles.action,
                )
                .await?;

                return Ok(());
            }
        }
    }

    warn!("unknown message: {:?}", msg);

    Ok(())
}

async fn send_success_msg(
    bot: &Bot,
    msg: Message,
    serial_id: &str,
    amount: i64,
    action: &str,
) -> MyResult<()> {
    let user = msg.from.unwrap();

    bot.send_message(
        msg.chat.id,
        indoc::formatdoc! {"<a href=\"{}\">{}</a> 投注了 {} 期
            投注积分: {}
            投注类型: {}",
            user.url(),
            user.username.as_ref().unwrap_or(&"".to_string()),
            serial_id,
            amount,
            action,
        },
    )
    .reply_to(msg.id)
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}
