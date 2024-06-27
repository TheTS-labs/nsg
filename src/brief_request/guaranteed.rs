//! Guaranteed brief request

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::BriefRequest;
use crate::data::address::Address;
use crate::data::comment::Comment;
use crate::data::internal_status::InternalStatus;
use crate::data::order_type::OrderType;
use crate::data::status::Status;
use crate::data::time_constrains::TimeConstrains;

/// Hence [`BriefRequest`] will not fail hard, it's not necessary valid. You can
/// guarantee validness of brief request with [`BriefRequest::into_guaranteed`].
/// For detailed information about field refer to it's documentation or
/// [`BriefRequest`]'s documentation
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct GuaranteedBriefRequest {
    pub order_id:          u32,
    pub internal_order_id: u32,
    pub order_type:        OrderType,
    pub creation_date:     DateTime<FixedOffset>,
    pub internal_status:   InternalStatus,
    pub address:           Address,
    pub client:            String,
    pub service:           Option<String>,
    pub pa:                String,
    pub time_constrains:   TimeConstrains,
    pub installers:        Vec<String>,
    pub last_comment:      Option<Comment>,
    pub first_comment:     Option<Comment>,
    pub status:            Status,
    /// Will contain at least one element
    pub phones:            Vec<String>,
}

impl BriefRequest {
    pub fn into_guaranteed(self) -> Option<GuaranteedBriefRequest> {
        if self.phones.is_empty() {
            return None;
        }

        Some(GuaranteedBriefRequest {
            order_id:          self.order_id?.ok()?,
            internal_order_id: self.internal_order_id?.ok()?,
            order_type:        self.order_type?.ok()?,
            creation_date:     self.creation_date?.ok()?,
            internal_status:   self.internal_status?.ok()?,
            address:           self.address?,
            client:            self.client?,
            service:           self.service,
            pa:                self.pa?,
            time_constrains:   self.time_constrains?.ok()?,
            installers:        self.installers,
            last_comment:      Some(self.last_comment?.ok()?),
            first_comment:     Some(self.first_comment?.ok()?),
            status:            self.status?.ok()?,
            phones:            self.phones,
        })
    }
}
