use std::fmt;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum AddressError {
    NoMatch,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Address {
    pub city:      String,
    pub street:    String,
    pub building:  String,
    pub apartment: String,
}

impl Address {
    pub fn from_parts(city: String, street: String, building: String, apartment: String) -> Address {
        Address {
            city,
            street: Address::transform_street(&street),
            building,
            apartment: apartment.replace(['u', 'k', '{', ':', '\'', ' '], ""),
        }
    }

    fn transform_street(street: &str) -> String {
        street
            .replace("  ", "")
            .replace("вулиця ", "")
            .replace("Зернова (Ленінський)", "Зернова")
    }

    pub fn from_work_schedule(s: &str) -> Result<Address, AddressError> {
        let pattern = Regex::new(r"(?P<city>.+),\s+(?P<street>.+) д\.(?P<building>\S+), кв\.(\{'uk': '(?P<failover_apartment>\S+)'|(?P<apartment>\S+))").unwrap();
        let captures = pattern.captures(s).ok_or(AddressError::NoMatch)?;

        Ok(Address {
            city:      captures["city"].to_string(),
            street:    Address::transform_street(&captures["street"]),
            building:  captures["building"].to_string(),
            apartment: captures
                .name("apartment")
                .map(|capture| capture.as_str().to_string())
                .unwrap_or_else(|| captures["failover_apartment"].to_string()),
        })
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} {}, кв. {}",
            self.city, self.street, self.building, self.apartment
        )
    }
}
