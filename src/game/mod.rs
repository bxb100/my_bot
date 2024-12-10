use crate::game::double_dice::DoubleDice;
use crate::types::{MyBot, MyResult};
use crate::utils::serial_id_gen;
use async_trait::async_trait;
use chrono::DateTime;
use chrono_tz::Tz;
use std::collections::HashMap;
use teloxide::prelude::ChatId;
use teloxide::types::Message;

mod double_dice;

/// guarantee that only use for one job simultaneously
pub type IFn = Box<dyn Fn(isize) -> isize + Send + Sync>;

pub fn games() -> Vec<Box<dyn Game + Send + Sync>> {
    vec![Box::new(DoubleDice)]
}

#[async_trait]
pub trait Game {
    fn name(&self) -> &str;

    fn message(&self, now: DateTime<Tz>, settle: DateTime<Tz>, scale: u32) -> String {
        let id = serial_id_gen(&now);

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
            scale,
            settle.format("%Y-%m-%d %H:%M:%S")
        };

        text
    }

    async fn start_play(
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
