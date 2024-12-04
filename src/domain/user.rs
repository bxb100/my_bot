#[derive(Debug)]
pub struct User {
    pub user_id: u64,
    pub name: Option<String>,
    pub points: u64,
    pub daily_reward: i64,
}
