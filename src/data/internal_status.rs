use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum InternalStatusError {
    InvalidStrStatus(String),
}

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

impl Display for InternalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            InternalStatus::Assigned => "Назначена",
            InternalStatus::Returned => "Возвращена",
            InternalStatus::Completed => "Выполнена",
            InternalStatus::Rejected => "Отклонена",
            InternalStatus::NotActivated => "Не активирована",
            InternalStatus::New => "Новая",
            InternalStatus::Agreed => "Договорена",
            InternalStatus::NotCompleted => "Не выполнена",
            InternalStatus::Delayed => "Отложена",
        })
    }
}
