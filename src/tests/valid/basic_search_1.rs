use std::fs;

use chrono::NaiveDate;

use crate::basic_search::BasicSearch;
use crate::data::address::Address;
use crate::data::internal_status::InternalStatus;
use crate::data::mdu::MDU;

#[test]
fn parses_exactly_one_search_entry() {
    let html = fs::read_to_string("src/tests/resources/valid/basic_search_1.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);

    assert_eq!(basic_search.0.len(), 1);
}

#[test]
fn entry_13354143() {
    let html = fs::read_to_string("src/tests/resources/valid/basic_search_1.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.first().unwrap();

    assert_eq!(entry.order_id, Some(Ok(13354143)));
    assert_eq!(entry.internal_order_id, Some(Ok(950599)));
    assert_eq!(entry.phone_number, Some("+38067███████".to_string()));
    assert_eq!(
        entry.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Професора Толока".to_string(),
            building:  "██".to_string(),
            apartment: "███".to_string(),
        })
    );
    assert_eq!(
        entry.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        }))
    );
    assert_eq!(entry.client, Some("█████████ ████████".to_string()));
    assert_eq!(entry.installer, Some("████ █████ █████████".to_string()));
    assert_eq!(entry.internal_status, Some(Ok(InternalStatus::NotActivated)));
    assert_eq!(
        entry.last_updated,
        Some(Ok(NaiveDate::from_ymd_opt(2024, 5, 16).unwrap()))
    );
}
