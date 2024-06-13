use chrono::format::ParseErrorKind;
use chrono::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Default, Deserialize)]
pub enum SerializableParseErrorKind {
    OutOfRange,
    Impossible,
    NotEnough,
    Invalid,
    TooShort,
    TooLong,
    BadFormat,
    #[default]
    Unknown,
}

impl From<ParseErrorKind> for SerializableParseErrorKind {
    fn from(error_kind: ParseErrorKind) -> Self {
        match error_kind {
            ParseErrorKind::OutOfRange => SerializableParseErrorKind::OutOfRange,
            ParseErrorKind::Impossible => SerializableParseErrorKind::Impossible,
            ParseErrorKind::NotEnough => SerializableParseErrorKind::NotEnough,
            ParseErrorKind::Invalid => SerializableParseErrorKind::Invalid,
            ParseErrorKind::TooShort => SerializableParseErrorKind::TooShort,
            ParseErrorKind::TooLong => SerializableParseErrorKind::TooLong,
            ParseErrorKind::BadFormat => SerializableParseErrorKind::BadFormat,
            _ => SerializableParseErrorKind::Unknown,
        }
    }
}

impl From<ParseError> for SerializableParseErrorKind {
    fn from(error: ParseError) -> Self {
        match error.kind() {
            ParseErrorKind::OutOfRange => SerializableParseErrorKind::OutOfRange,
            ParseErrorKind::Impossible => SerializableParseErrorKind::Impossible,
            ParseErrorKind::NotEnough => SerializableParseErrorKind::NotEnough,
            ParseErrorKind::Invalid => SerializableParseErrorKind::Invalid,
            ParseErrorKind::TooShort => SerializableParseErrorKind::TooShort,
            ParseErrorKind::TooLong => SerializableParseErrorKind::TooLong,
            ParseErrorKind::BadFormat => SerializableParseErrorKind::BadFormat,
            _ => SerializableParseErrorKind::Unknown,
        }
    }
}
