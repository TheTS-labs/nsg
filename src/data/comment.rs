use chrono::{DateTime, FixedOffset, NaiveDateTime};
use chrono_tz::Europe::Kyiv;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::serializable_parse_error_kind::SerializableParseErrorKind;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum CommentError {
    DateTimeFailed(String, SerializableParseErrorKind),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Comment {
    pub text:     String,
    pub user:     Option<String>,
    pub datetime: Option<Result<DateTime<FixedOffset>, SerializableParseErrorKind>>,
}

impl Comment {
    pub fn from(raw_text: &str) -> Result<Comment, CommentError> {
        let text = html_escape::decode_html_entities(raw_text);
        let text = text.to_string().replace("<br/>", "\n");

        let regex =
            Regex::new(r"(?m)(\((?<user>Система|\S+ \S+ \S+),(?<datetime>\d\d\d\d-\d\d-\d\d \d\d:\d\d:\d\d))\)")
                .unwrap();

        if let Some(caps) = regex.captures(&text) {
            let text = html_escape::decode_html_entities(regex.replace(&text, "").as_ref()).replace("<br/>", "\n");
            let user = Some(caps["user"].to_string());
            let datetime = Some(Self::as_datetime(&caps["datetime"]));

            return Ok(Comment { text, user, datetime });
        };

        Ok(Comment {
            text:     html_escape::decode_html_entities(&text).replace("<br/>", "\n"),
            user:     None,
            datetime: None,
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
