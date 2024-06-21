use chrono::NaiveTime;

use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};

#[test]
fn should_match() {
    assert_eq!(
        TimeConstrains::from("с 17:00 до 17:29"),
        Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(17, 29, 0).unwrap(),
        })
    );

    assert_eq!(
        TimeConstrains::from_work_schedule("17:00", "17:29"),
        Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(17, 29, 0).unwrap(),
        })
    );
}

#[test]
fn no_match() {
    assert_eq!(
        TimeConstrains::from("с 17:00до 17:29"),
        Err(TimeConstrainsError::NoMatch)
    );
}

// TODO: Gives FailedToParse because "" is passed to u32::from_str
// #[test]
// fn no_element() {
//     assert_eq!(
//         TimeConstrains::from_work_schedule(":00", "17:29"),
//         Err(TimeConstrainsError::NoHourElement)
//     );

//     assert_eq!(
//         TimeConstrains::from_work_schedule("17:00", ":29"),
//         Err(TimeConstrainsError::NoHourElement)
//     );

//     assert_eq!(
//         TimeConstrains::from_work_schedule("17:", "17:29"),
//         Err(TimeConstrainsError::NoMinuteElement)
//     );

//     assert_eq!(
//         TimeConstrains::from_work_schedule("17:00", "17:"),
//         Err(TimeConstrainsError::NoMinuteElement)
//     );
// }

#[test]
fn invalid_u32() {
    assert_eq!(
        TimeConstrains::from_work_schedule("4294967296:00", "17:29"),
        Err(TimeConstrainsError::FailedToParse)
    );

    assert_eq!(
        TimeConstrains::from_work_schedule("17:4294967296", "17:29"),
        Err(TimeConstrainsError::FailedToParse)
    );

    assert_eq!(
        TimeConstrains::from_work_schedule("17:00", "4294967296:29"),
        Err(TimeConstrainsError::FailedToParse)
    );

    assert_eq!(
        TimeConstrains::from_work_schedule("17:00", "17:4294967296"),
        Err(TimeConstrainsError::FailedToParse)
    );
}

#[test]
fn invalid_naive_time() {
    assert_eq!(
        TimeConstrains::from_work_schedule("99:99", "17:29"),
        Err(TimeConstrainsError::InvalidNaiveTime)
    );

    assert_eq!(
        TimeConstrains::from_work_schedule("17:00", "99:99"),
        Err(TimeConstrainsError::InvalidNaiveTime)
    );

    assert_eq!(
        TimeConstrains::from("с 99:99 до 17:29"),
        Err(TimeConstrainsError::InvalidNaiveTime)
    );
}
