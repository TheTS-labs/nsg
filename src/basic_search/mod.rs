//! Basic search parser (collection of [`SearchEntry`]s)
//!
//! ## Example usage
//! You can find example HTMLs in `src/tests/assets/basic_search/valid`
//!
//! ```
//! use nsg::basic_search::BasicSearch;
//!
//! let html = include_str!("../tests/assets/basic_search/valid/1.html");
//! let basic_search = BasicSearch::from(&html);
//!
//! println!("Search entries: {:#?}", basic_search.0);
//! ```

pub mod search_entry;

use std::fmt::Debug;
use std::str::FromStr;

use chrono::NaiveDate;
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use self::search_entry::SearchEntry;
use crate::data::address::Address;
use crate::data::internal_status::InternalStatus;
use crate::data::mdu::MDU;

/// Parsed work schedule containing vector of [`SearchEntry`]s
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Default, Deserialize)]
pub struct BasicSearch(pub Vec<SearchEntry>);

impl BasicSearch {
    /// Parse basic search from HTML
    pub fn from(html: &str) -> BasicSearch {
        let mut basic_search = BasicSearch::default();

        let fragment = Html::parse_fragment(&format!("<table>{html}</table>"));
        basic_search.collect_fragments(&fragment);

        basic_search
    }

    fn collect_fragments(&mut self, fragment: &Html) {
        let selector = Selector::parse("tr").unwrap();
        let search_entries_elements = fragment.select(&selector);

        for search_entry_element in search_entries_elements {
            let mut search_entry = SearchEntry::default();
            let mut children = search_entry_element.child_elements();

            search_entry.internal_order_id = BasicSearch::get_prop(children.nth(0), |value| {
                Some(u32::from_str(&value).map_err(|err| err.kind().clone().into()))
            });
            search_entry.order_id = BasicSearch::get_prop(children.nth(0), |value| {
                Some(u32::from_str(&value).map_err(|err| err.kind().clone().into()))
            });
            let city = BasicSearch::get_text(children.nth(0));
            let street = BasicSearch::get_text(children.nth(0));
            let building = BasicSearch::get_text(children.nth(0));
            let apartment = BasicSearch::get_text(children.nth(0));
            search_entry.client = BasicSearch::get_prop(children.nth(0), |value| Some(value.trim().to_string()));
            search_entry.phone_number = BasicSearch::get_prop(children.nth(0), Some);
            search_entry.last_updated = BasicSearch::get_prop(children.nth(0), |value| {
                Some(NaiveDate::parse_from_str(&value, "%Y-%m-%d").map_err(|err| err.kind().into()))
            });
            search_entry.installer = BasicSearch::get_prop(children.nth(2), Some);
            search_entry.internal_status =
                BasicSearch::get_prop(children.nth(0), |value| Some(InternalStatus::from_str(&value)));
            search_entry.mdu = BasicSearch::get_prop(children.nth(0), |value| Some(MDU::from_work_schedule(&value)));

            if let (Some(city), Some(street), Some(building), Some(apartment)) = (city, street, building, apartment) {
                search_entry.address = Some(Address::from_parts(city, street, building, apartment));
            }

            self.0.push(search_entry);
        }
    }

    fn get_text(element: Option<ElementRef<'_>>) -> Option<String> {
        let string = element?.text().collect_vec().join(" ");

        if string.is_empty() {
            return None;
        }

        Some(string)
    }

    fn get_prop<T>(element: Option<ElementRef<'_>>, callback: impl FnOnce(String) -> Option<T>) -> Option<T> {
        callback(BasicSearch::get_text(element)?)
    }
}
