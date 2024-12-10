use crate::types::{MyError, MyResult};
use serde::de::DeserializeOwned;
use teloxide::types::{ChatId, Message, MessageId};

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

pub fn deserialize_metadata<T: DeserializeOwned>(metadata: &serde_json::Value) -> MyResult<T> {
    let metadata = serde_json::from_value(metadata.clone())?;
    Ok(metadata)
}

pub fn telegram_message_url(
    chat_id: i64,
    chat_username: Option<String>,
    message_id: i32,
) -> String {
    Message::url_of(
        ChatId(chat_id),
        chat_username.as_deref(),
        MessageId(message_id),
    )
    .map_or("javascript:;".to_string(), |url| url.to_string())
}

#[test]
pub fn test_decode_call_data() {
    let data = "bet:1";
    let (action, id) = decode_call_data(data).unwrap();
    assert_eq!(action, "bet");
    assert_eq!(id, "1");
}
