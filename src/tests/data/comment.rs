use chrono::DateTime;

use crate::data::comment::Comment;
use crate::serializable_parse_error_kind::SerializableParseErrorKind;

#[test]
fn should_match() {
    assert_eq!(
        Comment::from("Полностью заменил кабель поменял порт (████ █████ █████████,2024-05-16 14:17:49)"),
        Ok(Comment {
            text:     "Полностью заменил кабель поменял порт ".to_string(),
            user:     Some("████ █████ █████████".to_string()),
            datetime: Some(Ok(DateTime::parse_from_rfc3339("2024-05-16T14:17:49+03:00").unwrap())),
        })
    );

    assert_eq!(
        Comment::from("Полностью заменил кабель поменял порт (████ █████ █████████,2024-16 14:17:49)"),
        Ok(Comment {
            text:     "Полностью заменил кабель поменял порт (████ █████ █████████,2024-16 14:17:49)".to_string(),
            user:     None,
            datetime: None,
        })
    );

    assert_eq!(
        Comment::from("Полностью заменил кабель поменял порт"),
        Ok(Comment {
            text:     "Полностью заменил кабель поменял порт".to_string(),
            user:     None,
            datetime: None,
        })
    );
}

#[test]
fn should_not_match() {
    assert_eq!(
        Comment::from("Полностью заменил кабель поменял порт (████ █████ █████████,2024-05-16 14:99:49)"),
        Ok(Comment {
            text:     "Полностью заменил кабель поменял порт ".to_string(),
            user:     Some("████ █████ █████████".to_string()),
            datetime: Some(Err(SerializableParseErrorKind::OutOfRange)),
        })
    );
}
