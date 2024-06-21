use std::str::FromStr;

use crate::data::internal_status::{InternalStatus, InternalStatusError};

#[test]
fn from_str() {
    assert_eq!(InternalStatus::from_str("Назначено"), Ok(InternalStatus::Assigned));
    assert_eq!(
        InternalStatus::from_str("Возврат (от монтажника)"),
        Ok(InternalStatus::Returned)
    );
    assert_eq!(InternalStatus::from_str("Выполнено"), Ok(InternalStatus::Completed));
    assert_eq!(InternalStatus::from_str("Отказ"), Ok(InternalStatus::Rejected));
    assert_eq!(
        InternalStatus::from_str("Не активировано"),
        Ok(InternalStatus::NotActivated)
    );
    assert_eq!(InternalStatus::from_str("Новая"), Ok(InternalStatus::New));
    assert_eq!(InternalStatus::from_str("Договорено"), Ok(InternalStatus::Agreed));
    assert_eq!(
        InternalStatus::from_str("Не выполнено"),
        Ok(InternalStatus::NotCompleted)
    );
    assert_eq!(InternalStatus::from_str("Отложено"), Ok(InternalStatus::Delayed));
    assert_eq!(
        InternalStatus::from_str("???"),
        Err(InternalStatusError::InvalidStrStatus("???".to_string()))
    );
}
