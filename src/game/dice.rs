use crate::api::telegram::send_message;
use crate::config::BOT_CONFIG;
use crate::db::Database;
use crate::domain::wager::{Wager, WagerDao};
use crate::types::MyBot;
use crate::utils::encode_call_data;
use chrono::{Local, TimeDelta};
use log::info;
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;
use teloxide::payloads::{EditMessageTextSetters, SendMessageSetters};
use teloxide::prelude::{ChatId, Message, Requester};
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

#[derive(Debug, Clone)]
pub struct Dice {
    pub bot: MyBot,
    pub chat_id: ChatId,
    pub database: Database,
}

impl Dice {
    pub fn new(database: Database) -> Self {
        Dice {
            bot: BOT_CONFIG.bot.clone(),
            chat_id: ChatId(BOT_CONFIG.chat_id),
            database,
        }
    }

    pub async fn play(self) -> Result<(), teloxide::RequestError> {
        let now = Local::now();

        let id = now.format("%y%m%d%H%M").to_string();

        let text = indoc::formatdoc! {
            r#"
        系统发起了一轮骰子猜大小单双(2骰子)竞猜
        期号: <a href="javascript:;">{}</a>
        倍率: {}
        开奖时间: {}
        请点击下方按钮投注
        开奖前 1 分钟停止下注"#,
            id,
            1,
            now.add(TimeDelta::minutes(3)).format("%Y-%m-%d %H:%M:%S")
        };

        let bot = self.bot;
        let chat_id = self.chat_id;

        let msg = bot
            .send_message(chat_id, text.clone())
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("小", encode_call_data("小", &id)),
                    InlineKeyboardButton::callback("小单(x2)", encode_call_data("小单", &id)),
                    InlineKeyboardButton::callback("大双(x2)", encode_call_data("大双", &id)),
                ],
                vec![
                    InlineKeyboardButton::callback("7点(x2)", encode_call_data("7点", &id)),
                    InlineKeyboardButton::callback("大", encode_call_data("大", &id)),
                    InlineKeyboardButton::callback("单", encode_call_data("单", &id)),
                ],
                vec![
                    InlineKeyboardButton::callback("双", encode_call_data("双", &id)),
                    InlineKeyboardButton::callback("大单(x2)", encode_call_data("大单", &id)),
                    InlineKeyboardButton::callback("小双(x2)", encode_call_data("小双", &id)),
                ],
            ]))
            .parse_mode(ParseMode::Html)
            .await?;

        bot.inner().pin(&msg).await?;

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(120)).await;
            bot.inner().unpin(&msg).await.expect("Failed unpin");
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                text.replace("javascript:;", msg.url().unwrap().as_ref())
                    .add("(已停止下注)"),
            )
            .parse_mode(ParseMode::Html)
            .await
            .expect("Failed to edit message");

            tokio::time::sleep(Duration::from_secs(60)).await;
            Self::draw(msg, id, self.database)
                .await
                .expect("Lottery draw failed");
        });
        Ok(())
    }

    pub async fn draw(msg: Message, id: String, database: Database) -> anyhow::Result<()> {
        let bot = BOT_CONFIG.bot.inner();
        bot.send_message(
            msg.chat.id,
            format!(
                r#"<a href="{}">{}</a> 期准备开奖了"#,
                msg.url().unwrap(),
                id
            ),
        )
        .parse_mode(ParseMode::Html)
        .reply_to(msg.id)
        .await?;

        let dice_1 = bot.send_dice(msg.chat.id).await?;
        let dice_2 = bot.send_dice(msg.chat.id).await?;

        let num1 = dice_1.dice().unwrap().value;
        let num2 = dice_2.dice().unwrap().value;

        let dao = WagerDao { database };
        let wagers = dao.get_by_time_id(id).await?;
        let mut map: HashMap<&Wager, i64> = HashMap::new();

        for w in &wagers {
            let amount = if let Some(amount) = w.amount {
                amount
            } else {
                continue;
            };
            match w.action.as_str() {
                "小" if num1 + num2 <= 6 => map.insert(w, amount * 2),
                "小单" if num1 + num2 <= 6 && num1 % 2 != 0 && num2 % 2 != 0 => {
                    map.insert(w, amount * 4)
                }
                "大双" if num1 + num2 > 6 && num1 % 2 == 0 && num2 % 2 == 0 => {
                    map.insert(w, amount * 4)
                }
                "7点" if num1 + num2 == 7 => map.insert(w, amount * 4),
                "大" if num1 + num2 > 6 => map.insert(w, amount * 2),
                "单" if num1 % 2 != 0 && num2 % 2 != 0 => map.insert(w, amount * 2),
                "双" if num1 % 2 == 0 && num2 % 2 == 0 => map.insert(w, amount * 2),
                "大单" if num1 + num2 > 6 && num1 % 2 != 0 && num2 % 2 != 0 => {
                    map.insert(w, amount * 4)
                }
                "小双" if num1 + num2 <= 6 && num1 % 2 == 0 && num2 % 2 == 0 => {
                    map.insert(w, amount * 4)
                }
                _ => map.insert(w, -amount),
            };
        }

        info!("map: {:?}", map);

        let mut summary = String::new();
        let mut lost_sum = 0;
        map.iter().for_each(|(wager, amount)| {
            lost_sum += amount;
            if *amount > 0 {
                summary += &format!(
                    "恭喜<a href=\"tg://user?id={}\">{}</a>赢了 {} 积分\n",
                    wager.user_id,
                    wager.user_name.as_ref().unwrap_or(&"anonymous".to_string()),
                    amount
                );
            } else {
                summary += &format!(
                    "<a href=\"tg://user?id={}\">{}</a>失去了 {} 积分\n",
                    wager.user_id,
                    wager.user_name.as_ref().unwrap_or(&"anonymous".to_string()),
                    -amount
                )
            }
        });
        summary += &format!(
            "庄家{}了 {} 积分",
            if lost_sum < 0 { "赢" } else { "输" },
            lost_sum.abs()
        );
        send_message(msg.chat.id, &summary).await?;
        // todo save to database

        Ok(())
    }
}
