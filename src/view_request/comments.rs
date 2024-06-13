use itertools::Itertools;
use scraper::{Html, Selector};

use super::ViewRequest;
use crate::data::full_comment::FullComment;

impl ViewRequest {
    pub fn set_comments(&mut self, html_fragment: &Html) {
        let comments_selector = Selector::parse(
            "#tblHistory > table > tbody > tr.tdeven:not([classcontext=\"tradereport\"]),#tblHistory > table > tbody \
             > tr.tdodd:not([classcontext=\"tradereport\"])",
        )
        .unwrap();

        self.comments = html_fragment
            .select(&comments_selector)
            .map(FullComment::from)
            .collect_vec();
    }
}
