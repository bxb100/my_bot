use crate::box_fn;
use crate::game::{Game, IFn};
use crate::types::{MyBot, MyResult};
use crate::utils::encode_call_data;
use async_trait::async_trait;
use std::collections::HashMap;
use teloxide::payloads::{SendDiceSetters, SendMessageSetters};
use teloxide::prelude::{ChatId, Message, Requester};
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{DiceEmoji, InlineKeyboardButton, InlineKeyboardMarkup};

pub struct Basketball;

#[async_trait]
impl Game for Basketball {
    fn name(&self) -> &str {
        "投篮"
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
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("投不中", encode_call_data("投不中", serial_id)),
                InlineKeyboardButton::callback("卡篮筐(x2)", encode_call_data("卡篮筐", serial_id)),
                InlineKeyboardButton::callback("投中", encode_call_data("投中", serial_id)),
            ]]))
            .await?;

        bot.inner().pin(&msg).await?;

        Ok(msg)
    }

    async fn execute(&self, bot: &MyBot, chat_id: ChatId) -> MyResult<HashMap<&str, IFn>> {
        let basketball = bot.send_dice(chat_id).emoji(DiceEmoji::Basketball).await?;

        let value = basketball.dice().unwrap().value;

        let mut map: HashMap<&str, IFn> = HashMap::new();
        // todo: check this
        map.insert("投不中", box_fn!(value < 3, 1));
        map.insert("卡篮筐", box_fn!(value == 3, 2));
        map.insert("投中", box_fn!(value > 3, 1));

        Ok(map)
    }
}
