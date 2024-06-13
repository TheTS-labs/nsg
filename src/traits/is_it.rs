use itertools::Itertools;
use scraper::ElementRef;

use super::prev_element_ref::PrevElementRef;

pub trait IsIt {
    fn is_it(
        &self,
        field: &str,
        element: ElementRef<'_>,
        text_getter: impl Fn(ElementRef<'_>) -> String,
    ) -> Option<String>;
}

impl<T: PrevElementRef> IsIt for T {
    fn is_it(
        &self,
        field: &str,
        element: ElementRef<'_>,
        text_getter: impl Fn(ElementRef<'_>) -> String,
    ) -> Option<String> {
        let prev = self.get_prev_element_ref(element)?;

        let prev_text = prev.text().collect_vec().join(" ").to_lowercase();

        let prev_element_name = prev.value().name();
        let cur_element_name = element.value().name();

        if prev_text == field && prev_element_name == "th" && (cur_element_name == "td" || cur_element_name == "a") {
            return Some(text_getter(element));
        }

        None
    }
}
