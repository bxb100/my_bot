pub fn encode_call_data(action: &str, id: &str) -> String {
    format!("{}:{}", action, id)
}

pub fn decode_call_data(call_data: &str) -> (&str, &str) {
    let data: Vec<&str> = call_data.splitn(2, ":").collect();
    (data[0], data[1])
}
