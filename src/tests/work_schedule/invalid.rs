use std::fs;

use paste::paste;

use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::work_schedule::WorkSchedule;

macro_rules! test {
    ($($var:ident: { $($expected:expr, $name:ident, $method:ident $(, $method_arg:expr)?;)* }),* $(,)?) => {
        $($(paste! {
            #[test]
            fn [<$var _ $name>]() {
                let html = fs::read_to_string(
                    format!("src/tests/assets/work_schedule/invalid/{}.html", stringify!($var))
                ).expect("Should have been able to read the file");

                let work_schedule = WorkSchedule::from(&html);
                let order = work_schedule.0.$method($($method_arg)*).unwrap();

                assert_eq!(
                    order.$var, $expected,
                    "order.{} != {}. Got {:?} ({:#?})",
                    stringify!($var), stringify!($expected), order.$var, order
                );
            }
        })*)*
    }
}

test! {
    // Test HTML should be at src/tests/assets/work_schedule/invalid/order_index.html
    // And tests are asserted against order_index
    // To recap: Parse WokSchedule from src/tests/assets/work_schedule/invalid/order_index.html,
    // then get .first() Order and assert_eq!(Order.order_index, Some(Err(SerializableIntErrorKind::PosOverflow)))
    order_index: {
        //         order_index's expected value,       Testcase name, Method to get Order from WorkSchedule
        Some(Err(SerializableIntErrorKind::PosOverflow)), invalid_u8, first;
    },
    order_id: {
        Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32, first;
    },
    internal_order_id: {
        None, no_rowid, first;
        Some(Err(SerializableIntErrorKind::InvalidDigit)), invalid_u32, get, 1;
    },
    time_constrains: {
        None, no_second_half, first;
        None, no_text_first_half, get, 1;
        None, no_text_second_half, get, 2;
        None, no_first_half, get, 3;
    },
    order_type: {
        None, no_element, first;
        None, no_text, get, 1;
    },
    client: {
        None, no_element, first;
        None, no_text, get, 1;
    },
}
