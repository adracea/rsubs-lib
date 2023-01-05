//! Implements helpers for `.srt`.
//!
//! It describes the [SRTFile] and [SRTLine] structs and
//! provides the [parse] function.

use serde::Deserialize;
use serde::Serialize;

use crate::util::time::Time;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str;
use std::str::FromStr;

use super::ssa::{SSAEvent, SSAFile};
use super::vtt::VTTFile;
use super::vtt::VTTLine;

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
///
/// The [parse] function takes as input a [String] that represents either the parsed file or the path to it.
///
/// A simple example of working with this would be the following :
///
///
/// ```
/// use rsubs_lib::srt::parse;
///
/// let mut a = parse("./tests/fixtures/test.srt".to_string()).unwrap();
/// for line in a.lines.iter_mut(){
///     line.line_text.push_str(" Ipsum"); // add "Ipsum" to the end of each line.
/// }
/// // print the parsed and modified `.srt` file
/// println!("{}",a.clone());
///
/// // and then write it to a file
/// a.to_file("./tests/fixtures/doctest1.srt".to_string());
///
/// ```
#[derive(Debug, Clone, PartialEq, Default, Eq, Serialize, Deserialize)]
pub struct SRTFile {
    pub lines: Vec<SRTLine>,
}

/// Describes each line
///
/// Each line has a start and end [Time], a [String] text and an [i32] line number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SRTLine {
    pub line_number: i32,
    pub line_text: String,
    pub line_start: Time,
    pub line_end: Time,
}
impl Default for SRTLine {
    fn default() -> Self {
        SRTLine {
            line_number: 0,
            line_text: "".to_string(),
            line_start: Time::default(),
            line_end: Time::default(),
        }
    }
}

impl SRTFile {
    /// Convert from [SRTFile] to [SSAFile] replacing `\r\n` to `\\N` since SSA/ASS is single line
    pub fn to_ass(self) -> SSAFile {
        let mut ssa = SSAFile::default();
        ssa.events.clear();
        for line in self.lines {
            let event = SSAEvent {
                line_start: line.line_start,
                line_end: line.line_end,
                line_text: line.line_text.replace("\r\n", "\\N"),
                ..Default::default()
            };
            ssa.events.push(event);
        }
        ssa
    }
    /// Convert from [SRTFile] to [VTTFile], WebVTT at its core is exactly the same as Subrip
    pub fn to_vtt(self) -> VTTFile {
        let mut vtt = VTTFile::default();
        vtt.lines.clear();
        for line in self.lines {
            let vttline = VTTLine {
                line_number: line.line_number.to_string(),
                line_start: line.line_start,
                line_end: line.line_end,
                line_text: line.line_text,
                ..Default::default()
            };
            vtt.lines.push(vttline);
        }
        vtt
    }
    /// Takes the path of the file in the form of a [String] to be written to as input.
    pub fn to_file(self, path: String) -> std::io::Result<()> {
        let mut w = File::options()
            .write(true)
            .create(true)
            .open(path)
            .expect("File can't be created");
        write!(w, "{}", self)?;
        Ok(())
    }
}
impl From<VTTFile> for SRTFile {
    fn from(a: VTTFile) -> Self {
        a.to_srt()
    }
}
impl From<SSAFile> for SRTFile {
    fn from(a: SSAFile) -> Self {
        a.to_srt()
    }
}
impl Display for SRTFile {
    /// Consumes self and dumps the file to String.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = "".to_string();
        for i in self.lines.clone() {
            let line = i.line_number.to_string()
                + "\r\n"
                + &i.line_start.to_string().replace('.', ",")
                + " --> "
                + &i.line_end.to_string().replace('.', ",")
                + "\r\n"
                + &i.line_text.replace("\\N", "\r\n")
                + "\r\n\r\n";
            lines += &line;
        }
        write!(f, "{}", lines)
    }
}

impl FromStr for SRTFile {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path_or_content = s.to_string();
        let mut b: String = "".to_string();
        let mut sub: SRTFile = SRTFile::default();
        if !path_or_content.contains("\r\n") {
            if std::fs::read(&path_or_content).is_ok() {
                let mut f = File::open(path_or_content).expect("Couldn't open file");
                f.read_to_string(&mut b).expect("Couldn't read file");
            }
        } else {
            b = path_or_content;
        }
        let lines = b.split("\r\n\r\n").collect::<Vec<&str>>();
        for i in lines {
            let mut subline = SRTLine::default();
            let subsplit: Vec<&str> = i.split("\r\n").collect();
            if !subsplit
                .first()
                .expect("Failed to parse line number")
                .is_empty()
            {
                subline.line_number = subsplit
                    .first()
                    .expect("Failed to parse line number")
                    .parse::<i32>()
                    .expect("Failed to parse line number");
                let mut timesplit = subsplit
                    .get(1)
                    .expect("Failed to parse times line")
                    .split(" --> ");
                (subline.line_start, subline.line_end) = (
                    Time::from_str(timesplit.next().unwrap()).unwrap(),
                    Time::from_str(timesplit.next().unwrap()).unwrap(),
                );
                subline.line_text = subsplit
                    .get(2..)
                    .expect("Couldn't find text")
                    .join("\r\n")
                    .replace("\r\n", "\\N");
                sub.lines.push(subline)
            }
        }
        Ok(sub)
    }
}

/// Parses the given [String] into a [SRTFile]
///
/// The string may represent either the path to a file or the file content itself.
pub fn parse(path_or_content: String) -> Result<SRTFile, std::io::Error> {
    let mut b: String = "".to_string();
    let mut sub: SRTFile = SRTFile::default();
    if !path_or_content.contains("\r\n") {
        if std::fs::read(&path_or_content).is_ok() {
            let mut f = File::open(path_or_content).expect("Couldn't open file");
            f.read_to_string(&mut b).expect("Couldn't read file");
        }
    } else {
        b = path_or_content;
    }
    let lines = b.split("\r\n\r\n").collect::<Vec<&str>>();
    for i in lines {
        let mut subline = SRTLine::default();
        let subsplit: Vec<&str> = i.split("\r\n").collect();
        if !subsplit
            .first()
            .expect("Failed to parse line number")
            .is_empty()
        {
            subline.line_number = subsplit
                .first()
                .expect("Failed to parse line number")
                .parse::<i32>()
                .expect("Failed to parse line number");
            let mut timesplit = subsplit
                .get(1)
                .expect("Failed to parse times line")
                .split(" --> ");
            (subline.line_start, subline.line_end) = (
                Time::from_str(timesplit.next().unwrap()).unwrap(),
                Time::from_str(timesplit.next().unwrap()).unwrap(),
            );
            subline.line_text = subsplit
                .get(2..)
                .expect("Couldn't find text")
                .join("\r\n")
                .replace("\r\n", "\\N");
            sub.lines.push(subline)
        }
    }
    Ok(sub)
}
