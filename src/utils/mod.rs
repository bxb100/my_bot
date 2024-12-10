use crate::types::{MyError, MyResult};
use chrono::DateTime;
use chrono_tz::Tz;
use teloxide::types::{ChatId, ChatKind, Message, MessageId, PublicChatKind};

pub fn encode_call_data(action: &str, id: &str) -> String {
    format!("{}:{}", action, id)
}

pub fn decode_call_data(call_data: &str) -> MyResult<(&str, &str)> {
    let data: Vec<&str> = call_data.splitn(2, ":").collect();
    if data.len() != 2 {
        Err(MyError::Unknown(format!(
            "decode_call_date failed: {}",
            call_data
        )))
    } else {
        Ok((data[0], data[1]))
    }
}

pub fn telegram_message_url(chat_id: i64, chat_username: Option<&str>, message_id: i32) -> String {
    Message::url_of(ChatId(chat_id), chat_username, MessageId(message_id))
        .map_or("javascript:;".to_string(), |url| url.to_string())
}

pub fn serial_id_gen(time: &DateTime<Tz>) -> String {
    time.format("%y%m%d%H%M").to_string()
}

#[test]
pub fn test_decode_call_data() {
    let data = "bet:1";
    let (action, id) = decode_call_data(data).unwrap();
    assert_eq!(action, "bet");
    assert_eq!(id, "1");
}

pub fn get_chat_kind(kind: &ChatKind) -> &str {
    match kind {
        ChatKind::Public(kind) => match kind.kind {
            PublicChatKind::Channel(_) => "channel",
            PublicChatKind::Group(_) => "group",
            PublicChatKind::Supergroup(_) => "supergroup",
        },
        ChatKind::Private(_) => "private",
    }
}
