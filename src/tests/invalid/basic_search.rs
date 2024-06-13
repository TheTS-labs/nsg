use std::fs;

use crate::basic_search::BasicSearch;
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;

#[test]
fn internal_order_id_invalid_u32() {
    let html = fs::read_to_string("src/tests/resources/invalid/basic_search/internal_order_id.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.first().unwrap();

    assert_eq!(
        entry.internal_order_id,
        Some(Err(SerializableIntErrorKind::InvalidDigit))
    );
}

#[test]
fn order_id_invalid_u32() {
    let html = fs::read_to_string("src/tests/resources/invalid/basic_search/order_id.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.first().unwrap();

    assert_eq!(entry.order_id, Some(Err(SerializableIntErrorKind::InvalidDigit)));
}

#[test]
fn last_updated_invalid_date() {
    let html = fs::read_to_string("src/tests/resources/invalid/basic_search/last_updated.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.first().unwrap();

    assert_eq!(entry.last_updated, Some(Err(SerializableParseErrorKind::Invalid)));
}
