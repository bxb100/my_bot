use crate::dao::jobs;
use crate::game::games;
use crate::handlers::stop_betting::{StopBettingJob, StopBettingMetadata};
use crate::jobs::{Job, GAME_END_BETTING_IN_SECS, GAME_STOP_BETTING_IN_SECS};
use crate::types::{Context, MyResult};
use crate::utils::serial_id_gen;
use async_trait::async_trait;
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use log::info;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration;
use teloxide::types::ChatId;

pub struct StartBettingJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartBettingMetadata {
    pub chat_id: i64,
}

#[async_trait]
impl Job for StartBettingJob {
    fn name(&self) -> &str {
        "start_betting"
    }

    async fn run(&self, ctx: &Context, metadata: Option<&String>) -> MyResult<()> {
        info!("start_betting, {:?}", metadata);

        let metadata: StartBettingMetadata = serde_json::from_str(metadata.unwrap())?;
        let games = games();

        // todo choose one game:
        let game = games.first().unwrap();
        let now = Utc::now().with_timezone(&Shanghai);
        let serial_id = serial_id_gen(&now);
        let stop = now.add(Duration::from_secs(GAME_STOP_BETTING_IN_SECS));
        let settle = now.add(Duration::from_secs(GAME_END_BETTING_IN_SECS));

        let text = game.message(now, settle, 1);
        let msg = game
            .start_play(
                &ctx.bot,
                ChatId(metadata.chat_id),
                serial_id.clone(),
                text.clone(),
            )
            .await?;

        // add next job
        let metadata = StopBettingMetadata {
            chat_id: msg.chat.id.0,
            message_id: msg.id.0,
            serial_id,
            game_name: game.name().to_string(),
            chat_username: msg.chat.username().map(str::to_string),
            text,
        };

        jobs::insert(
            ctx.database.pool,
            StopBettingJob.name(),
            &stop.to_utc(),
            &serde_json::to_value(metadata)?,
        )
        .await?;

        Ok(())
    }
}
