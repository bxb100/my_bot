use crate::db::Database;
use crate::domain::user::{UpsertUser, UserDao};
use crate::domain::wager::{UpsertWager, WagerDao};
use crate::types::{MyBot, MyResult};
use chrono::Local;
use log::info;
use teloxide::prelude::{CallbackQuery, Requester};

pub async fn handler(
    bot: MyBot,
    database: Database,
    callback_query: CallbackQuery,
) -> MyResult<()> {
    info!("callback_query: {:?}", callback_query.data);
    // check user exist
    let user_id = callback_query.from.id.0;
    info!("call_back_query: {:?}", callback_query.from);

    let user_dao = UserDao {
        database: database.clone(),
    };
    if user_dao.get_by_id(user_id as i64).await?.is_none() {
        user_dao
            .upsert(UpsertUser {
                id: user_id as i64,
                name: callback_query.from.username.clone(),
                points: 1000,
                daily_reward: Local::now().timestamp(),
            })
            .await?;
    }

    if let Some(data) = callback_query.data {
        let (action, id) = crate::utils::decode_call_data(&data);
        let wager_dao = WagerDao {
            database: database.clone(),
        };
        let exist = wager_dao
            .get_by_user_id_and_time_id(user_id as i64, id)
            .await?;
        if exist.is_none() {
            wager_dao
                .insert(UpsertWager {
                    time_id: id.to_string(),
                    user_id: user_id as i64,
                    user_name: callback_query.from.username,
                    action: action.to_string(),
                    amount: None,
                })
                .await;
        }
    }

    bot.answer_callback_query(callback_query.id).await?;
    Ok(())
}
