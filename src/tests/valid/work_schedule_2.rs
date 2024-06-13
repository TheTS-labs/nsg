use std::fs;

use chrono::NaiveTime;

use crate::data::address::Address;
use crate::data::internal_status::InternalStatus;
use crate::data::mdu::MDU;
use crate::data::order_type::OrderType;
use crate::data::status::Status;
use crate::data::time_constrains::TimeConstrains;
use crate::work_schedule::WorkSchedule;

#[test]
fn parses_exactly_three_orders() {
    let html = fs::read_to_string("src/tests/resources/valid/work_schedule_2.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);

    assert_eq!(work_schedule.0.len(), 2);
}

#[test]
fn order_13372027() {
    let html = fs::read_to_string("src/tests/resources/valid/work_schedule_2.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);
    let order = work_schedule.0.first().unwrap();

    assert_eq!(order.order_index, Some(Ok(1)));
    assert_eq!(order.order_id, Some(Ok(13372027)));
    assert_eq!(order.internal_order_id, Some(Ok(953155)));
    assert_eq!(
        order.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(10, 29, 0).unwrap(),
        }))
    );
    assert_eq!(order.phones, Some(vec!["+38098███████".to_string()]));
    assert_eq!(order.pa, Some("88████████80".to_string()));
    assert_eq!(
        order.address,
        Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Товариська".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }))
    );
    assert_eq!(
        order.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        }))
    );
    assert_eq!(order.status, Some(Ok(Status::InWork)));
    assert_eq!(order.order_type, Some(Ok(OrderType::NetNewPassive)));
    assert_eq!(order.client, Some("█████████ ███████ ██████████".to_string()));
    assert_eq!(order.internal_status, Some(Ok(InternalStatus::Assigned)));
}

#[test]
fn order_13373090() {
    let html = fs::read_to_string("src/tests/resources/valid/work_schedule_2.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);
    let order = work_schedule.0.get(1).unwrap();

    assert_eq!(order.order_index, Some(Ok(2)));
    assert_eq!(order.order_id, Some(Ok(13373090)));
    assert_eq!(order.internal_order_id, Some(Ok(953314)));
    assert_eq!(
        order.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(15, 29, 0).unwrap(),
        }))
    );
    assert_eq!(
        order.phones,
        Some(vec!["+38068███████".to_string(), "+38068███████".to_string()])
    );
    assert_eq!(order.pa, Some("82███59".to_string()));
    assert_eq!(
        order.address,
        Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Товариська".to_string(),
            building:  "███".to_string(),
            apartment: "██".to_string(),
        }))
    );
    assert_eq!(
        order.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        }))
    );
    assert_eq!(order.status, Some(Ok(Status::Assigned)));
    assert_eq!(order.order_type, Some(Ok(OrderType::NetBroken)));
    assert_eq!(order.client, Some("█████████ ███████ ████████".to_string()));
    assert_eq!(order.internal_status, Some(Ok(InternalStatus::Assigned)));
}
