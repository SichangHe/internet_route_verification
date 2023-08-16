pub use const_format::formatcp;
pub use once_cell::sync::{Lazy, OnceCell};
pub use regex::{Regex, RegexBuilder};

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
