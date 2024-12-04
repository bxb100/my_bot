pub(crate) mod gamble_dice;

use teloxide::types::ChatId;
use teloxide::Bot;

pub trait GambleControl {
    async fn compute(&self, bot: Bot, chat_id: ChatId) -> Result<(), teloxide::RequestError>;
}
