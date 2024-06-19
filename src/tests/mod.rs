mod brief_request;
mod view_request;
mod work_schedule;

macro_rules! test {
    ($set:expr, $parser:ident; $($var:ident: { $($expected:expr, $name:ident;)* }),* $(,)?) => {
        $($(paste! {
            #[test]
            fn [<$var _ $name>]() {
                let html = fs::read_to_string(
                    format!("src/tests/assets/{}/invalid/{}.html", $set, stringify!($var))
                ).expect("Should have been able to read the file");

                let view_request = $parser::from(&html);

                assert_eq!(
                    view_request.$var, $expected,
                    "{}.{} != {}. Got {:?} ({:#?})",
                    $set, stringify!($var), stringify!($expected), view_request.$var, view_request
                );
            }
        })*)*
    };

    ($set:expr, $parser:ident; $($var:ident: { $($expected:expr, $name:ident, $method:ident $(, $method_arg:expr)?;)* }),* $(,)?) => {
        $($(paste! {
            #[test]
            fn [<$var _ $name>]() {
                let html = fs::read_to_string(
                    format!("src/tests/assets/{}/invalid/{}.html", $set, stringify!($var))
                ).expect("Should have been able to read the file");

                let work_schedule = $parser::from(&html);
                let order = work_schedule.0.$method($($method_arg)*).unwrap();

                assert_eq!(
                    order.$var, $expected,
                    "{}.{} != {}. Got {:?} ({:#?})",
                    $set, stringify!($var), stringify!($expected), order.$var, order
                );
            }
        })*)*
    }
}

use test;
