use std::str::FromStr;

use crate::data::status::{Status, StatusError};

#[test]
fn from_str() {
    assert_eq!(Status::from_str("Назначена в график"), Ok(Status::Assigned));
    assert_eq!(Status::from_str("В работе"), Ok(Status::InWork));
    assert_eq!(Status::from_str("Выполнена"), Ok(Status::Completed));
    assert_eq!(Status::from_str("Отклонена"), Ok(Status::Declined));
    assert_eq!(Status::from_str("Удалена"), Ok(Status::Deleted));
    assert_eq!(Status::from_str("Отказ"), Ok(Status::Rejected));
    assert_eq!(Status::from_str(""), Ok(Status::Empty));
    assert_eq!(Status::from_str("Принята в работу"), Ok(Status::AcceptedForWork));
    assert_eq!(
        Status::from_str("???"),
        Err(StatusError::InvalidStatus("???".to_string()))
    );
}
