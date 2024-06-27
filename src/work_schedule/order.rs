//! Work schedule order
//!
//! ## Example usage
//! You can find example HTMLs in `src/tests/assets/work_schedule/valid`.
//!
//! ```
//! use nsg::work_schedule::order::Order;
//! use scraper::{Html, Selector};
//!
//! let html = include_str!("../tests/assets/work_schedule/valid/1.html");
//! let fragment = Html::parse_fragment(html);
//!
//! let request_row_selector = Selector::parse("table tr td table tr.requestrow").unwrap();
//! let request_row = fragment.select(&request_row_selector).next().unwrap();
//!
//! let order = Order::from_row_and_fragment(&request_row, &fragment);
//!
//! println!("Order: {:#?}", order);
//! ```

use std::str::FromStr;

use itertools::Itertools;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::data::address::{Address, AddressError};
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::data::mdu::{MDUError, MDU};
use crate::data::order_type::{OrderType, OrderTypeError};
use crate::data::status::{Status, StatusError};
use crate::data::time_constrains::{TimeConstrains, TimeConstrainsError};
use crate::macros::selector;
use crate::serializable_int_error_kind::SerializableIntErrorKind;

/// Parsed order from work schedule. For detailed information about field refer
/// to it's documentation. Note that all fields will not fail hard allowing to
/// work with partially valid orders
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize, Default)]
pub struct Order {
    /// Order index isn't necessary unique. For example in cases of several
    /// work schedules for the same day
    pub order_index:       Option<Result<u8, SerializableIntErrorKind>>,
    pub order_id:          Option<Result<u32, SerializableIntErrorKind>>,
    pub internal_order_id: Option<Result<u32, SerializableIntErrorKind>>,
    pub time_constrains:   Option<Result<TimeConstrains, TimeConstrainsError>>,
    /// List of client's contact phone numbers
    pub phones:            Option<Vec<String>>,
    /// Client's personal account number
    pub pa:                Option<String>,
    pub address:           Option<Result<Address, AddressError>>,
    pub mdu:               Option<Result<MDU, MDUError>>,
    pub status:            Option<Result<Status, StatusError>>,
    pub order_type:        Option<Result<OrderType, OrderTypeError>>,
    /// Client's full name (Kyivstar's version)
    pub client:            Option<String>,
    pub internal_status:   Option<Result<InternalStatus, InternalStatusError>>,
}

/// Hence [`Order`] will not fail hard, it's not necessary valid. You can
/// guarantee validness of order with [`Order::into_guaranteed`]. For detailed
/// information about field refer to it's documentation or [`Order`]'s
/// documentation
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct GuaranteedOrder {
    pub order_index:       u8,
    pub order_id:          u32,
    pub internal_order_id: u32,
    pub time_constrains:   TimeConstrains,
    pub phones:            Vec<String>,
    pub pa:                String,
    pub address:           Address,
    pub mdu:               MDU,
    pub status:            Status,
    pub order_type:        OrderType,
    pub client:            String,
    pub internal_status:   InternalStatus,
}

impl Order {
    pub fn into_guaranteed(self) -> Option<GuaranteedOrder> {
        Some(GuaranteedOrder {
            order_id:          self.order_id?.ok()?,
            internal_order_id: self.internal_order_id?.ok()?,
            order_index:       self.order_index?.ok()?,
            time_constrains:   self.time_constrains?.ok()?,
            phones:            self.phones?,
            pa:                self.pa?,
            address:           self.address?.ok()?,
            mdu:               self.mdu?.ok()?,
            status:            self.status?.ok()?,
            order_type:        self.order_type?.ok()?,
            client:            self.client?,
            internal_status:   self.internal_status?.ok()?,
        })
    }

    /// Parse order from relevant row and whole HTML table from which row is
    /// extracted
    pub fn from_row_and_fragment(row: &ElementRef<'_>, fragment: &Html) -> Self {
        let rowid = row.attr("rowid");

        Self {
            order_index:       selector!(
                get_and_text "th", next,
                row, Self::inner_text,
                (|text: String| Some(u8::from_str(&text.replace('.', "")).map_err(|err| err.kind().clone().into())))
            ),
            order_id:          selector!(
                get_and_text "td.td_nocontext", next,
                row, Self::inner_text,
                (|text: String| Some(u32::from_str(&text.replace("№ наряда: ", "")).map_err(|err| err.kind().clone().into())))
            ),
            internal_order_id: (|| Some(u32::from_str(rowid?).map_err(|err| err.kind().clone().into())))(),
            time_constrains:   selector!(select "th", row, (|elements: &mut Select| {
                let from = elements.skip(1).nth(0)?;
                let to = elements.nth(0)?;

                Some(TimeConstrains::from_work_schedule(
                    &Self::inner_text(&from)?,
                    &Self::inner_text(&to)?,
                ))
            })),
            phones:            (|| {
                let selector = format!(
                    r#"table tr td table tr.requestrow2[rowid="{}"] > td[rowspan="2"]"#,
                    rowid?
                );

                selector!(
                    get_and_text & selector,
                    next,
                    fragment,
                    Self::inner_text,
                    (|text: String| Some(text.split(',').map(|element| element.to_string()).collect_vec()))
                )
            })(),
            pa:                selector!(
                get_and_text "td.td_nocontext > b", next,
                row, Self::inner_text,
                (|text: String| Some(text))
            ),
            address:           selector!(get_as "a.viewAddrLink", next, row, Self::inner_text, Address::from_work_schedule),
            mdu:               selector!(get_as "td span.small", next, row, Self::inner_text, MDU::from_work_schedule),
            status:            selector!(get_as "span.networkstatus", next, row, Self::inner_text, Status::from_str),
            order_type:        selector!(select "td", row, (|elements: &mut Select| {
                let order_type_element = elements.filter(|element| element.value().attrs().count() == 0).nth(1)?;
                let order_type = Self::inner_text(&order_type_element)?;

                Some(OrderType::from_str(&order_type))
            })),
            client:            selector!(select "td", row, (|elements: &mut Select| {
                let client_element = elements.find(|element| element.value().attrs().count() == 0)?;
                let client = Self::inner_text(&client_element)?;

                Some(client.trim().to_string())
            })),
            internal_status:   selector!(get_as "td", last, row, Self::inner_text, InternalStatus::from_str),
        }
    }

    fn inner_text(element: &ElementRef<'_>) -> Option<String> {
        let text = element.text().collect_vec().join(" ");

        if text.is_empty() {
            return None;
        }

        Some(text)
    }
}
