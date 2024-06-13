use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum StatusError {
    InvalidStatus(String),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum Status {
    Assigned,
    InWork,
    Completed,
    Declined,
    Deleted,
    Rejected,
    Empty,
    AcceptedForWork,
}

impl FromStr for Status {
    type Err = StatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Назначена в график" => Ok(Status::Assigned),
            "В работе" => Ok(Status::InWork),
            "Выполнена" => Ok(Status::Completed),
            "Отклонена" => Ok(Status::Declined),
            "Удалена" => Ok(Status::Deleted),
            "Отказ" => Ok(Status::Rejected),
            "" => Ok(Status::Empty),
            "Принята в работу" => Ok(Status::AcceptedForWork),
            _ => Err(StatusError::InvalidStatus(s.to_string())),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Status::Assigned => "Назначена",
            Status::InWork => "В работе",
            Status::Completed => "Выполнена",
            Status::Declined => "Отклонена",
            Status::Deleted => "Удалена",
            Status::Rejected => "Отказ",
            Status::Empty => "[-]",
            Status::AcceptedForWork => "Принята в работу",
        })
    }
}
