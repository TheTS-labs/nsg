use std::fs;

use chrono::NaiveDate;

use crate::basic_search::BasicSearch;
use crate::data::address::Address;
use crate::data::internal_status::InternalStatus;
use crate::data::mdu::MDU;

#[test]
fn should_guarantee() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);

    for search_entry in basic_search.0 {
        assert!(search_entry.into_guaranteed().is_some());
    }
}

#[test]
fn parses_exactly_five_search_entries() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);

    assert_eq!(basic_search.0.len(), 5);
}

#[test]
fn entry_13354143() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
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

#[test]
fn entry_11417701() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.get(1).unwrap();

    assert_eq!(entry.order_id, Some(Ok(11417701)));
    assert_eq!(entry.internal_order_id, Some(Ok(673213)));
    assert_eq!(entry.phone_number, Some("095███████".to_string()));
    assert_eq!(
        entry.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Комарова".to_string(),
            building:  "██".to_string(),
            apartment: "█".to_string(),
        })
    );
    assert_eq!(
        entry.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    57,
        }))
    );
    assert_eq!(entry.client, Some("████████ ██████ █████████".to_string()));
    assert_eq!(entry.installer, None);
    assert_eq!(entry.internal_status, Some(Ok(InternalStatus::Rejected)));
    assert_eq!(
        entry.last_updated,
        Some(Ok(NaiveDate::from_ymd_opt(2021, 10, 24).unwrap()))
    );
}

#[test]
fn entry_11180291() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.get(2).unwrap();

    assert_eq!(entry.order_id, Some(Ok(11180291)));
    assert_eq!(entry.internal_order_id, Some(Ok(640797)));
    assert_eq!(entry.phone_number, Some("+380 95███████".to_string()));
    assert_eq!(
        entry.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Північнокільцева".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(
        entry.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    56,
        }))
    );
    assert_eq!(entry.client, Some("██████ ████████ █████████".to_string()));
    assert_eq!(entry.installer, None);
    assert_eq!(entry.internal_status, Some(Ok(InternalStatus::Rejected)));
    assert_eq!(
        entry.last_updated,
        Some(Ok(NaiveDate::from_ymd_opt(2021, 7, 20).unwrap()))
    );
}

#[test]
fn entry_10842659() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.get(3).unwrap();

    assert_eq!(entry.order_id, Some(Ok(10842659)));
    assert_eq!(entry.internal_order_id, Some(Ok(596125)));
    assert_eq!(entry.phone_number, Some("095███████".to_string()));
    assert_eq!(
        entry.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Бочарова".to_string(),
            building:  "███".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(
        entry.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    49,
        }))
    );
    assert_eq!(entry.client, Some("█████ ██████ ██████████".to_string()));
    assert_eq!(entry.installer, None);
    assert_eq!(entry.internal_status, Some(Ok(InternalStatus::Rejected)));
    assert_eq!(
        entry.last_updated,
        Some(Ok(NaiveDate::from_ymd_opt(2021, 2, 27).unwrap()))
    );
}

#[test]
fn entry_10842472() {
    let html = fs::read_to_string("src/tests/assets/basic_search/valid/2.html")
        .expect("Should have been able to read the file");

    let basic_search = BasicSearch::from(&html);
    let entry = basic_search.0.get(4).unwrap();

    assert_eq!(entry.order_id, Some(Ok(10842472)));
    assert_eq!(entry.internal_order_id, Some(Ok(596097)));
    assert_eq!(entry.phone_number, Some("098███████".to_string()));
    assert_eq!(
        entry.address,
        Some(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Лахтинська".to_string(),
            building:  "█".to_string(),
            apartment: "██".to_string(),
        })
    );
    assert_eq!(
        entry.mdu,
        Some(Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    43,
        }))
    );
    assert_eq!(entry.client, Some("████████ ███████ ███████████".to_string()));
    assert_eq!(entry.installer, Some("██████ ████████ █████████".to_string()));
    assert_eq!(entry.internal_status, Some(Ok(InternalStatus::Completed)));
    assert_eq!(
        entry.last_updated,
        Some(Ok(NaiveDate::from_ymd_opt(2021, 2, 25).unwrap()))
    );
}
