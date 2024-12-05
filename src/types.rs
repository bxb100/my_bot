use teloxide::adaptors::DefaultParseMode;
use teloxide::Bot;

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum MyError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    RequestError(#[from] teloxide::RequestError),

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type MyBot = DefaultParseMode<Bot>;
pub type MyResult<T> = Result<T, MyError>;
