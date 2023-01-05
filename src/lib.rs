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
//!   println!("{}", srt);
//!   ```
//!
//!

use std::fmt::Debug;
use std::str::FromStr;
use std::{fmt::Display, io::Error};

use serde::{Deserialize, Serialize};
use srt::SRTFile;
use ssa::SSAFile;
use vtt::VTTFile;
pub mod srt;
pub mod ssa;
pub mod util;
pub mod vtt;

pub trait Subtrait: Default + Display + FromStr + Debug {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subtitle<'a, T: Subtrait> {
    pub format: &'a str,
    pub subtitle: T,
}

impl<T: Subtrait> Default for Subtitle<'_, T> {
    fn default() -> Self {
        let s = "1
        00:11.000 --> 00:13.000
        We are in New York City
        ";
        Subtitle {
            format: "srt",
            subtitle: T::from_str(s).unwrap_or_default(),
        }
    }
}
impl Subtrait for SSAFile {}
impl Subtrait for SRTFile {}
impl Subtrait for VTTFile {}

impl<T: Subtrait> FromStr for Subtitle<'_, T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            let sr = s.to_string();
            if sr.lines().count() > 1 {
                let first_line = sr.lines().next().unwrap();
                if first_line.contains("WEBVTT") {
                    Ok(Subtitle {
                        format: "vtt",
                        subtitle: T::from_str(s).unwrap_or_default(),
                    })
                } else if first_line.contains("[Script Info]") {
                    Ok(Subtitle {
                        format: "ass",
                        subtitle: T::from_str(s).unwrap_or_default(),
                    })
                } else {
                    Ok(Subtitle {
                        format: "srt",
                        subtitle: T::from_str(s).unwrap_or_default(),
                    })
                }
            } else if sr.contains(".vtt") {
                Ok(Subtitle {
                    format: "vtt",
                    subtitle: T::from_str(s).unwrap_or_default(),
                })
            } else if sr.contains(".ass") || sr.contains(".ssa") {
                Ok(Subtitle {
                    format: "ass",
                    subtitle: T::from_str(s).unwrap_or_default(),
                })
            } else {
                Ok(Subtitle {
                    format: "srt",
                    subtitle: T::from_str(s).unwrap_or_default(),
                })
            }
        }
    }
}

// impl<T: Subtrait> Subtitle<'_, T> {
//     pub fn derive(s: &str) -> Result<T, Error>
//     where
//         T: Subtrait,
//     {
//         let sr = s.to_string();
//         if sr.lines().count() > 1 {
//             let first_line = sr.lines().next().unwrap();
//             if first_line.contains("WEBVTT") {
//                 Ok(T::from_str(s).unwrap_or_default())
//             } else if first_line.contains("[Script Info]") {
//                 Ok(T::from_str(s).unwrap_or_default())
//             } else {
//                 Ok(T::from_str(s).unwrap_or_default())
//             }
//         } else if sr.contains(".vtt") {
//             Ok(T::from_str(s).unwrap_or_default())
//         } else if sr.contains(".ass") || sr.contains(".ssa") {
//             Ok(T::from_str(s).unwrap_or_default())
//         } else {
//             Ok(T::from_str(s).unwrap_or_default())
//         }
//     }
// }

impl<T: Subtrait> Display for Subtitle<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.subtitle)
    }
}
