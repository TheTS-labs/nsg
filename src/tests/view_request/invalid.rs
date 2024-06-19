use std::fs;

use paste::paste;

use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::view_request::ViewRequest;
use crate::tests::test;

test! {
    "view_request", ViewRequest;
    internal_order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u8; },
    order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32; },
    service: { None, empty_service; },
    assigned_for: { Some(Err(SerializableParseErrorKind::Invalid)), invalid_naive_date; },
}

#[test]
fn should_not_guarantee() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/1.html")
        .expect("Should have been able to read the file");

    let mut view_request = ViewRequest::from(&html);
    view_request.phones.clear();

    assert!(view_request.into_guaranteed().is_none());
}
