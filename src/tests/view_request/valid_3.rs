use std::fs;

use chrono::{DateTime, NaiveDate, NaiveTime};

use crate::data::address::Address;
use crate::data::full_comment::FullComment;
use crate::data::internal_status::InternalStatus;
use crate::data::order_type::OrderType;
use crate::data::status::Status;
use crate::data::time_constrains::TimeConstrains;
use crate::view_request::ViewRequest;

#[test]
fn should_guarantee() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/3.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert!(view_request.into_guaranteed().is_some());
}

#[test]
fn fields() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/3.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.order_id, Some(Ok(13355298)));
    assert_eq!(view_request.internal_order_id, Some(Ok(950767)));
    assert_eq!(view_request.order_type, Some(Ok(OrderType::OthersMaster)));
    assert_eq!(
        view_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-05-15 18:33:00+03:00").unwrap()))
    );
    assert_eq!(view_request.internal_status, Some(Ok(InternalStatus::Rejected)));
    assert_eq!(
        view_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "███".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(view_request.client, Some("Anonymous Anonymous Anonymous".to_string()));
    assert_eq!(view_request.service, None);
    assert_eq!(view_request.pa, Some("38097███████".to_string()));
    assert_eq!(view_request.seller, None);
    assert_eq!(
        view_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(17, 30, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(17, 59, 0).unwrap(),
        }))
    );
    assert_eq!(view_request.installers, vec![
        "██████ █████████ █████████████".to_string(),
        "██████ ██████ ███████████".to_string()
    ]);
    assert_eq!(view_request.status, Some(Ok(Status::Rejected)));
    assert_eq!(view_request.phones, vec!["+38097███████".to_string()]);
    assert_eq!(
        view_request.assigned_for,
        Some(Ok(NaiveDate::from_ymd_opt(2024, 5, 17).unwrap()))
    );
}

#[test]
fn comments() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/3.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.comments, vec![
        Ok(FullComment {
            text:            Some("ок".to_string()),
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-17 14:27:42+03:00").unwrap(),
            internal_status: InternalStatus::Rejected,
        }),
        Ok(FullComment {
            text:            Some("В отказ".to_string()),
            user:            "██████ █████████ █████████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-17 14:22:08+03:00").unwrap(),
            internal_status: InternalStatus::Returned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 17:34:30+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            Some("17,05".to_string()),
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 17:33:26+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 18:41:03+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "Система".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 18:35:47+03:00").unwrap(),
            internal_status: InternalStatus::New,
        })
    ])
}
