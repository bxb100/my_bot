use crate::game::double_dice::DoubleDice;
use crate::types::{MyBot, MyResult};
use async_trait::async_trait;
use chrono::DateTime;
use chrono_tz::Tz;
use std::collections::HashMap;
use teloxide::prelude::ChatId;
use teloxide::types::Message;

mod double_dice;

pub type IFn = Box<dyn Fn(isize) -> isize>;

pub fn games() -> Vec<Box<dyn Game + Send + Sync>> {
    vec![Box::new(DoubleDice)]
}

#[async_trait]
pub trait Game {
    fn name(&self) -> &str;

    fn message(&self, now: DateTime<Tz>, settle: DateTime<Tz>, time: u32) -> String {
        let id = now.format("%y%m%d%H%M").to_string();

        let text = indoc::formatdoc! {
            r#"
                系统发起了一轮{}竞猜
                期号: <a href="javascript:;">{}</a>
                倍率: {}
                开奖时间: {}
                请点击下方按钮投注
                开奖前 1 分钟停止下注"#,
            self.name(),
            id,
            time,
            settle.format("%Y-%m-%d %H:%M:%S")
        };

        text
    }

    async fn play(
        &self,
        bot: &MyBot,
        chat_id: ChatId,
        serial_id: String,
        msg: String,
    ) -> MyResult<Message>;

    async fn execute(&self, bot: &MyBot, chat_id: ChatId) -> MyResult<HashMap<&str, IFn>>;
}

#[macro_export]
macro_rules! box_fn {
    ($predicate:expr, $time:tt) => {
        Box::new(
            move |amount| {
                if $predicate {
                    amount * $time
                } else {
                    -amount
                }
            },
        )
    };
}
