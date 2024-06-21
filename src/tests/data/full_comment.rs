use chrono::DateTime;
use scraper::{Html, Selector};

use crate::data::full_comment::{FullComment, FullCommentError};
use crate::data::internal_status::{InternalStatus, InternalStatusError};
use crate::serializable_parse_error_kind::SerializableParseErrorKind;

#[test]
fn should_match() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>2024-05-16 14:17:49</td>
            <td></td>
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td>Выполнено</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Ok(FullComment {
            text:            Some("Полностью заменил кабель   поменял порт".to_string()),
            user:            "████ █████ █████████".to_string(),
            datetime:        DateTime::parse_from_rfc3339("2024-05-16 14:17:49+03:00").unwrap(),
            internal_status: InternalStatus::Completed,
        })
    )
}

// TODO: Return Err instead of panic
#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
fn invalid_html() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>2024-05-16 14:17:49</td>
            <!-- <td></td> -->
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td>Выполнено</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    let _ = FullComment::from(raw.select(&selector).next().unwrap());
}

#[test]
fn no_datetime() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td></td>
            <td></td>
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td>Выполнено</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Err(FullCommentError::DateTimeShouldBePresent)
    )
}

#[test]
fn no_user() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>2024-05-16 14:17:49</td>
            <td></td>
            <td></td>
            <td>10.1.162.25</td>
            <td>Выполнено</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Err(FullCommentError::UserShouldBePresent)
    )
}

#[test]
fn no_internal_status() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>2024-05-16 14:17:49</td>
            <td></td>
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td></td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Err(FullCommentError::InternalStatusShouldBePresent)
    )
}

#[test]
fn invalid_datetime() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>???</td>
            <td></td>
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td>Выполнено</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Err(FullCommentError::DateTimeFailed(
            "???".to_string(),
            SerializableParseErrorKind::Invalid
        ))
    )
}

#[test]
fn invalid_internal_status() {
    let raw = Html::parse_fragment(
        "<table><tr class=\"tdeven\" onclick=\"toggleDisplay(5914732)\">
            <td>2024-05-16 14:17:49</td>
            <td></td>
            <td>████ █████ █████████</td>
            <td>10.1.162.25</td>
            <td>???</td>
            <td>Полностью заменил кабель   поменял порт</td>     
        </tr></table>",
    );

    let selector = Selector::parse("tr").unwrap();
    assert_eq!(
        FullComment::from(raw.select(&selector).next().unwrap()),
        Err(FullCommentError::InvalidInternalStatus(
            InternalStatusError::InvalidStrStatus("???".to_string())
        ))
    )
}
