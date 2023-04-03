use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use walle::walle_core::prelude::{IntoMessage, MsgSegment};

#[derive(Debug)]
pub enum BotError {
    Msg(MsgSegment),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotError::Msg(e) => {
                write!(f, "{:?}", e.clone().into_message())
            }
        }
    }
}

impl StdError for BotError {}

pub type BotResult<T> = Result<T, BotError>;

impl BotError {
    pub fn to_msg_segment(self) -> MsgSegment {
        match self {
            BotError::Msg(e) => e,
        }
    }
}

impl From<BotError> for std::io::Error {
    fn from(arg: BotError) -> Self {
        arg.into()
    }
}

impl From<&dyn std::error::Error> for BotError {
    fn from(arg: &dyn std::error::Error) -> Self {
        return BotError::Msg(MsgSegment::from(arg.to_string()));
    }
}
macro_rules! bot_error {
    ($error:ty) => {
        impl From<$error> for BotError {
            fn from(arg: $error) -> Self {
                BotError::Msg(MsgSegment::from(arg.to_string()))
            }
        }
    };
}
bot_error!(&str);
bot_error!(String);
bot_error!(std::io::Error);
bot_error!(reqwest::Error);
bot_error!(serde_json::Error);
bot_error!(serde_yaml::Error);
bot_error!(rand::Error);
bot_error!(regex::Error);
bot_error!(rbs::Error);
bot_error!(rbatis::Error);
bot_error!(rbdc_sqlite::error::SqliteError);
