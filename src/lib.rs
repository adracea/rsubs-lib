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
//!   use std::str::FromStr;
//!   let mut srt: rsubs_lib::srt::SRTFile = rsubs_lib::Subtitle::from_str("test.srt").expect("failed parsing").into();
//!   for line in srt.lines.iter_mut() {
//!       line.line_end += 1000;
//!       line.line_start += 1000;
//!   }
//!   println!("{}", srt);
//!   ```
//!
//!

use std::str::FromStr;
use std::{fmt::Display, io::Error};

use srt::SRTFile;
use ssa::SSAFile;
use vtt::VTTFile;
pub mod srt;
pub mod ssa;
pub mod util;
pub mod vtt;

#[derive(Clone, Debug)]
pub enum Subtitle {
    SRT(Option<SRTFile>),
    VTT(Option<VTTFile>),
    SSA(Option<SSAFile>),
}

impl FromStr for Subtitle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sr = s.to_string();
        if sr.lines().count() > 1 {
            let first_line = sr.lines().next().unwrap();
            if first_line.contains("WEBVTT") {
                Ok(Subtitle::VTT(Some(
                    VTTFile::from_str(s).unwrap_or_default(),
                )))
            } else if first_line.contains("[Script Info]") {
                Ok(Subtitle::SSA(Some(
                    SSAFile::from_str(s).unwrap_or_default(),
                )))
            } else {
                Ok(Subtitle::SRT(Some(
                    SRTFile::from_str(s).unwrap_or_default(),
                )))
            }
        } else if sr.contains(".vtt") {
            Ok(Subtitle::VTT(Some(
                VTTFile::from_str(s).unwrap_or_default(),
            )))
        } else if sr.contains(".ass") || sr.contains(".ssa") {
            Ok(Subtitle::SSA(Some(
                SSAFile::from_str(s).unwrap_or_default(),
            )))
        } else {
            Ok(Subtitle::SRT(Some(
                SRTFile::from_str(s).unwrap_or_default(),
            )))
        }
    }
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subtitle::SRT(Some(i)) => write!(f, "{i}"),
            Subtitle::VTT(Some(i)) => write!(f, "{i}"),
            Subtitle::SSA(Some(i)) => write!(f, "{i}"),
            _ => panic!("format error"),
        }
    }
}

impl From<Subtitle> for SSAFile {
    fn from(value: Subtitle) -> Self {
        match value {
            Subtitle::SRT(Some(i)) => i.into(),
            Subtitle::VTT(Some(i)) => i.into(),
            Subtitle::SSA(Some(i)) => i,
            _ => panic!("format error"),
        }
    }
}
impl From<Subtitle> for VTTFile {
    fn from(value: Subtitle) -> Self {
        match value {
            Subtitle::SRT(Some(i)) => i.into(),
            Subtitle::VTT(Some(i)) => i,
            Subtitle::SSA(Some(i)) => i.into(),
            _ => panic!("format error"),
        }
    }
}

impl From<Subtitle> for SRTFile {
    fn from(value: Subtitle) -> Self {
        match value {
            Subtitle::SRT(Some(i)) => i,
            Subtitle::VTT(Some(i)) => i.into(),
            Subtitle::SSA(Some(i)) => i.into(),
            _ => panic!("format error"),
        }
    }
}
