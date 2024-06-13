use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

use super::ViewRequest;
use crate::data::address::Address;
use crate::data::full_comment::FullComment;
use crate::data::internal_status::InternalStatus;
use crate::data::order_type::OrderType;
use crate::data::status::Status;
use crate::data::time_constrains::TimeConstrains;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct GuaranteedViewRequest {
    pub order_id:          u32,
    pub internal_order_id: u32,
    pub order_type:        OrderType,
    pub creation_date:     DateTime<FixedOffset>,
    pub internal_status:   InternalStatus,
    pub address:           Address,
    pub client:            String,
    pub service:           Option<String>,
    pub pa:                String,
    pub seller:            Option<String>,
    pub time_constrains:   TimeConstrains,
    pub installers:        Vec<String>,
    pub status:            Status,
    pub phones:            Vec<String>,
    pub assigned_for:      NaiveDate,
    pub comments:          Vec<FullComment>,
}

impl ViewRequest {
    pub fn into_guaranteed(self) -> Option<GuaranteedViewRequest> {
        if self.phones.is_empty() {
            return None;
        }

        let mut comments = Vec::with_capacity(self.comments.len());
        for comment in self.comments {
            comments.push(comment.ok()?);
        }

        Some(GuaranteedViewRequest {
            order_id: self.order_id?.ok()?,
            internal_order_id: self.internal_order_id?.ok()?,
            order_type: self.order_type?.ok()?,
            creation_date: self.creation_date?.ok()?,
            internal_status: self.internal_status?.ok()?,
            address: self.address?,
            client: self.client?,
            service: self.service,
            pa: self.pa?,
            seller: self.seller,
            time_constrains: self.time_constrains?.ok()?,
            installers: self.installers,
            status: self.status?.ok()?,
            phones: self.phones,
            assigned_for: self.assigned_for?.ok()?,
            comments,
        })
    }
}
