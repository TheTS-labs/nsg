//! View request parser
//!
//! ## Example usage
//! You can find example HTMLs in `src/tests/assets/view_request/valid`
//!
//! ```
//! use nsg::view_request::ViewRequest;
//!
//! let html = include_str!("../tests/assets/view_request/valid/1.html");
//! let view_request = ViewRequest::from(&html);
//!
//! println!("Order (view request): {:#?}", view_request);
//! ```

mod comments;
pub mod guaranteed;

use std::fmt::Debug;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Europe::Kyiv;
use itertools::Itertools;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

use crate::data::address::Address;
use crate::data::full_comment::{FullComment, FullCommentError};
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::data::order_type::{OrderType, OrderTypeError};
use crate::data::status::{Status, StatusError};
use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};
use crate::macros::match_and_set;
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::traits::is_it::IsIt;
use crate::traits::prev_element_ref::PrevElementRef;

/// Parsed view request containing full order information and it's detailed
/// comments. Note that all fields will not fail hard allowing to work with
/// partially valid order
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize, Default)]
pub struct ViewRequest {
    pub order_id:          Option<Result<u32, SerializableIntErrorKind>>,
    pub internal_order_id: Option<Result<u32, SerializableIntErrorKind>>,
    // TODO: Use "Подтип" to determine active or passive order type
    /// Currently [`OrderType`] in view request is limited to
    /// [`OrderType::NetNewUnknown`], [`OrderType::NetRecoveryUnknown`] and
    /// [`OrderType::NetRelocationUnknown`] instead of their active or passive
    /// variants. Other types are not affected
    pub order_type:        Option<Result<OrderType, OrderTypeError>>,
    // TODO: Rename to creation_datetime
    pub creation_date:     Option<Result<DateTime<FixedOffset>, SerializableParseErrorKind>>,
    pub internal_status:   Option<Result<InternalStatus, InternalStatusError>>,
    pub address:           Option<Address>,
    /// Client's full name (Kyivstar's version)
    pub client:            Option<String>,
    /// Only orders for subscription (connection) to Kyivstar's network contain
    /// service package name
    pub service:           Option<String>,
    /// Client's personal account number
    pub pa:                Option<String>,
    // TODO: Parse also "Канал подачи заявки", "Код продавца" and "Продавец"
    // TODO: Example at src/tests/assets/view_request/valid/4.html
    /// Name or phone number of person who created order
    pub seller:            Option<String>,
    pub time_constrains:   Option<Result<TimeConstrains, TimeConstrainsError>>,
    /// One order can have up to two installers
    pub installers:        Vec<String>,
    pub status:            Option<Result<Status, StatusError>>,
    // TODO: View request gives only one of possibly two phones
    /// List of client's contact phone numbers
    pub phones:            Vec<String>,
    /// Date on which the order is scheduled
    pub assigned_for:      Option<Result<NaiveDate, SerializableParseErrorKind>>,
    pub comments:          Vec<Result<FullComment, FullCommentError>>,
}

impl PrevElementRef for ViewRequest {}

impl ViewRequest {
    /// Parse view request from HTML
    pub fn from(html: &str) -> ViewRequest {
        log::debug!(target: "nsg", "Processing HTML: {:?}", html);
        let mut view_request = ViewRequest::default();

        let html_fragment = Html::parse_fragment(html);
        view_request.collect_fragments(&html_fragment);

        view_request
    }

    fn collect_fragments(&mut self, html_fragment: &Html) {
        log::info!(target: "nsg", "Parsing brief request HTML...");

        let elements = html_fragment
            .root_element()
            .descendent_elements()
            .filter_map(|element| {
                if element.child_elements().count() == 0 {
                    return Some(element);
                }

                if element.child_elements().count() == 1 && element.child_elements().next()?.value().name() == "a" {
                    return Some(element);
                }

                None
            });
        log::debug!(target: "nsg", "Defined iterator over elements without child elements");

        let text_getter = |element: ElementRef<'_>| -> String { element.text().collect_vec().join(" ") };
        log::debug!(target: "nsg", "Defined text getter");

        let mut city = None;
        let mut address_string = None;
        let mut apartment = None;

        for element in elements {
            log::debug!(target: "nsg", "Processing {:?}...", element.id());

            if element.value().name() == "h3" {
                let cur_text = text_getter(element);

                if cur_text.contains("Заявка №") {
                    log::info!(
                        target: "nsg",
                        "[{:?}] Found self.internal_order_id, inner text: {:?}",
                        element.id(),
                        &cur_text
                    );
                    self.internal_order_id =
                        Some(u32::from_str(&cur_text.replace("Заявка №", "")).map_err(|err| err.kind().clone().into()));
                    continue;
                }
            }

            match_and_set!(
                "наряд:",
                self.order_id,
                (|| Some(u32::from_str(&cur_text).map_err(|err| err.kind().clone().into()))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "подтип:",
                self.order_type,
                (|| Some(OrderType::from_str(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "дата создания:",
                self.creation_date,
                (|| Some(self.as_datetime(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "статус:",
                self.internal_status,
                (|| Some(InternalStatus::from_str(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "город:",
                city,
                (|| Some(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "адрес:",
                address_string,
                (|| Some(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "квартира:",
                apartment,
                (|| Some(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "клиент:",
                self.client,
                (|| Some(cur_text.trim().to_string())),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "пакет:",
                self.service,
                (|| if cur_text.is_empty() { None } else { Some(cur_text) }),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "телефон:",
                (|| self.phones.push(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "телефон 2:",
                (|| self.phones.push(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "лицевой счет:",
                self.pa,
                (|| Some(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "телефон продавца:",
                self.seller,
                (|| Some(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "время подключения:",
                self.time_constrains,
                (|| Some(TimeConstrains::from(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "монтажник:",
                (|| self.installers.push(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "монтажник 2:",
                (|| self.installers.push(cur_text)),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "статус у заказчика:",
                self.status,
                (|| Some(Status::from_str(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "дата подключения:",
                self.assigned_for,
                (|| Some(NaiveDate::parse_from_str(&cur_text, "%d.%m.%Y").map_err(|err| err.kind().into()))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );

            log::debug!(
                target: "nsg",
                "[{:?}] Was not a fragment of brief request data, inner html: {:?}",
                element.id(),
                element.html()
            );
        }

        self.set_comments(html_fragment);

        if let (Some(city), Some(address_string), Some(apartment)) = (city, address_string, apartment) {
            let mut address_iter = address_string.split(',');

            self.address = Some(Address::from_parts(
                city,
                address_iter.next().unwrap().to_string(),
                address_iter.next().unwrap().to_string(),
                apartment,
            ));
        }
    }

    fn as_datetime(&self, input: &str) -> Result<DateTime<FixedOffset>, SerializableParseErrorKind> {
        let naive = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")?;
        let datetime = naive
            .and_local_timezone(Kyiv)
            .earliest()
            .expect("Never should have gotten a time that doesn't exist in the Kyiv time zone")
            .fixed_offset();

        Ok(datetime)
    }
}
