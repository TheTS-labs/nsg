//! Time period for which order is scheduled
//!
//! ## Example usage
//!
//! ```
//! use chrono::NaiveTime;
//! use nsg::data::time_constrains::TimeConstrains;
//!
//! assert_eq!(
//!     TimeConstrains::from("с 10:00 до 10:29"),
//!     Ok(TimeConstrains {
//!         from: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
//!         to:   NaiveTime::from_hms_opt(10, 29, 0).unwrap(),
//!     })
//! );
//! assert_eq!(
//!     TimeConstrains::from_work_schedule("10:00", "10:29"),
//!     Ok(TimeConstrains {
//!         from: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
//!         to:   NaiveTime::from_hms_opt(10, 29, 0).unwrap(),
//!     })
//! );
//! ```

use chrono::NaiveTime;
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum TimeConstrainsError {
    // TODO: improve errors
    /// Provided `&str` didn't match the regex and thus can't be represented as
    /// [`TimeConstrains`]
    NoMatch,
    /// Failed to parse some time component as `u32`
    FailedToParse,
    /// Failed to create [`NaiveTime`]
    InvalidNaiveTime,
    /// [`TimeConstrains::from_work_schedule`] one of components has no hour
    /// element
    NoHourElement,
    /// [`TimeConstrains::from_work_schedule`] one of components has no minute
    /// element
    NoMinuteElement,
}

/// Time period for which order is scheduled
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct TimeConstrains {
    pub from: NaiveTime,
    pub to:   NaiveTime,
}

impl TimeConstrains {
    /// Will create [`TimeConstrains`] from string in
    /// `с (\d\d):(\d\d) до (\d\d):(\d\d)` format
    pub fn from(text: &str) -> Result<TimeConstrains, TimeConstrainsError> {
        let regex = Regex::new(r"(?m)с (?<from_hour>\d\d):(?<from_minute>\d\d) до (?<to_hour>\d\d):(?<to_minute>\d\d)")
            .unwrap();
        let captures = regex.captures(text).ok_or(TimeConstrainsError::NoMatch)?;

        let from_hour = captures["from_hour"]
            .parse::<u32>()
            // TODO: Impossible to fail
            .map_err(|_| TimeConstrainsError::FailedToParse)?;
        let from_minute = captures["from_minute"]
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;
        let to_hour = captures["to_hour"]
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;
        let to_minute = captures["to_minute"]
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;

        let from = NaiveTime::from_hms_opt(from_hour, from_minute, 0).ok_or(TimeConstrainsError::InvalidNaiveTime)?;
        let to = NaiveTime::from_hms_opt(to_hour, to_minute, 0).ok_or(TimeConstrainsError::InvalidNaiveTime)?;

        Ok(TimeConstrains { from, to })
    }

    /// Will create [`TimeConstrains`] from `from` and  `to` components
    pub fn from_work_schedule(raw_from: &str, raw_to: &str) -> Result<TimeConstrains, TimeConstrainsError> {
        let from = raw_from.split(':').collect_vec();
        let to = raw_to.split(':').collect_vec();

        let from_hour = from.first().ok_or(TimeConstrainsError::NoHourElement)?;
        let from_minute = from.get(1).ok_or(TimeConstrainsError::NoMinuteElement)?;
        let to_hour = to.first().ok_or(TimeConstrainsError::NoHourElement)?;
        let to_minute = to.get(1).ok_or(TimeConstrainsError::NoMinuteElement)?;

        let from_hour = from_hour
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;
        let from_minute = from_minute
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;
        let to_hour = to_hour.parse::<u32>().map_err(|_| TimeConstrainsError::FailedToParse)?;
        let to_minute = to_minute
            .parse::<u32>()
            .map_err(|_| TimeConstrainsError::FailedToParse)?;

        let from = NaiveTime::from_hms_opt(from_hour, from_minute, 0).ok_or(TimeConstrainsError::InvalidNaiveTime)?;
        let to = NaiveTime::from_hms_opt(to_hour, to_minute, 0).ok_or(TimeConstrainsError::InvalidNaiveTime)?;

        Ok(TimeConstrains { from, to })
    }
}
