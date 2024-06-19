use std::fs;

use paste::paste;

use crate::brief_request::BriefRequest;
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::tests::test;

test! {
    "brief_request", BriefRequest;
    internal_order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32; },
    order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32; },
    creation_date: { Some(Err(SerializableParseErrorKind::Invalid)), invalid_datetime; },
    // TODO: If any part in address is empty return None
    // address: { None, invalid_address; },
}

#[test]
fn should_not_guarantee() {
    let html = fs::read_to_string("src/tests/assets/brief_request/valid/1.html")
        .expect("Should have been able to read the file");

    let mut brief_request = BriefRequest::from(&html);
    brief_request.phones.clear();

    assert!(brief_request.into_guaranteed().is_none());
}
