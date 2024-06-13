pub mod order;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::work_schedule::order::Order;

/// Representation of parsed work schedule
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Default, Deserialize)]
pub struct WorkSchedule(pub Vec<Order>);

impl WorkSchedule {
    /// Parse work schedule from html
    pub fn from(html: &str) -> WorkSchedule {
        let fragment = Html::parse_fragment(html);
        let orders = WorkSchedule::parse(&fragment);

        WorkSchedule(orders)
    }

    fn parse(fragment: &Html) -> Vec<Order> {
        let request_rows_selector = Selector::parse("table tr td table tr.requestrow").unwrap();
        let request_rows = fragment.select(&request_rows_selector);

        let mut orders = Vec::new();

        for row in request_rows {
            orders.push(Order::from_row_and_fragment(&row, fragment));
        }

        orders
    }
}
