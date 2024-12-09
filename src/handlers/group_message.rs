use crate::config::BOT_CONFIG;
use crate::db::Database;
use crate::domain::users::{UpsertUsers, UserDao};
use crate::domain::wager::{UpsertWager, WagerDao};
use crate::types::{MyBot, MyResult};
use log::warn;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::MediaKind::Text;
use teloxide::types::{MediaText, MessageCommon, MessageKind, ParseMode};
use teloxide::Bot;

pub async fn handler(bot: MyBot, msg: Message, text: String, database: Database) -> MyResult<()> {
    println!("{text}");
    // check wager exist
    if let Some(ref user) = msg.from {
        if let MessageKind::Common(MessageCommon {
            media_kind: Text(MediaText { ref text, .. }),
            ..
        }) = msg.kind
        {
            let wager_dao = WagerDao {
                database: database.clone(),
            };
            let user_dao = UserDao {
                database: database.clone(),
            };

            // fix
            let user_id = if user.is_bot {
                BOT_CONFIG.maintainer_id
            } else {
                user.id.0
            } as i64;

            if text == "梭哈" {
                if let Ok(wager) = wager_dao.get_by_user_id_and_empty_amount(user_id).await {
                    let user = user_dao.get_by_id(user_id).await?.unwrap();
                    user_dao
                        .upsert(UpsertUsers {
                            id: user_id,
                            name: user.name,
                            points: 0,
                            daily_reward: user.daily_reward,
                        })
                        .await?;

                    wager_dao
                        .update(
                            UpsertWager {
                                time_id: wager.time_id.clone(),
                                user_id: wager.user_id,
                                user_name: wager.user_name,
                                action: wager.action.clone(),
                                amount: Some(user.points),
                            },
                            wager.id,
                        )
                        .await?;

                    send_success_msg(bot.inner(), msg, &wager.time_id, user.points, &wager.action)
                        .await?;
                }
                return Ok(());
            } else if let Ok(num) = text.parse::<u64>() {
                if let Ok(wager) = wager_dao.get_by_user_id_and_empty_amount(user_id).await {
                    let user = user_dao.get_by_id(user_id).await?.unwrap();

                    if user.points < num as i64 {
                        bot.inner()
                            .send_message(msg.chat.id, "积分不足")
                            .reply_to(msg.id)
                            .await?;
                        return Ok(());
                    }

                    user_dao
                        .upsert(UpsertUsers {
                            id: user_id,
                            name: user.name,
                            points: user.points - num as i64,
                            daily_reward: user.daily_reward,
                        })
                        .await?;

                    wager_dao
                        .update(
                            UpsertWager {
                                time_id: wager.time_id.clone(),
                                user_id: wager.user_id,
                                user_name: wager.user_name,
                                action: wager.action.clone(),
                                amount: Some(num as i64),
                            },
                            wager.id,
                        )
                        .await?;

                    send_success_msg(bot.inner(), msg, &wager.time_id, num as i64, &wager.action)
                        .await?;
                }
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
    time_id: &str,
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
            time_id,
            amount,
            action,
        },
    )
    .reply_to(msg.id)
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}
