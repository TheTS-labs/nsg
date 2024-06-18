use std::fs;

use paste::paste;

use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::view_request::ViewRequest;

macro_rules! test {
    ($($var:ident: { $($expected:expr, $name:ident;)* }),* $(,)?) => {
        $($(paste! {
            #[test]
            fn [<$var _ $name>]() {
                let html = fs::read_to_string(
                    format!("src/tests/assets/view_request/invalid/{}.html", stringify!($var))
                ).expect("Should have been able to read the file");

                let view_request = ViewRequest::from(&html);

                assert_eq!(
                    view_request.$var, $expected,
                    "view_request.{} != {}. Got {:?} ({:#?})",
                    stringify!($var), stringify!($expected), view_request.$var, view_request
                );
            }
        })*)*
    }
}

test! {
    internal_order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u8; },
    order_id: { Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32; },
    service: { None, empty_service; },
    assigned_for: { Some(Err(SerializableParseErrorKind::Invalid)), invalid_naive_date; },
}
