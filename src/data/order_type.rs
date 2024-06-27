//! Type of work on order
//!
//! # Terminology
//!
//! - **Active** - order for subscription (connection) to Kyivstar's network
//!   created from inside "Нет-Строй"
//! - **Passive** - as opposed to Active, it's order created by whoever outside
//!   "Нет-Строй"
//! - [`OrderType::SomeBitchComplained`] - is also used to give additional
//!   information for installer and/or logistician but who fucking cares, shut
//!   up your complaining mouth 😇

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum OrderTypeError {
    /// Provided `&str` didn't match any order type and thus can't be
    /// represented as [`OrderType`]
    InvalidOrderType(String),
}

/// Order type is a type of work on order that needs to be done
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

    // TODO: There's no "Work schedule format" or "Brief request format"
    // TODO: It's just type and subtype: Order of type "Сервис" and subtype
    // TODO: "Тюнер ДЕМОНТАЖ"
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
