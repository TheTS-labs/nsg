use std::fs;

use chrono::NaiveTime;

use crate::data::address::{Address, AddressError};
use crate::data::internal_status::InternalStatusError;
use crate::data::mdu::{MDUError, MDU};
use crate::data::order_type::{OrderType, OrderTypeError};
use crate::data::status::{Status, StatusError};
use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::work_schedule::order::Order;
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
fn invalid_order_id() {
    let order = Order {
        order_id: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Err(SerializableIntErrorKind::Zero)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_internal_order_id() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Err(SerializableIntErrorKind::Zero)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_order_index() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Err(SerializableIntErrorKind::Zero)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_time_constrains() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Err(TimeConstrainsError::NoMatch)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_phones() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_pa() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_address() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Err(AddressError::NoMatch)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_mdu() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Err(MDUError::NoMatch)),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_status() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Err(StatusError::InvalidStatus("".to_string()))),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_order_type() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Ok(Status::Assigned)),
        order_type: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Ok(Status::Assigned)),
        order_type: Some(Err(OrderTypeError::InvalidOrderType("".to_string()))),
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_client() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Ok(Status::Assigned)),
        order_type: Some(Ok(OrderType::NetBroken)),
        client: None,
        ..Order::default()
    };
    assert!(order.into_guaranteed().is_none());
}

#[test]
fn invalid_internal_status() {
    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Ok(Status::Assigned)),
        order_type: Some(Ok(OrderType::NetBroken)),
        client: Some("".to_string()),
        internal_status: None
    };
    assert!(order.into_guaranteed().is_none());

    let order = Order {
        order_id: Some(Ok(0)),
        internal_order_id: Some(Ok(0)),
        order_index: Some(Ok(0)),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        phones: Some(vec!["+38067███████".to_string()]),
        pa: Some("00██████62".to_string()),
        address: Some(Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })),
        mdu: Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })),
        status: Some(Ok(Status::Assigned)),
        order_type: Some(Ok(OrderType::NetBroken)),
        client: Some("".to_string()),
        internal_status: Some(Err(InternalStatusError::InvalidStrStatus("".to_string())))
    };
    assert!(order.into_guaranteed().is_none());
}