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

pub struct SlotMachine;

#[async_trait]
impl Game for SlotMachine {
    fn name(&self) -> &str {
        "水果机"
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
                    InlineKeyboardButton::callback("3个bar(x60)", encode_call_data("1", serial_id)),
                    InlineKeyboardButton::callback("3个🍋(x60)", encode_call_data("2", serial_id)),
                ],
                vec![
                    InlineKeyboardButton::callback("3个🍇(x60)", encode_call_data("3", serial_id)),
                    InlineKeyboardButton::callback("3个7(x60)", encode_call_data("4", serial_id)),
                ],
            ]))
            .await?;

        bot.inner().pin(&msg).await?;

        Ok(msg)
    }

    async fn execute(&self, bot: &MyBot, chat_id: ChatId) -> MyResult<HashMap<&str, IFn>> {
        let slot_machine = bot.send_dice(chat_id).emoji(DiceEmoji::SlotMachine).await?;

        let value = slot_machine.dice().unwrap().value;

        let mut map: HashMap<&str, IFn> = HashMap::new();

        // https://github.com/MasterGroosha/telegram-casino-bot/blob/aiogram3/bot/dice_check.py#L10
        map.insert("1", box_fn!(value == 1, 60));
        map.insert("2", box_fn!(value == 22, 60));
        map.insert("3", box_fn!(value == 43, 60));
        map.insert("4", box_fn!(value == 64, 60));

        Ok(map)
    }
}
