//! [`IntErrorKind`] with derived `Serialize` and `Deserialize` implementations
//!
//! See [`IntErrorKind`] for more information

use std::num::IntErrorKind;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum SerializableIntErrorKind {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
}

impl From<IntErrorKind> for SerializableIntErrorKind {
    fn from(error_kind: IntErrorKind) -> Self {
        match error_kind {
            IntErrorKind::Empty => SerializableIntErrorKind::Empty,
            IntErrorKind::InvalidDigit => SerializableIntErrorKind::InvalidDigit,
            IntErrorKind::PosOverflow => SerializableIntErrorKind::PosOverflow,
            IntErrorKind::NegOverflow => SerializableIntErrorKind::NegOverflow,
            IntErrorKind::Zero => SerializableIntErrorKind::Zero,
            _ => unreachable!(),
        }
    }
}
