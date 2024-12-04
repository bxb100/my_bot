#[derive(Debug)]
pub struct Wager {
    pub id: u64,
    pub time_id: String,
    pub user_id: u64,
    pub action: String,
    pub amount: Option<u32>,
}
