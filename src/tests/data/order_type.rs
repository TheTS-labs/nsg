use std::str::FromStr;

use crate::data::order_type::{OrderType, OrderTypeError};

#[test]
fn from_str() {
    assert_eq!(
        OrderType::from_str("Сервис-Жалобы на обслуживание"),
        Ok(OrderType::NetBroken)
    );

    assert_eq!(OrderType::from_str("Жалобы на обслуживание"), Ok(OrderType::NetBroken));

    assert_eq!(
        OrderType::from_str("Сервис-Тюнер ДЕМОНТАЖ"),
        Ok(OrderType::TunerRemoval)
    );

    assert_eq!(OrderType::from_str("Тюнер ДЕМОНТАЖ"), Ok(OrderType::TunerRemoval));

    assert_eq!(
        OrderType::from_str("Сервис-Тюнер ЗАМЕНА"),
        Ok(OrderType::TunerReplacement)
    );

    assert_eq!(OrderType::from_str("Тюнер ЗАМЕНА"), Ok(OrderType::TunerReplacement));

    assert_eq!(
        OrderType::from_str("Сервис-Тюнер УСТАНОВКА"),
        Ok(OrderType::TunerInstallation)
    );

    assert_eq!(OrderType::from_str("Тюнер УСТАНОВКА"), Ok(OrderType::TunerInstallation));

    assert_eq!(OrderType::from_str("Сервис-Гігабіт"), Ok(OrderType::NetGigabit));

    assert_eq!(OrderType::from_str("Гігабіт"), Ok(OrderType::NetGigabit));

    assert_eq!(
        OrderType::from_str("Актив-Восстановление"),
        Ok(OrderType::NetRecoveryActive)
    );

    assert_eq!(
        OrderType::from_str("Пассив-Восстановление"),
        Ok(OrderType::NetRecoveryPassive)
    );

    assert_eq!(OrderType::from_str("Восстановление"), Ok(OrderType::NetRecoveryUnknown));

    assert_eq!(
        OrderType::from_str("Актив-Новое подключение"),
        Ok(OrderType::NetNewActive)
    );

    assert_eq!(
        OrderType::from_str("Пассив-Новое подключение"),
        Ok(OrderType::NetNewPassive)
    );

    assert_eq!(OrderType::from_str("Новое подключение"), Ok(OrderType::NetNewUnknown));

    assert_eq!(
        OrderType::from_str("Сервис-Жалобы на обслуживание"),
        Ok(OrderType::NetBroken)
    );

    assert_eq!(OrderType::from_str("Жалобы на обслуживание"), Ok(OrderType::NetBroken));

    assert_eq!(
        OrderType::from_str("Сервис-Мастер\nКорпоративный"),
        Ok(OrderType::OthersMaster)
    );

    assert_eq!(OrderType::from_str("Мастер"), Ok(OrderType::OthersMaster));

    assert_eq!(OrderType::from_str("Актив-Переезд"), Ok(OrderType::NetRelocationActive));

    assert_eq!(
        OrderType::from_str("Пассив-Переезд"),
        Ok(OrderType::NetRelocationPassive)
    );

    assert_eq!(OrderType::from_str("Переезд"), Ok(OrderType::NetRelocationUnknown));

    assert_eq!(
        OrderType::from_str("Сервис-Жалобы по включениям"),
        Ok(OrderType::SomeBitchComplained)
    );

    assert_eq!(
        OrderType::from_str("Жалобы по включениям"),
        Ok(OrderType::SomeBitchComplained)
    );

    assert_eq!(
        OrderType::from_str("???"),
        Err(OrderTypeError::InvalidOrderType("???".to_string()))
    );
}
