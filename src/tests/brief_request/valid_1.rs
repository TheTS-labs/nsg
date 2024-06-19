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
    let html = fs::read_to_string("src/tests/assets/brief_request/valid/1.html")
        .expect("Should have been able to read the file");

    let brief_request = BriefRequest::from(&html);

    assert!(brief_request.into_guaranteed().is_some());
}

#[test]
fn fields() {
    let html = fs::read_to_string("src/tests/assets/brief_request/valid/1.html")
        .expect("Should have been able to read the file");

    let brief_request = BriefRequest::from(&html);

    assert_eq!(brief_request.order_id, Some(Ok(13356689)));
    assert_eq!(brief_request.internal_order_id, Some(Ok(950974)));
    assert_eq!(brief_request.order_type, Some(Ok(OrderType::NetGigabit)));
    assert_eq!(
        brief_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-05-16 15:10:23+03:00").unwrap()))
    );
    assert_eq!(brief_request.internal_status, Some(Ok(InternalStatus::Completed)));
    assert_eq!(
        brief_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Товариська".to_string(),
            building:  "█████".to_string(),
            apartment: "█".to_string(),
        })
    );
    assert_eq!(brief_request.client, Some("████████ █████████ ██████████".to_string()));
    assert_eq!(brief_request.service, None);
    assert_eq!(brief_request.pa, Some("00██████20".to_string()));
    assert_eq!(
        brief_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(11, 59, 0).unwrap(),
        }))
    );
    assert_eq!(brief_request.installers, vec!["████ █████ █████████".to_string()]);
    assert_eq!(
        brief_request.last_comment,
        Some(Ok(Comment {
            datetime: Some(DateTime::parse_from_rfc3339("2024-05-17 12:30:46+03:00").unwrap()),
            text:     "Частично заменил кабель, нужен гигабитный свитч ".to_string(),
            user:     Some("████ █████ █████████".to_string()),
        }))
    );
    assert_eq!(
        brief_request.first_comment,
        Some(Ok(Comment {
            datetime: None,
            text:     "Заявка на замену кабеля;Замена кабеля с 2 на 4 пары для Гигабита;Для можливості замовлення ТП \
                       Гігабіт;Repair date-2024-05-17;Указать метр повреждения: ; Для можливості замовлення ТП \
                       Гігабіт\n\n"
                .to_string(),
            user:     None,
        }))
    );
    assert_eq!(brief_request.status, Some(Ok(Status::Completed)));
    assert_eq!(brief_request.phones, vec![
        "+38067███████".to_string(),
        "+38067███████".to_string()
    ]);
}