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
    let html = fs::read_to_string("src/tests/resources/valid/view_request_1.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.order_id, Some(Ok(13351186)));
    assert_eq!(view_request.internal_order_id, Some(Ok(950207)));
    assert_eq!(view_request.order_type, Some(Ok(OrderType::NetGigabit)));
    assert_eq!(
        view_request.creation_date,
        Some(Ok(DateTime::parse_from_rfc3339("2024-05-13 16:16:00+03:00").unwrap()))
    );
    assert_eq!(view_request.internal_status, Some(Ok(InternalStatus::Completed)));
    assert_eq!(
        view_request.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(view_request.client, Some("█████ ██████ ████████████".to_string()));
    assert_eq!(view_request.service, None);
    assert_eq!(view_request.pa, Some("82███59".to_string()));
    assert_eq!(view_request.seller, Some("██████ ███████".to_string()));
    assert_eq!(
        view_request.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(11, 29, 0).unwrap(),
        }))
    );
    assert_eq!(view_request.installers, vec!["████ █████ █████████".to_string()]);
    assert_eq!(view_request.status, Some(Ok(Status::Completed)));
    assert_eq!(view_request.phones, vec!["+38096███████".to_string()]);
    assert_eq!(
        view_request.assigned_for,
        Some(Ok(NaiveDate::from_ymd_opt(2024, 5, 16).unwrap()))
    );
}

#[test]
fn comments() {
    let html = fs::read_to_string("src/tests/resources/valid/view_request_1.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert_eq!(view_request.comments, vec![
        Ok(FullComment {
            text:            Some("Полностью заменил кабель   поменял порт".to_string()),
            user:            "████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 14:17:49+03:00").unwrap(),
            internal_status: InternalStatus::Completed,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 14:09:28+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            Some("16,05".to_string()),
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 14:07:42+03:00").unwrap(),
            internal_status: InternalStatus::Agreed,
        }),
        Ok(FullComment {
            text:            Some("Звонить на  38098███████. Договорился с абонентом на 16.05".to_string()),
            user:            "████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-15 14:03:52+03:00").unwrap(),
            internal_status: InternalStatus::Returned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "██████ █████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-14 08:36:32+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "██████ █████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-14 08:35:24+03:00").unwrap(),
            internal_status: InternalStatus::Agreed,
        }),
        Ok(FullComment {
            text:            Some("15.05".to_string()),
            user:            "████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-14 08:34:42+03:00").unwrap(),
            internal_status: InternalStatus::Returned,
        }),
        Ok(FullComment {
            text:            None,
            user:            "████████ ██████ ███████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-13 17:18:52+03:00").unwrap(),
            internal_status: InternalStatus::Assigned,
        }),
        Ok(FullComment {
            text:            Some(
                "Заявка на замену кабеля;Замена кабеля с 2 на 4 пары для Гигабита;Для можливості замовлення ТП \
                 Гігабіт;Repair date-2024-05-14;Указать метр повреждения: ; Для можливості замовлення ТП \
                 Гігабіт\n\nПросьба набрать заранее \n\n"
                    .to_string()
            ),
            user:            "Система".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-13 17:05:51+03:00").unwrap(),
            internal_status: InternalStatus::New,
        })
    ])
}
