//! Comment with all available data

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
    /// Can't find datetime in provided [`ElementRef`]
    DateTimeShouldBePresent,
    /// Can't find user in provided [`ElementRef`]
    UserShouldBePresent,
    /// Can't find internal status in provided [`ElementRef`]
    InternalStatusShouldBePresent,
    /// Found datetime in provided [`ElementRef`], but it can't be parsed as
    /// such
    DateTimeFailed(String, SerializableParseErrorKind),
    /// Found internal status in provided [`ElementRef`], but it's invalid
    InvalidInternalStatus(InternalStatusError),
}

/// On the portal it's used to change the internal status of an order, I
/// believe, although providing additional information for other users is what
/// gives it its name
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct FullComment {
    pub text:            Option<String>,
    pub user:            String,
    pub datetime:        DateTime<FixedOffset>,
    /// Internal status of the order to which it has changed with a comment
    pub internal_status: InternalStatus,
}

impl FullComment {
    /// Parse comment from its HTML. For more HTML examples refer to
    /// `src/tests/asserts/view_request/valid`
    ///
    /// ```
    /// # include_str!("../tests/assets/view_request/valid/1.html");
    /// use nsg::data::internal_status::InternalStatus;
    /// use nsg::data::full_comment::FullComment;
    /// use scraper::{Html, Selector};
    /// use chrono::DateTime;
    ///
    /// let table_of_comments = Html::parse_fragment(
    ///     "<table>
    ///         <tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
    ///             <td>2024-05-16 14:17:49</td>
    ///             <td></td>
    ///             <td>████ █████ █████████</td>
    ///             <td>10.1.162.25</td>
    ///             <td>Выполнено</td>
    ///             <td>Полностью заменил кабель   поменял порт</td>     
    ///         </tr>
    ///     </table>",
    /// );
    ///
    /// let selector = Selector::parse("tr").unwrap();
    /// let first_comment = table_of_comments.select(&selector).next().unwrap();
    /// println!("HTML of the first comment: {}", first_comment.html());
    ///
    /// assert_eq!(
    ///     FullComment::from(first_comment),
    ///     Ok(FullComment {
    ///         text:            Some("Полностью заменил кабель   поменял порт".to_string()),
    ///         user:            "████ █████ █████████".to_string(),
    ///         datetime:        DateTime::parse_from_rfc3339("2024-05-16 14:17:49+03:00").unwrap(),
    ///         internal_status: InternalStatus::Completed,
    ///     })
    /// )
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
