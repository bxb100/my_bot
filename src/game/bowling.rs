use crate::box_fn;
use crate::game::{Game, IFn};
use crate::types::{MyBot, MyResult};
use crate::utils::encode_call_data;
use async_trait::async_trait;
use std::collections::HashMap;
use teloxide::payloads::{SendDiceSetters, SendMessageSetters};
use teloxide::prelude::{ChatId, Message};
use teloxide::requests::Requester;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{DiceEmoji, InlineKeyboardButton, InlineKeyboardMarkup};

pub struct Bowling;

#[async_trait]
impl Game for Bowling {
    fn name(&self) -> &str {
        "保龄球击倒数"
    }

    async fn start_play(
        &self,
        bot: &MyBot,
        chat_id: ChatId,
        serial_id: &str,
        msg: &str,
    ) -> MyResult<Message> {
        let msg = bot
            .send_message(chat_id, msg)
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback(
                        "全倒(x5))",
                        encode_call_data("全倒", serial_id),
                    ),
                    InlineKeyboardButton::callback(
                        "擦边(x5))",
                        encode_call_data("擦边", serial_id),
                    ),
                    InlineKeyboardButton::callback("1个(x5))", encode_call_data("1个", serial_id)),
                ],
                vec![
                    InlineKeyboardButton::callback("3个(x5))", encode_call_data("3个", serial_id)),
                    InlineKeyboardButton::callback("4个(x5))", encode_call_data("4个", serial_id)),
                    InlineKeyboardButton::callback("5个(x5))", encode_call_data("5个", serial_id)),
                ],
            ]))
            .await?;

        bot.inner().pin(&msg).await?;

        Ok(msg)
    }

    async fn execute(&self, bot: &MyBot, chat_id: ChatId) -> MyResult<HashMap<&str, IFn>> {
        let bowling = bot.send_dice(chat_id).emoji(DiceEmoji::Bowling).await?;

        let value = bowling.dice().unwrap().value;

        let mut map: HashMap<&str, IFn> = HashMap::new();

        map.insert("全倒", box_fn!(value == 6, 5));
        map.insert("擦边", box_fn!(value == 0, 5));
        map.insert("1个", box_fn!(value == 1, 5));
        map.insert("3个", box_fn!(value == 3, 5));
        map.insert("4个", box_fn!(value == 4, 5));
        map.insert("5个", box_fn!(value == 5, 5));

        Ok(map)
    }
}
