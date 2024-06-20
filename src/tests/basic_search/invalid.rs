use std::fs;

use paste::paste;

use crate::basic_search::BasicSearch;
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::tests::test;

test! {
    "basic_search", BasicSearch;
    order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32, first; },
    internal_order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32, first; },
    last_updated: { Some(Err(SerializableParseErrorKind::Invalid)), invalid_naive_date, first; },
}
