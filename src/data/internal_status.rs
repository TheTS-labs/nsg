//! Portal's order status

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum InternalStatusError {
    /// Provided `&str` didn't match any internal status and thus can't be
    /// represented as [`InternalStatus`]
    InvalidStrStatus(String),
}

/// Portal's order status
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum InternalStatus {
    Assigned,
    Returned,
    Completed,
    Rejected,
    NotActivated,
    New,
    Agreed,
    NotCompleted,
    Delayed,
}

impl FromStr for InternalStatus {
    type Err = InternalStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Назначено" => Ok(InternalStatus::Assigned),
            "Возврат (от монтажника)" => Ok(InternalStatus::Returned),
            "Выполнено" => Ok(InternalStatus::Completed),
            "Отказ" => Ok(InternalStatus::Rejected),
            "Не активировано" => Ok(InternalStatus::NotActivated),
            "Новая" => Ok(InternalStatus::New),
            "Договорено" => Ok(InternalStatus::Agreed),
            "Не выполнено" => Ok(InternalStatus::NotCompleted),
            "Отложено" => Ok(InternalStatus::Delayed),
            _ => Err(InternalStatusError::InvalidStrStatus(s.to_string())),
        }
    }
}
