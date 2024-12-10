use crate::box_fn;
use crate::game::{Game, IFn};
use crate::types::{MyBot, MyResult};
use crate::utils::encode_call_data;
use async_trait::async_trait;
use std::collections::HashMap;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

pub struct DoubleDice;

// todo: options to enum with display and into<&str>

#[async_trait]
impl Game for DoubleDice {
    fn name(&self) -> &str {
        "骰子猜大小单双(2骰子)"
    }

    async fn start_play(
        &self,
        bot: &MyBot,
        chat_id: ChatId,
        serial_id: String,
        msg: String,
    ) -> MyResult<Message> {
        let msg = bot
            .send_message(chat_id, msg)
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("小", encode_call_data("小", &serial_id)),
                    InlineKeyboardButton::callback(
                        "小单(x2)",
                        encode_call_data("小单", &serial_id),
                    ),
                    InlineKeyboardButton::callback(
                        "大双(x2)",
                        encode_call_data("大双", &serial_id),
                    ),
                ],
                vec![
                    InlineKeyboardButton::callback("7点(x2)", encode_call_data("7点", &serial_id)),
                    InlineKeyboardButton::callback("大", encode_call_data("大", &serial_id)),
                    InlineKeyboardButton::callback("单", encode_call_data("单", &serial_id)),
                ],
                vec![
                    InlineKeyboardButton::callback("双", encode_call_data("双", &serial_id)),
                    InlineKeyboardButton::callback(
                        "大单(x2)",
                        encode_call_data("大单", &serial_id),
                    ),
                    InlineKeyboardButton::callback(
                        "小双(x2)",
                        encode_call_data("小双", &serial_id),
                    ),
                ],
            ]))
            .await?;

        bot.inner().pin(&msg).await?;

        Ok(msg)
    }

    async fn execute(&self, bot: &MyBot, chat_id: ChatId) -> MyResult<HashMap<&str, IFn>> {
        let dice_1 = bot.send_dice(chat_id).await?;
        let dice_2 = bot.send_dice(chat_id).await?;

        let num1 = dice_1.dice().unwrap().value;
        let num2 = dice_2.dice().unwrap().value;

        let mut map: HashMap<&str, IFn> = HashMap::new();

        map.insert("小", box_fn!(num1 + num2 == 7, 2));
        map.insert(
            "小单",
            box_fn!(num1 + num2 <= 6 && num1 % 2 != 0 && num2 % 2 != 0, 4),
        );
        map.insert(
            "大双",
            box_fn!(num1 + num2 > 6 && num1 % 2 == 0 && num2 % 2 == 0, 4),
        );
        map.insert("7点", box_fn!(num1 + num2 == 7, 4));
        map.insert("大", box_fn!(num1 + num2 > 6, 2));
        map.insert("单", box_fn!(num1 % 2 != 0 && num2 % 2 != 0, 2));
        map.insert("双", box_fn!(num1 % 2 == 0 && num2 % 2 == 0, 2));
        map.insert(
            "大单",
            box_fn!(num1 + num2 > 6 && num1 % 2 != 0 && num2 % 2 != 0, 4),
        );
        map.insert(
            "小双",
            box_fn!(num1 + num2 <= 6 && num1 % 2 == 0 && num2 % 2 == 0, 4),
        );

        Ok(map)
    }
}
