use crate::handlers::group_message::handler;
use crate::types::MyError;
use teloxide::dispatching::UpdateHandler;
use teloxide::dptree;
use teloxide::prelude::Message;

pub fn route() -> UpdateHandler<MyError> {
    dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup()).endpoint(handler)
}
