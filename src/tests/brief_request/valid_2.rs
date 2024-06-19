use std::fs;

use chrono::{DateTime, NaiveTime};

use crate::brief_request::BriefRequest;
use crate::data::address::Address;
use crate::data::comment::Comment;
use crate::data::internal_status::InternalStatus;
use crate::data::order_type::OrderType;
use crate::data::status::Status;
use crate::data::time_constrains::TimeConstrains;

#[test]
fn should_guarantee() {
    let html = fs::read_to_string("src/tests/assets/brief_request/valid/2.html")
        .expect("Should have been able to read the file");

    let brief_request = BriefRequest::from(&html);

    assert!(brief_request.into_guaranteed().is_some());
}

#[test]
fn fields() {
    let html = fs::read_to_string("src/tests/assets/brief_request/valid/2.html")
        .expect("Should have been able to read the file");

    let brief_request = BriefRequest::from(&html);

    assert_eq!(brief_request.order_id, Some(Ok(13373621)));
    assert_eq!(brief_request.internal_order_id, Some(Ok(953389)));
    assert_eq!(brief_request.order_type, Some(Ok(OrderType::NetBroken)));
    assert_eq!(
        brief_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-05-26 12:41:56+03:00").unwrap()))
    );
    assert_eq!(brief_request.internal_status, Some(Ok(InternalStatus::Assigned)));
    assert_eq!(
        brief_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(brief_request.client, Some("████████ ██████ ████████████".to_string()));
    assert_eq!(brief_request.service, Some("Luck(2051)".to_string()));
    assert_eq!(brief_request.pa, Some("71███92".to_string()));
    assert_eq!(
        brief_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(13, 59, 0).unwrap(),
        }))
    );
    assert_eq!(brief_request.installers, vec![
        "████ █████ █████████".to_string(),
        "██████ █████ ███████".to_string()
    ]);
    assert_eq!(
        brief_request.last_comment,
        Some(Ok(Comment {
            datetime: Some(DateTime::parse_from_rfc3339("2024-05-26 12:47:12+03:00").unwrap()),
            text:     "Індикатор \"WAN \"не горить;DOWN;Кабель відкритий на N більше 20 метрів;Repair \
                       date-2024-05-26;Указать метр повреждения-Pair A length: 52meter(s) Pair B length: 56meter(s) \
                       Pair A state: Open Pair B state: Open;Указать метр повреждения: Pair A length: 52meter(s) Pair \
                       B length: 56meter(s) Pair A state: Open Pair B state: Open; Кабель відкритий на N більше 20 \
                       метрів \n\nНет линка по кабелю. Просьба проверить ПМ.\nУказать метр повреждения: Pair A \
                       length: 52meter(s) Pair B length: 56meter(s) "
                .to_string(),
            user:     Some("Система".to_string()),
        }))
    );
    assert_eq!(
        brief_request.first_comment,
        Some(Ok(Comment {
            datetime: None,
            text:     "Індикатор \"WAN\" не горить;DOWN;Кабель відкритий на N більше 20 метрів;Repair \
                       date-2024-05-26;Указать метр повреждения-Pair A length: 52meter(s) Pair B length: 56meter(s) \
                       Pair A state: Open Pair B state: Open;Указать метр повреждения: Pair A length: 52meter(s) Pair \
                       B length: 56meter(s) Pair A state: Open Pair B state: Open; Кабель відкритий на N більше 20 \
                       метрів\n\nНет линка по кабелю. Просьба проверить ПМ.\nУказать метр повреждения: Pair A length: \
                       52meter(s) Pair B length: 56meter(s)"
                .to_string(),
            user:     None,
        }))
    );
    assert_eq!(brief_request.status, Some(Ok(Status::Assigned)));
    assert_eq!(brief_request.phones, vec![
        "+38097███████".to_string(),
        "+38097███████".to_string()
    ]);
}
