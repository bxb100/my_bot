use crate::dice::GambleControl;
use chrono::{Local, TimeDelta};
use log::info;
use std::ops::Add;
use std::time::Duration;
use teloxide::payloads::{EditMessageTextSetters, SendMessageSetters};
use teloxide::prelude::{ChatId, Message, Requester};
use teloxide::requests::HasPayload;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::MessageEntityKind::{PhoneNumber, Url};
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, LinkPreviewOptions, MessageEntity, ParseMode,
};
use teloxide::Bot;

#[derive(Debug, Copy, Clone)]
pub struct GambleDice;

impl GambleControl for GambleDice {
    async fn compute(&self, bot: Bot, chat_id: ChatId) -> Result<(), teloxide::RequestError> {
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

        let msg = bot
            .send_message(chat_id, text.clone())
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("单", "001"),
                InlineKeyboardButton::callback("双", "002"),
            ]]))
            .parse_mode(ParseMode::Html)
            .await?;

        bot.pin(&msg).await?;

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            bot.unpin(&msg).await.expect("Failed unpin");
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                text.replace("javascript:;", msg.url().unwrap().as_ref())
                    .add("(已停止下注)"),
            )
            .parse_mode(ParseMode::Html)
            .await
            .expect("Failed to edit message");

            tokio::time::sleep(Duration::from_secs(2)).await;
            draw(bot, msg, id).await.expect("Lottery draw failed");
        });
        Ok(())
    }
}

async fn draw(bot: Bot, msg: Message, id: String) -> anyhow::Result<()> {
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

    if let Some(num) = dice_1.dice() {
        info!("{}", num.value);
    }
    if let Some(num) = dice_2.dice() {
        info!("{}", num.value);
    }

    Ok(())
}
