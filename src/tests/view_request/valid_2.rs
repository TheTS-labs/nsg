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
    let html = fs::read_to_string("src/tests/assets/view_request/valid/2.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert!(view_request.into_guaranteed().is_some());
}

#[test]
fn fields() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/2.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.order_id, Some(Ok(13354143)));
    assert_eq!(view_request.internal_order_id, Some(Ok(950599)));
    assert_eq!(view_request.order_type, Some(Ok(OrderType::NetBroken)));
    assert_eq!(
        view_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-05-15 10:49:41+03:00").unwrap()))
    );
    assert_eq!(view_request.internal_status, Some(Ok(InternalStatus::NotActivated)));
    assert_eq!(
        view_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Професора Толока".to_string(),
            building:  "██".to_string(),
            apartment: "███".to_string(),
        })
    );
    assert_eq!(view_request.client, Some("█████████ ████████".to_string()));
    assert_eq!(view_request.service, None);
    assert_eq!(view_request.pa, Some("88████████80".to_string()));
    assert_eq!(view_request.seller, Some("█████████ ███████".to_string()));
    assert_eq!(
        view_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(12, 29, 0).unwrap(),
        }))
    );
    assert_eq!(view_request.installers, vec!["████ █████ █████████".to_string()]);
    assert_eq!(view_request.status, Some(Ok(Status::Deleted)));
    assert_eq!(view_request.phones, vec!["+38067███████".to_string()]);
    assert_eq!(
        view_request.assigned_for,
        Some(Ok(NaiveDate::from_ymd_opt(2024, 5, 16).unwrap()))
    );
}

#[test]
fn comments() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/2.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.comments, vec![
        Ok(FullComment {
            text:            Some("Статус у заказчика:Отклонена. Переведено в Не активировано".to_string()),
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 16:42:20+03:00").unwrap(),
            internal_status: InternalStatus::NotActivated,
        }),
        Ok(FullComment {
            text:            Some("Поменял патчкорд в ТКД ".to_string()),
            user:            "████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 15:38:18+03:00").unwrap(),
            internal_status: InternalStatus::Completed,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 11:07:48+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            Some(
                "Індикатор \"WAN\" не горить;DOWN;Обрив одієї із жил;Указать метр повреждения-Длина кабеля\tPair A \
                 length: 24meter(s) Pair B length: 1meter(s) Pair A state: Open Pair B state: Open;Указать метр \
                 повреждения: Длина кабеля\tPair A length: 24meter(s) Pair B length: 1meter(s) Pair A state: Open \
                 Pair B state: Open; Обрив одієї із жил\n\n"
                    .to_string()
            ),
            user:            "Система".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 10:55:33+03:00").unwrap(),
            internal_status: InternalStatus::New,
        })
    ])
}