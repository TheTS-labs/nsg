use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum MDUError {
    NoMatch,
    FailedToParseNumber,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct MDU {
    pub city_code: String,
    pub number:    u32,
}

impl MDU {
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
