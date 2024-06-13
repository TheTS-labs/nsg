pub mod guaranteed;

use std::fmt::Debug;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime};
use chrono_tz::Europe::Kyiv;
use itertools::Itertools;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

use crate::data::address::Address;
use crate::data::comment::{Comment, CommentError};
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::data::order_type::{OrderType, OrderTypeError};
use crate::data::status::{Status, StatusError};
use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};
use crate::macros::match_and_set;
use crate::serializable_int_error_kind::SerializableIntErrorKind;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;
use crate::traits::is_it::IsIt;
use crate::traits::prev_element_ref::PrevElementRef;

/// Representation of parsed brief request
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize, Default)]
pub struct BriefRequest {
    pub order_id:          Option<Result<u32, SerializableIntErrorKind>>,
    pub internal_order_id: Option<Result<u32, SerializableIntErrorKind>>,
    pub order_type:        Option<Result<OrderType, OrderTypeError>>,
    pub creation_date:     Option<Result<DateTime<FixedOffset>, SerializableParseErrorKind>>,
    pub internal_status:   Option<Result<InternalStatus, InternalStatusError>>,
    pub address:           Option<Address>,
    pub client:            Option<String>,
    pub service:           Option<String>,
    pub pa:                Option<String>,
    pub time_constrains:   Option<Result<TimeConstrains, TimeConstrainsError>>,
    pub installers:        Vec<String>,
    pub last_comment:      Option<Result<Comment, CommentError>>,
    pub first_comment:     Option<Result<Comment, CommentError>>,
    pub status:            Option<Result<Status, StatusError>>,
    pub phones:            Vec<String>,
}

impl PrevElementRef for BriefRequest {}

impl BriefRequest {
    /// Parse brief request from html
    pub fn from(html: &str) -> BriefRequest {
        log::debug!(target: "nsg", "Processing HTML: {:?}", html);
        let mut brief_request = BriefRequest::default();

        let html_fragment = Html::parse_fragment(html);
        brief_request.collect_fragments(&html_fragment);

        brief_request
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
            match_and_set!(
                "заявка:",
                self.internal_order_id,
                (|| Some(u32::from_str(&cur_text).map_err(|err| err.kind().clone().into()))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );

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
                (|| {
                    if cur_text.is_empty() {
                        log::warn!(
                            target: "nsg",
                            "[{:?}] NOTE: intentionally ignoring self.service because it's empty",
                            element.id()
                        );
                        return None;
                    }

                    Some(cur_text)
                }),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "телефон:",
                (|| self.phones.push(cur_text.trim().to_string())),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
            match_and_set!(
                "телефон 2:",
                (|| self.phones.push(cur_text.trim().to_string())),
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
                "статус заказчика:",
                self.status,
                (|| Some(Status::from_str(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );

            match_and_set!(
                "начальный комментарий:",
                self.first_comment,
                (|| Some(Comment::from(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );

            match_and_set!(
                "последний комментарий:",
                self.last_comment,
                (|| Some(Comment::from(&cur_text))),
                self,
                element,
                text_getter,
                is_it,
                cur_text
            );
        }

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
