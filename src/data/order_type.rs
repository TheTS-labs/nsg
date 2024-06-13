use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum OrderTypeError {
    InvalidOrderType(String),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum OrderType {
    TunerRemoval,
    TunerInstallation,
    TunerReplacement,
    NetGigabit,
    NetRecoveryActive,
    NetRecoveryPassive,
    NetNewActive,
    NetNewPassive,
    NetBroken,
    OthersMaster,
    NetRelocationActive,
    NetRelocationPassive,
    SomeBitchComplained,
    NetRecoveryUnknown,
    NetNewUnknown,
    NetRelocationUnknown,
}

impl FromStr for OrderType {
    type Err = OrderTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // --- Work schedule format ---
            "Сервис-Тюнер ДЕМОНТАЖ" => Ok(OrderType::TunerRemoval),
            "Сервис-Тюнер ЗАМЕНА" => Ok(OrderType::TunerReplacement),
            "Сервис-Тюнер УСТАНОВКА" => Ok(OrderType::TunerInstallation),
            "Сервис-Гігабіт" => Ok(OrderType::NetGigabit),
            "Актив-Восстановление" => Ok(OrderType::NetRecoveryActive),
            "Пассив-Восстановление" => Ok(OrderType::NetRecoveryPassive),
            "Актив-Новое подключение" => Ok(OrderType::NetNewActive),
            "Пассив-Новое подключение" => Ok(OrderType::NetNewPassive),
            "Сервис-Жалобы на обслуживание" => Ok(OrderType::NetBroken),
            s if s.contains("Сервис-Мастер") => Ok(OrderType::OthersMaster),
            "Актив-Переезд" => Ok(OrderType::NetRelocationActive),
            "Пассив-Переезд" => Ok(OrderType::NetRelocationPassive),
            "Сервис-Жалобы по включениям" => Ok(OrderType::SomeBitchComplained),
            // --- Brief request format ---
            "Тюнер ДЕМОНТАЖ" => Ok(OrderType::TunerRemoval),
            "Тюнер ЗАМЕНА" => Ok(OrderType::TunerReplacement),
            "Тюнер УСТАНОВКА" => Ok(OrderType::TunerInstallation),
            "Гігабіт" => Ok(OrderType::NetGigabit),
            "Восстановление" => Ok(OrderType::NetRecoveryUnknown),
            "Новое подключение" => Ok(OrderType::NetNewUnknown),
            "Жалобы на обслуживание" => Ok(OrderType::NetBroken),
            "Мастер" => Ok(OrderType::OthersMaster),
            "Переезд" => Ok(OrderType::NetRelocationUnknown),
            "Жалобы по включениям" => Ok(OrderType::SomeBitchComplained),

            _ => Err(OrderTypeError::InvalidOrderType(s.to_string())),
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OrderType::TunerRemoval => "Демонтаж тюнера",
            OrderType::TunerInstallation => "Установка тюнера",
            OrderType::TunerReplacement => "Замена тюнера",
            OrderType::NetGigabit => "Гигабит",
            OrderType::NetRecoveryActive => "Восстановление (актив)",
            OrderType::NetRecoveryPassive => "Восстановление (пассив)",
            OrderType::NetNewActive => "Новое подключение (актив)",
            OrderType::NetNewPassive => "Новое подключение (пассив)",
            OrderType::NetBroken => "Ремонт",
            OrderType::OthersMaster => "Мастер",
            OrderType::NetRelocationActive => "Переезд (актив)",
            OrderType::NetRelocationPassive => "Переезд (пассив)",
            OrderType::SomeBitchComplained => "Какая-то сучка пожаловалась",
            OrderType::NetRecoveryUnknown => "Восстановление",
            OrderType::NetNewUnknown => "Новое подключение",
            OrderType::NetRelocationUnknown => "Переезд",
        })
    }
}
