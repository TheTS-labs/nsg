use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::data::address::Address;
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::data::mdu::{MDUError, MDU};
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;

/// Representation of parsed search entry
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize, Default)]
pub struct SearchEntry {
    pub order_id:          Option<Result<u32, SerializableIntErrorKind>>,
    pub internal_order_id: Option<Result<u32, SerializableIntErrorKind>>,
    pub phone_number:      Option<String>,
    pub address:           Option<Address>,
    pub mdu:               Option<Result<MDU, MDUError>>,
    pub client:            Option<String>,
    pub installer:         Option<String>,
    pub internal_status:   Option<Result<InternalStatus, InternalStatusError>>,
    pub last_updated:      Option<Result<NaiveDate, SerializableParseErrorKind>>,
}

/// Guarantees that all fields are ok
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize)]
pub struct GuaranteedSearchEntry {
    pub order_id:          u32,
    pub internal_order_id: u32,
    pub phone_number:      String,
    pub address:           Address,
    pub mdu:               MDU,
    pub client:            String,
    pub installer:         Option<String>,
    pub internal_status:   InternalStatus,
    pub last_updated:      NaiveDate,
}

impl SearchEntry {
    pub fn into_guaranteed(self) -> Option<GuaranteedSearchEntry> {
        Some(GuaranteedSearchEntry {
            order_id:          self.order_id?.ok()?,
            internal_order_id: self.internal_order_id?.ok()?,
            phone_number:      self.phone_number?,
            address:           self.address?,
            mdu:               self.mdu?.ok()?,
            client:            self.client?,
            installer:         self.installer,
            internal_status:   self.internal_status?.ok()?,
            last_updated:      self.last_updated?.ok()?,
        })
    }
}
