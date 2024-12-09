use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Jobs {
    id: u64,
    name: String,
    scheduled_at: DateTime<Utc>,
    metadata: Option<String>,
    executed_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
}
