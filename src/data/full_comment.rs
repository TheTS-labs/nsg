use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime};
use chrono_tz::Europe::Kyiv;
use itertools::Itertools;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use super::internal_status::{InternalStatus, InternalStatusError};
use crate::serializable_parse_error_kind::SerializableParseErrorKind;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum FullCommentError {
    DateTimeShouldBePresent,
    UserShouldBePresent,
    InternalStatusShouldBePresent,
    DateTimeFailed(String, SerializableParseErrorKind),
    InvalidInternalStatus(InternalStatusError),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct FullComment {
    pub text:            Option<String>,
    pub user:            String,
    pub datetime:        DateTime<FixedOffset>,
    pub internal_status: InternalStatus,
}

impl FullComment {
    pub fn from(comment: ElementRef) -> Result<FullComment, FullCommentError> {
        log::debug!(target: "nsg", "Creating full comment from {}", comment.html());
        let data = comment.child_elements().map(|x| x.text().nth(0)).collect_vec();

        let raw_datetime = data[0].ok_or(FullCommentError::DateTimeShouldBePresent)?;
        let user = data[2].ok_or(FullCommentError::UserShouldBePresent)?;
        let raw_internal_status = data[4].ok_or(FullCommentError::InternalStatusShouldBePresent)?;
        let text = data[5].map(|x| html_escape::decode_html_entities(x).replace("<br/>", "\n"));

        let datetime = Self::as_datetime(raw_datetime)
            .map_err(|err| FullCommentError::DateTimeFailed(raw_datetime.to_string(), err))?;
        let internal_status =
            InternalStatus::from_str(raw_internal_status).map_err(FullCommentError::InvalidInternalStatus)?;

        Ok(FullComment {
            datetime,
            text,
            user: user.to_string(),
            internal_status,
        })
    }

    fn as_datetime(input: &str) -> Result<DateTime<FixedOffset>, SerializableParseErrorKind> {
        let naive = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")?;
        let datetime = naive
            .and_local_timezone(Kyiv)
            .earliest()
            .expect("Never should have gotten a time that doesn't exist in the Kyiv time zone")
            .fixed_offset();

        Ok(datetime)
    }
}
