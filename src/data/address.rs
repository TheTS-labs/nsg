//! Client's living address

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum AddressError {
    /// Provided `&str` didn't match the regex and thus can't be represented as
    /// [`Address`]
    NoMatch,
}

/// Fixes `{'uk': '1'` artifacts in the [`Address::apartment`] and transforms
/// [`Address::street`] to correct some street names
///
/// ### Transformed street names
/// - "Зернова (Ленінський)" -> "Зернова"
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Address {
    pub city:      String,
    pub street:    String,
    /// **Note**: Building can use letter(s) such as `19А` along with normal
    /// buildings `42`
    pub building:  String,
    /// **Note**: Apartment can use letter(s) such as `37A` along with normal
    /// apartments `92`
    pub apartment: String,
}

impl Address {
    /// Creates new [`Address`] from parts. `street` and `apartment` will be
    /// treated  the same way as in [`Address::from_work_schedule`]
    ///
    /// ```
    /// use nsg::data::address::Address;
    ///
    /// assert_eq!(
    ///     Address::from_parts(
    ///         "Запоріжжя".to_string(),
    ///         "вулиця Зернова (Ленінський)".to_string(),
    ///         "19А".to_string(),
    ///         "{'uk': '0'".to_string()
    ///     ),
    ///     Address {
    ///         city:      "Запоріжжя".to_string(),
    ///         street:    "Зернова".to_string(),
    ///         building:  "19А".to_string(),
    ///         apartment: "0".to_string(),
    ///     }
    /// );
    /// ```
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

    /// Creates new [`Address`] from work schedule string. Will fix apartment
    /// and street name as described for [`Address`]
    ///
    /// ```
    /// use nsg::data::address::Address;
    ///
    /// assert_eq!(
    ///     Address::from_work_schedule("Запоріжжя, вулиця Зернова (Ленінський) д.19А, кв.{'uk': '0'"),
    ///     Ok(Address {
    ///         city:      "Запоріжжя".to_string(),
    ///         street:    "Зернова".to_string(),
    ///         building:  "19А".to_string(),
    ///         apartment: "0".to_string(),
    ///     })
    /// );
    /// ```
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
