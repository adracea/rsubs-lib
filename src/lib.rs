//! # rsubs-lib
//!
//! This [crate](https://crates.io/crates/rsubs-lib) provides a simple way for parsing, modifying or converting
//! subtitle files such as `.srt`,`.ssa`,`.ass` and `.vtt`.
//!
//! Example usage :
//!
//! In this example we read a .srt file and add 1s(1000ms) to each line
//! Afterwards we print the result to stdout.
//!
//!   ```
//!   let mut srt = rsubs_lib::srt::parse("test.srt".to_string()).expect("failed parsing");
//!   for line in srt.lines.iter_mut() {
//!       line.line_end += 1000;
//!       line.line_start += 1000;
//!   }
//!   println!("{}", srt.stringify());
//!   ```
//!
//!
pub mod srt;
pub mod ssa;
pub mod util;
pub mod vtt;
