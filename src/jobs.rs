use crate::config::BOT_CONFIG;
use crate::dao::jobs::JobSchedule;
use crate::handlers::settle_bets::SettleBetsJob;
use crate::handlers::start_betting::{StartBettingJob, StartBettingMetadata};
use crate::handlers::stop_betting::StopBettingJob;
use crate::types::{Context, MyResult};
use async_trait::async_trait;

pub const GAME_STOP_BETTING_IN_SECS: u64 = 240;
pub const GAME_END_BETTING_IN_SECS: u64 = 300;
pub const GAME_STOP_TO_END_BETTING_IN_SECS: u64 = 60;

#[async_trait]
pub trait Job {
    fn name(&self) -> &str;

    async fn run(&self, id: &i64, ctx: &Context, metadata: &serde_json::Value) -> MyResult<()>;
}

pub fn jobs() -> Vec<Box<dyn Job + Send + Sync>> {
    vec![
        Box::new(SettleBetsJob),
        Box::new(StartBettingJob),
        Box::new(StopBettingJob),
    ]
}

pub fn default_job_schedules() -> Vec<JobSchedule> {
    vec![JobSchedule {
        name: StartBettingJob.name(),
        schedule: "0 0/10 08-23 * * ?".parse().unwrap(),
        metadata: serde_json::to_value(StartBettingMetadata {
            chat_id: BOT_CONFIG.chat_id,
        })
        .unwrap(),
    }]
}
