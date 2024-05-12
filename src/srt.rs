//! Implements helpers for `.srt`.
//!
//! It describes the [SRTFile] and [SRTLine] structs and
//! provides the [parse] function.

use serde::Deserialize;
use serde::Serialize;

use std::fmt::Display;
use std::str;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

use crate::error;
use time::Time;

use super::ssa::SSA;
use super::vtt::{VTTLine, VTT};

const TIME_FORMAT: &[BorrowedFormatItem] =
    format_description!("[hour]:[minute]:[second],[subsecond digits:3]");

/// Contains a Vec<[SRTLine]>
///
/// The `.srt` format is relatively simple to parse and generally looks like :
///```text
/// 0
/// 00:00:00,000 --> 00:00:02,000
/// This is my text
///
/// 1
/// 00:00:02,000 --> 00:00:04,000
/// This is my second text
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct SRT {
    pub lines: Vec<SRTLine>,
}

/// Describes each line
///
/// Each line has a start and end [Time], a [String] text and an [i32] line number.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SRTLine {
    pub sequence_number: u32,
    pub start: Time,
    pub end: Time,
    pub text: String,
}

impl SRT {
    /// Parses the given [String] into a [SRTFile].
    pub fn parse<S: AsRef<str>>(content: S) -> Result<SRT, SRTError> {
        let mut line_num = 0;

        let mut blocks = vec![vec![]];
        for line in content.as_ref().lines() {
            if line.trim().is_empty() {
                if !blocks.last().unwrap().is_empty() {
                    blocks.push(vec![])
                }
            } else {
                blocks.last_mut().unwrap().push(line)
            }
        }
        if blocks.last().is_some_and(|b| b.is_empty()) {
            blocks.remove(blocks.len() - 1);
        }

        let mut lines = vec![];
        for block in blocks {
            line_num += 1;

            let mut block_lines = block.into_iter();

            // sequence number
            let sequence_number = block_lines
                .next()
                .ok_or(SRTError::new(
                    SRTErrorKind::Parse("invalid sequence number".to_string()),
                    line_num,
                ))?
                .trim()
                .parse::<u32>()
                .map_err(|e| SRTError::new(SRTErrorKind::Parse(e.to_string()), line_num))?;
            line_num += 1;
            // start & end times
            let (start, end) = {
                let (start, end) = block_lines
                    .next()
                    .ok_or(SRTError::new(
                        SRTErrorKind::Parse("invalid time range".to_string()),
                        line_num,
                    ))?
                    .split_once("-->")
                    .ok_or(SRTError::new(
                        SRTErrorKind::Parse("invalid time range".to_string()),
                        line_num,
                    ))?;
                let start_time = Time::parse(start.trim(), TIME_FORMAT)
                    .map_err(|e| SRTError::new(SRTErrorKind::Parse(e.to_string()), line_num))?;
                let end_time = Time::parse(end.trim(), TIME_FORMAT)
                    .map_err(|e| SRTError::new(SRTErrorKind::Parse(e.to_string()), line_num))?;
                (start_time, end_time)
            };
            line_num += 1;
            line_num += block_lines.len();
            // text
            let text = block_lines.collect::<Vec<&str>>().join("\n");

            lines.push(SRTLine {
                sequence_number,
                start,
                end,
                text,
            })
        }

        Ok(SRT { lines })
    }

    /// Convert from [SRTFile] to [SSAFile] replacing `\r\n` to `\\N` since SSA/ASS is single line
    pub fn to_ssa(&self) -> SSA {
        self.to_vtt().to_ssa()
    }
    /// Convert from [SRTFile] to [VTTFile], WebVTT at its core is exactly the same as Subrip
    pub fn to_vtt(&self) -> VTT {
        VTT {
            regions: vec![],
            styles: vec![],
            lines: self
                .lines
                .iter()
                .map(|l| VTTLine {
                    identifier: Some(l.sequence_number.to_string()),
                    start: l.start,
                    end: l.end,
                    text: l.text.clone(),
                    ..Default::default()
                })
                .collect(),
        }
    }
}

impl Display for SRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks = vec![];

        for line in &self.lines {
            let mut block = vec![];

            block.push(line.sequence_number.to_string());
            block.push(format!(
                "{} --> {}",
                line.start.format(TIME_FORMAT).unwrap(),
                line.end.format(TIME_FORMAT).unwrap()
            ));
            block.push(line.text.clone());

            blocks.push(block)
        }

        write!(
            f,
            "{}",
            blocks
                .into_iter()
                .map(|b| b.join("\n"))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}

error! {
    SRTError => SRTErrorKind {
        Parse(String),
    }
}
