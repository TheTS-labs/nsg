//! Kyivstar's order status

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum StatusError {
    /// Provided `&str` didn't match any order type and thus can't be
    /// represented as [`Status`]
    InvalidStatus(String),
}

/// Kyivstar's order status
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
