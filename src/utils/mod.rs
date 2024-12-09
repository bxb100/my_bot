use crate::types::{MyError, MyResult};

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

#[test]
pub fn test_decode_call_data() {
    let data = "bet:1";
    let (action, id) = decode_call_data(data).unwrap();
    assert_eq!(action, "bet");
    assert_eq!(id, "1");
}
