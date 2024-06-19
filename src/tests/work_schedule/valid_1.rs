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
fn should_guarantee() {
    let html = fs::read_to_string("src/tests/assets/work_schedule/valid/1.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);

    for order in work_schedule.0 {
        assert!(order.into_guaranteed().is_some());
    }
}

#[test]
fn order_13367829() {
    let html = fs::read_to_string("src/tests/assets/work_schedule/valid/1.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);
    let order = work_schedule.0.first().unwrap();

    assert_eq!(order.order_index, Some(Ok(1)));
    assert_eq!(order.order_id, Some(Ok(13367829)));
    assert_eq!(order.internal_order_id, Some(Ok(952561)));
    assert_eq!(
        order.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(10, 29, 0).unwrap(),
        }))
    );
    assert_eq!(
        order.phones,
        Some(vec!["+38067███████".to_string(), "+38067███████".to_string()])
    );
    assert_eq!(order.pa, Some("00██████62".to_string()));
    assert_eq!(
        order.address,
        Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
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
    assert_eq!(order.status, Some(Ok(Status::Completed)));
    assert_eq!(order.order_type, Some(Ok(OrderType::NetBroken)));
    assert_eq!(order.client, Some("██████████ █████ █████████████".to_string()));
    assert_eq!(order.internal_status, Some(Ok(InternalStatus::Completed)));
}

#[test]
fn order_13370195() {
    let html = fs::read_to_string("src/tests/assets/work_schedule/valid/1.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);
    let order = work_schedule.0.get(1).unwrap();

    assert_eq!(order.order_index, Some(Ok(2)));
    assert_eq!(order.order_id, Some(Ok(13370195)));
    assert_eq!(order.internal_order_id, Some(Ok(952908)));
    assert_eq!(
        order.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(12, 29, 0).unwrap(),
        }))
    );
    assert_eq!(order.phones, Some(vec!["+38066███████".to_string()]));
    assert_eq!(order.pa, Some("88████████61".to_string()));
    assert_eq!(
        order.address,
        Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "█".to_string(),
        }))
    );
    assert_eq!(
        order.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        }))
    );
    assert_eq!(order.status, Some(Ok(Status::Completed)));
    assert_eq!(order.order_type, Some(Ok(OrderType::NetNewActive)));
    assert_eq!(order.client, Some("████████ ███████ ████████████".to_string()));
    assert_eq!(order.internal_status, Some(Ok(InternalStatus::Completed)));
}

#[test]
fn order_13372183() {
    let html = fs::read_to_string("src/tests/assets/work_schedule/valid/1.html")
        .expect("Should have been able to read the file");

    let work_schedule = WorkSchedule::from(&html);
    let order = work_schedule.0.get(2).unwrap();

    assert_eq!(order.order_index, Some(Ok(3)));
    assert_eq!(order.order_id, Some(Ok(13372183)));
    assert_eq!(order.internal_order_id, Some(Ok(953184)));
    assert_eq!(
        order.time_constrains,
        Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(14, 59, 0).unwrap(),
        }))
    );
    assert_eq!(order.phones, Some(vec!["+38067███████".to_string()]));
    assert_eq!(order.pa, Some("88████████70".to_string()));
    assert_eq!(
        order.address,
        Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "█".to_string(),
        }))
    );
    assert_eq!(
        order.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        }))
    );
    assert_eq!(order.status, Some(Ok(Status::Completed)));
    assert_eq!(order.order_type, Some(Ok(OrderType::NetNewPassive)));
    assert_eq!(order.client, Some("█████ ███████ ██████████".to_string()));
    assert_eq!(order.internal_status, Some(Ok(InternalStatus::Completed)));
}
