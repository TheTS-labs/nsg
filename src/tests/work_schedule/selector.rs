use itertools::Itertools;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};

#[test]
fn selector_select() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(select "p", html.root_element(), (|elements: &mut Select| {
        let element = elements.find(|element| element.value().attrs().count() == 0)?;

        Some(element.text().collect_vec().join(""))
    }));

    assert_eq!(result, Some("Another".to_string()));
}

#[test]
fn selector_select_invalid() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(select "\"&$^", html.root_element(), (|elements: &mut Select| {
        let element = elements.find(|element| element.value().attrs().count() == 0)?;

        Some(element.text().collect_vec().join(""))
    }));

    assert_eq!(result, None);
}

#[test]
fn selector_get_and_text() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_and_text "p", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: String| Some(text))
    );

    assert_eq!(result, Some("Test".to_string()));
}

#[test]
fn selector_get_and_text_invalid() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_and_text "\"&$^", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: String| Some(text))
    );

    assert_eq!(result, None);
}

#[test]
fn selector_get_and_text_no_elements() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_and_text "th", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: String| Some(text))
    );

    assert_eq!(result, None);
}

#[test]
fn selector_get_and_text_no_text() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_and_text "p", next,
        html.root_element(), (|_element: &ElementRef| None),
        (|text: String| Some(text))
    );

    assert_eq!(result, None);
}

#[test]
fn selector_get_as() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_as "p", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: &String| Some(text.clone()))
    );

    assert_eq!(result, Some(Some("Test".to_string())));
}

#[test]
fn selector_get_as_invalid() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_as "\"&$^", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: &String| Some(text.clone()))
    );

    assert_eq!(result, None);
}

#[test]
fn selector_get_as_no_element() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_as "th", next,
        html.root_element(), (|element: &ElementRef| Some(element.text().collect_vec().join(" "))),
        (|text: &String| Some(text.clone()))
    );

    assert_eq!(result, None);
}

#[test]
fn selector_get_as_no_text() {
    let html = Html::parse_fragment("<div><p tag='1'>Test</p><p>Another</p></div>");
    let result = selector!(
        get_as "p", next,
        html.root_element(), (|_element: &ElementRef| None),
        (|text: &String| Some(text.clone()))
    );

    assert_eq!(result, None);
}
