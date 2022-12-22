use crate::util::time::{self, Time};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str;

use super::ssa::{SSAEvent, SSAFile};
use super::vtt::VTTFile;
use super::vtt::VTTLine;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SRTFile {
    pub lines: Vec<SRTLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn to_vtt(self) -> VTTFile {
        let mut vtt = VTTFile::default();
        vtt.lines.clear();
        for line in self.lines {
            let vttline = VTTLine {
                line_number: line.line_number.to_string(),
                line_start: line.line_start,
                line_end: line.line_end,
                line_text: line.line_text.replace("\r\n", "\\N"),
                ..Default::default()
            };
            vtt.lines.push(vttline);
        }
        vtt
    }
    pub fn to_file(self, path: String) -> std::io::Result<()> {
        let mut w = File::options()
            .write(true)
            .create(true)
            .open(path)
            .expect("File can't be created");
        w.write_all(self.stringify().as_bytes())
            .expect("Couldn't write");
        Ok(())
    }
    pub fn stringify(self) -> String {
        let mut lines = "".to_string();
        for i in self.lines {
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
        lines
    }
}

pub fn parse(path_or_content: String) -> SRTFile {
    let mut b: String = "".to_string();
    let mut sub: SRTFile = SRTFile::default();
    if std::fs::read(&path_or_content).is_ok() {
        let mut f = File::open(path_or_content).expect("Couldn't open file");
        f.read_to_string(&mut b).expect("Couldn't read file");
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
                time::time_from_string(timesplit.next().unwrap().to_string()),
                time::time_from_string(timesplit.next().unwrap().to_string()),
            );
            subline.line_text = subsplit
                .get(2..)
                .expect("Couldn't find text")
                .join("\r\n")
                .replace("\r\n", "\\N");
            sub.lines.push(subline)
        }
    }
    sub
}
