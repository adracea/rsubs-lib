//! # rsubs-lib
//!
//! This [crate](https://crates.io/crates/rsubs-lib) provides a simple way for parsing, modifying or converting
//! subtitle files such as `.srt`,`.ssa`,`.ass` and `.vtt`.
//!
//! Example usage:
//! ```
//! use std::ops::Add;
//! use std::time::Duration;
//! use rsubs_lib::SRT;
//!
//! let raw_srt = r#"1
//! 00:00:00,000 --> 00:00:02,000
//! This is a example .srt file
//!
//! 2
//! 00:00:02,000 --> 00:00:06,000
//! The following code will delay the subtitles by one second
//! "#;
//!
//! let mut srt = SRT::parse(raw_srt).unwrap();
//! for line in &mut srt.lines {
//!     line.start.add(Duration::from_secs(1));
//!     line.end.add(Duration::from_secs(1));
//! }
//! println!("{}", srt)
//! ```
//!
//!
mod srt;
mod ssa;
pub mod util;
mod vtt;

pub use srt::*;
pub use ssa::*;
pub use vtt::*;

macro_rules! error {
    ($error_name:ident => $kind_name:ident { $($field:ident $(($($t:ty),*))?),*, }) => {
        #[derive(Debug, Eq, PartialEq)]
        pub struct $error_name {
            line: usize,
            kind: $kind_name
        }

        impl $error_name {
            fn new(kind: $kind_name, line: usize) -> Self {
                Self { line, kind }
            }

            pub fn line(&self) -> usize {
                self.line
            }

            pub fn kind(&self) -> &$kind_name {
                &self.kind
            }
        }

        impl std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "error at line {}: {:?}", self.line, self.kind)
            }
        }

        impl std::error::Error for $error_name {}

        #[derive(Debug, Eq, PartialEq)]
        pub enum $kind_name {
            $($field $(($($t),*))?),*
        }
    };
}
pub(crate) use error;

fn strip_bom<S: AsRef<str>>(content: &S) -> &str {
    let content = content.as_ref();
    content.strip_prefix('\u{FEFF}').unwrap_or(content)
}
