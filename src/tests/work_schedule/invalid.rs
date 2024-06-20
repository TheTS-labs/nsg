use std::fs;

use paste::paste;

use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::tests::test;
use crate::work_schedule::WorkSchedule;

test! {
    "work_schedule", WorkSchedule;
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
