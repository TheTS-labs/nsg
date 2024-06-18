use std::fs;

use chrono::{NaiveTime, Utc};

use crate::data::address::Address;
use crate::data::full_comment::{FullComment, FullCommentError};
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::data::order_type::{OrderType, OrderTypeError};
use crate::data::status::{Status, StatusError};
use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::view_request::ViewRequest;

// TODO: Code duplication

#[test]
fn should_guarantee() {
    let html = fs::read_to_string("src/tests/assets/view_request/valid/1.html")
        .expect("Should have been able to read the file");

    let view_request = ViewRequest::from(&html);

    assert!(view_request.into_guaranteed().is_some());
}

#[test]
fn invalid_order_id() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Err(SerializableIntErrorKind::Zero)),
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_internal_order_id() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Err(SerializableIntErrorKind::Zero)),
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_order_type() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Err(OrderTypeError::InvalidOrderType("".to_string()))),
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_creation_date() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Err(SerializableParseErrorKind::Invalid)),
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_internal_status() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Err(InternalStatusError::InvalidStrStatus("".to_string()))),
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_address() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: None,
        ..ViewRequest::default()
    };
    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_client() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: None,
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_pa() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: None,
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_time_constrains() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: None,
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Err(TimeConstrainsError::FailedToParse)),
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_status() {
    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: None,
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        phones: vec!["+38067███████".to_string()],
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Err(StatusError::InvalidStatus("".to_string()))),
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_phones() {
    let view_request = ViewRequest {
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Ok(Status::Empty)),
        phones: vec![],
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_assigned_for() {
    let view_request = ViewRequest {
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Ok(Status::Empty)),
        phones: vec!["+38067███████".to_string()],
        assigned_for: None,
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Ok(Status::Empty)),
        phones: vec!["+38067███████".to_string()],
        assigned_for: Some(Err(SerializableParseErrorKind::Invalid)),
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}

#[test]
fn invalid_comments() {
    let view_request = ViewRequest {
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Ok(Status::Empty)),
        phones: vec!["+38067███████".to_string()],
        assigned_for: Some(Ok(Utc::now().date_naive())),
        comments: vec![Err(FullCommentError::DateTimeShouldBePresent)],
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());

    let view_request = ViewRequest {
        order_id: Some(Ok(1)),
        internal_order_id: Some(Ok(1)),
        order_type: Some(Ok(OrderType::NetBroken)),
        creation_date: Some(Ok(Utc::now().fixed_offset())),
        internal_status: Some(Ok(InternalStatus::Assigned)),
        address: Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        }),
        client: Some("██".to_string()),
        pa: Some("██".to_string()),
        time_constrains: Some(Ok(TimeConstrains {
            from: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            to:   NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        })),
        status: Some(Ok(Status::Empty)),
        phones: vec!["+38067███████".to_string()],
        assigned_for: Some(Ok(Utc::now().date_naive())),
        comments: vec![
            Ok(FullComment {
                text:            None,
                user:            "██".to_string(),
                datetime:        Utc::now().fixed_offset(),
                internal_status: InternalStatus::Assigned,
            }),
            Err(FullCommentError::DateTimeShouldBePresent),
        ],
        ..ViewRequest::default()
    };

    assert!(view_request.into_guaranteed().is_none());
}
