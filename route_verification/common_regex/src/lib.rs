pub use const_format::formatcp;
pub use lazy_regex::regex::{Captures, Regex, RegexBuilder, Replacer};
pub use once_cell::sync::{Lazy, OnceCell};

pub mod set;
#[cfg(test)]
mod tests;

/// Lazy case-insensitive regex.
#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        use $crate::*;
        static RE: OnceCell<Regex> = OnceCell::new();
        RE.get_or_init(|| {
            RegexBuilder::new($re)
                .case_insensitive(true)
                .build()
                .unwrap()
        })
    }};
}

pub const RANGE_OPERATOR: &str = r"\^(:?\-|\+|[0-9]+(:?\-[0-9]+)?)";
