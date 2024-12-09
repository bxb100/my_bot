use crate::dao::gambles;
use crate::dao::gambles::{get_by_user_id_and_serial_id, Gambles};
use crate::dao::users::{get_by_id, insert, Users};
use crate::db::Database;
use crate::types::{MyBot, MyResult};
use chrono::Utc;
use log::info;
use teloxide::prelude::Requester;

pub async fn handler(
    bot: MyBot,
    database: Database,
    callback_query: teloxide::prelude::CallbackQuery,
) -> MyResult<()> {
    info!("callback_query: {:?}", callback_query.data);
    // check user exist
    let user_id = callback_query.from.id.0;
    info!("call_back_query: {:?}", callback_query.from);

    if get_by_id(database.pool, user_id as i64).await?.is_none() {
        insert(
            database.pool,
            Users {
                id: user_id as i64,
                name: callback_query.from.username.clone(),
                points: 1000,
                daily_reward: Some(Utc::now()),
            },
        )
        .await?;
    }

    if let Some(data) = callback_query.data {
        let (action, id) = crate::utils::decode_call_data(&data)?;
        let exist = get_by_user_id_and_serial_id(database.pool, user_id as i64, id).await?;
        if exist.is_none() {
            gambles::insert(
                database.pool,
                Gambles {
                    serial_id: id.to_string(),
                    user_id: user_id as i64,
                    user_name: callback_query.from.username,
                    action: action.to_string(),
                    ..Gambles::default()
                },
            )
            .await?;
        }
    }

    bot.answer_callback_query(callback_query.id).await?;
    Ok(())
}
