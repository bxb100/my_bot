use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::UserId;
use teloxide::Bot;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    RequestError(#[from] teloxide::RequestError),

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type MyBot = DefaultParseMode<Bot>;
pub type MyResult<T> = Result<T, MyError>;
pub type ParsedCallbackData<'a> = (&'a str, UserId, &'a str);
