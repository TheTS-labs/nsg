//! MDU (multiple dwelling unit or a specific set of houses)
//!
//! ## Example usage
//!
//! ```
//! use nsg::data::mdu::MDU;
//!
//! assert_eq!(
//!     MDU::from_work_schedule("MDU_ZAP00029"),
//!     Ok(MDU {
//!         city_code: "ZAP".to_string(),
//!         number:    29,
//!     })
//! );
//! assert_eq!(
//!     MDU::from_work_schedule("MDU_ZAP61"),
//!     Ok(MDU {
//!         city_code: "ZAP".to_string(),
//!         number:    61,
//!     })
//! );
//! assert_eq!(
//!     MDU::from_work_schedule("MDU_KIE20"),
//!     Ok(MDU {
//!         city_code: "KIE".to_string(),
//!         number:    20,
//!     })
//! );
//! ```

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum MDUError {
    /// Provided `&str` didn't match the regex and thus can't be represented as
    /// [`MDU`]
    NoMatch,
    /// [`MDU`]'s number can't be parsed as `u32`
    FailedToParseNumber,
}

/// MDU is multiple dwelling unit or basically a specific set of houses
/// located near each other and forming a service area
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct MDU {
    /// Code of the city in which the MDU is located like ZAP or KIE
    pub city_code: String,
    /// Number of the MDU
    pub number:    u32,
}

impl MDU {
    /// Will create [MDU] instance from `&str` in format
    /// `MDU_<city_code><number>`. For example, `MDU_ZAP00029`,
    /// `MDU_ZAP00056` and so on.
    ///
    /// Note that zeros aren't necessary: `MDU_ZAP00029` is the same as
    /// `MDU_ZAP29`. Because simplified regex for matching `MDU` is
    /// `MDU_([a-zA-Z]+)(\d+)`
    pub fn from_work_schedule(text: &str) -> Result<MDU, MDUError> {
        let re = r"(?m)MDU_(?<city_code>[a-zA-Z]+)(?<number>\d+)".to_string();
        let regex = Regex::new(&re).unwrap();
        let captures = regex.captures(text).ok_or(MDUError::NoMatch)?;

        Ok(MDU {
            city_code: captures["city_code"].to_string(),
            // TODO: FailedToParseNumber should take the error
            number:    captures["number"].parse().map_err(|_| MDUError::FailedToParseNumber)?,
        })
    }
}
