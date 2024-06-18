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
fn fields() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/4.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.order_id, Some(Ok(13413282)));
    assert_eq!(view_request.internal_order_id, Some(Ok(958790)));
    assert_eq!(view_request.order_type, Some(Ok(OrderType::NetNewUnknown)));
    assert_eq!(
        view_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-06-15 15:39:12+03:00").unwrap()))
    );
    assert_eq!(view_request.internal_status, Some(Ok(InternalStatus::Assigned)));
    assert_eq!(
        view_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Бочарова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(view_request.client, Some("█████ ████ █████████████".to_string()));
    assert_eq!(view_request.service, Some("Luck(2051)".to_string()));
    assert_eq!(view_request.pa, Some("88████████78".to_string()));
    assert_eq!(view_request.seller, Some("38097███████".to_string()));
    assert_eq!(
        view_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(13, 00, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(13, 29, 0).unwrap(),
        }))
    );
    assert_eq!(view_request.installers, vec!["████ █████ █████████".to_string()]);
    assert_eq!(view_request.status, Some(Ok(Status::InWork)));
    assert_eq!(view_request.phones, vec!["+38068███████".to_string()]);
    assert_eq!(
        view_request.assigned_for,
        Some(Ok(NaiveDate::from_ymd_opt(2024, 6, 17).unwrap()))
    );
}

#[test]
fn comments() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/4.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.comments, vec![
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-06-15 16:42:45+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-06-15 16:41:28+03:00").unwrap(),
            internal_status: InternalStatus::Agreed,
        }),
        Ok(FullComment {
            text:            Some("17,06 с 13 до 15...в районе обеда".to_string()),
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-06-15 16:41:15+03:00").unwrap(),
            internal_status: InternalStatus::Agreed,
        }),
        Ok(FullComment {
            text:            Some("\u{a0}Коментар клієнта:Новый жилец. Согласен на 17.06".to_string()),
            user:            "Система".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-06-15 15:46:25+03:00").unwrap(),
            internal_status: InternalStatus::New,
        })
    ])
}
