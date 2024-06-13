macro_rules! match_and_set {
    // Set self variable to the result of the closure
    (
        $look_for:expr, // Text to match, like "наряд:" or "подтип:"
        self.$field:ident, // Field in the self to set value
        $value:tt, // Value to set in the field
        $self:ident, // Struct's self
        $element:expr, // Element itself
        $text_getter:expr,
        $is_it:ident, // Matcher function, namely is_it, defined in the struct
        $cur_text:ident // https://stackoverflow.com/a/50349141/19729483
    ) => {
        if let Some($cur_text) = $self.$is_it($look_for, $element, $text_getter) {
            log::info!(
                target: "nsg",
                "[{:?}] Found self.{}, inner text: {:?}",
                $element.id(),
                stringify!($field),
                &$cur_text
            );

            #[allow(clippy::redundant_closure_call)]
            {
                $self.$field = $value();
            }
            continue;
        }
    };

    // Set local variable to the result of the closure
    (
        $look_for:expr, // Text to match, like "наряд:" or "подтип:"
        $field:ident, // Field in the self to set value
        $value:tt, // Value to set in the field
        $self:ident, // Struct's self
        $element:expr, // Element itself
        $text_getter:expr,
        $is_it:ident, // Matcher function, namely is_it, defined in the struct
        $cur_text:ident // https://stackoverflow.com/a/50349141/19729483
    ) => {
        if let Some($cur_text) = $self.$is_it($look_for, $element, $text_getter) {
            log::info!(
                target: "nsg",
                "[{:?}] Found {:?}, inner text: {:?}",
                $element.id(),
                stringify!($field),
                &$cur_text
            );
            $field = $value();
            continue;
        }
    };

    // Let the closure to set variable
    (
        $look_for:expr, // Text to match, like "наряд:" or "подтип:"
        $value:tt, // Value to set in the field
        $self:ident, // Struct's self
        $element:expr, // Element itself
        $text_getter:expr,
        $is_it:ident, // Matcher function, namely is_it, defined in the struct
        $cur_text:ident // https://stackoverflow.com/a/50349141/19729483
    ) => {
        if let Some($cur_text) = $self.$is_it($look_for, $element, $text_getter) {
            log::info!(
                target: "nsg",
                "[{:?}] Found {:?}, inner text: {:?}",
                $element.id(),
                stringify!($look_for),
                &$cur_text
            );

            #[allow(clippy::redundant_closure_call)]
            {
                $value();
            }
            continue;
        }
    };
}

macro_rules! selector {
    (select $selector:expr, $fragment:expr, $body:tt) => {
        (|| {
            let selector = Selector::parse($selector).ok()?;
            let mut elements = $fragment.select(&selector);

            $body(&mut elements)
        })()
    };

    (get_and_text $selector:expr, $which:ident, $fragment:expr, $inner_text:expr, $body:tt) => {
        (|| {
            let element = $fragment.select(&Selector::parse($selector).ok()?).$which()?;

            $body($inner_text(&element)?)
        })()
    };

    (get_as $selector:expr, $which:ident, $fragment:expr, $inner_text:expr, $func:expr) => {
        (|| {
            let element = $fragment.select(&Selector::parse($selector).ok()?).$which()?;

            Some($func(&$inner_text(&element)?))
        })()
    };
}

pub(crate) use {match_and_set, selector};
