use crate::data::address::{Address, AddressError};

#[test]
fn should_match() {
    assert_eq!(
        Address::from_work_schedule("Запоріжжя, вулиця Зернова (Ленінський) д.██, кв.██"),
        Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Зернова".to_string(),
            building:  "██".to_string(),
            apartment: "██".to_string(),
        })
    );

    assert_eq!(
        Address::from_work_schedule("Запоріжжя,  Ладозька д.██, кв.{'uk': '█'"),
        Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "█".to_string(),
        })
    );

    assert_eq!(
        Address::from_work_schedule("Запоріжжя, вулиця Ладозька д.██, кв.█"),
        Ok(Address {
            city:      "Запоріжжя".to_string(),
            street:    "Ладозька".to_string(),
            building:  "██".to_string(),
            apartment: "█".to_string(),
        })
    );
}

#[test]
fn should_not_match() {
    assert_eq!(Address::from_work_schedule("???"), Err(AddressError::NoMatch));
}

#[test]
fn should_create_address() {
    assert_eq!(
        Address::from_parts(
            "City".to_string(),
            "Street".to_string(),
            "Building".to_string(),
            "Apartment".to_string()
        ),
        Address {
            city:      "City".to_string(),
            street:    "Street".to_string(),
            building:  "Building".to_string(),
            apartment: "Apartment".to_string(),
        }
    );
}
