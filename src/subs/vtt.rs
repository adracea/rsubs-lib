use super::srt::SRTLine;
use super::ssa::{SSAEvent, SSAFile, SSAStyle};
use crate::subs::srt::SRTFile;
use crate::util::time::time_from_string;
use crate::util::{color, color::Color, time, time::Time};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct VTTStyle {
    pub color: Color,
    pub font_family: String,
    pub font_size: String,
    pub text_shadow: String,
    pub background_color: Color,
    pub name: Option<String>,
    pub others: HashMap<String, String>,
}
pub static OVERRIDE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"<(?P<trigger>.*?)>").expect("Regex Failure"));
impl Default for VTTStyle {
    fn default() -> Self {
        VTTStyle {
            color: color::WHITE,
            font_family: "\"Trebuchet MS\"".to_string(),
            font_size: "020px".to_string(),
            text_shadow: "#000000ff -2px 0px 2px, #000000ff 0px 2px 2px, #000000ff 0px -2px 2px, #000000ff 2px 0px 2px".to_string(),
            background_color: color::TRANSPARENT,
            name: None,
            others: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VTTLine {
    pub line_number: String,
    pub style: Option<String>,
    pub line_start: Time,
    pub line_end: Time,
    pub position: Option<VTTPos>,
    pub line_text: String,
}
impl Default for VTTLine {
    fn default() -> Self {
        VTTLine {
            line_number: "0".to_string(),
            style: Some("Default".to_string()),
            line_start: time_from_string("00:00:00.000".to_string()),
            line_end: time_from_string("00:00:02.000".to_string()),
            position: Some(VTTPos::default()),
            line_text: "Lorem Ipsum".to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VTTPos {
    pub pos: i32,
    pub size: i32,
    pub line: i32,
    pub align: String,
}
impl Default for VTTPos {
    fn default() -> Self {
        VTTPos {
            pos: 0,
            size: 0,
            line: 0,
            align: "center".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct VTTFile {
    pub styles: Vec<VTTStyle>,
    pub lines: Vec<VTTLine>,
}
impl Default for VTTFile {
    fn default() -> Self {
        VTTFile {
            styles: vec![VTTStyle::default()],
            lines: vec![VTTLine::default()],
        }
    }
}

pub fn parse(path_or_content: String) -> VTTFile {
    let mut b: String = "".to_string();
    let mut sub: VTTFile = VTTFile::default();
    if std::fs::read(&path_or_content).is_ok() {
        let mut f = File::open(path_or_content).expect("Couldn't open file");
        f.read_to_string(&mut b).expect("Couldn't read file");
    } else {
        b = path_or_content;
    }
    let line_blocks = b.split("\r\n\r\n").collect::<Vec<&str>>();
    if !line_blocks.first().unwrap().contains("WEBVTT") {
        panic!("File not .VTT")
    }
    let mut line_found = false;
    let mut styles_found = 0;
    for i in line_blocks {
        if i.trim().starts_with("::cue") | i.trim().starts_with("STYLE") {
            let line = i.split("\r\n").collect::<Vec<&str>>();
            let mut styl = VTTStyle::default();
            for i in line {
                if i.starts_with("color:") {
                    styl.color = Color::from_str(
                        i.split(": ")
                            .collect::<Vec<&str>>()
                            .get(1)
                            .expect("No Color ")
                            .strip_suffix(';')
                            .expect("Broken Color"),
                    )
                    .expect("Broken Color");
                } else if i.starts_with("font-family:") {
                    styl.font_family = i
                        .split(": ")
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("No Font ")
                        .strip_suffix(';')
                        .expect("Broken Font")
                        .to_string();
                } else if i.starts_with("font-size:") {
                    styl.font_size = i
                        .split(": ")
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("No Font size")
                        .strip_suffix(';')
                        .expect("Broken Font size")
                        .to_string();
                } else if i.starts_with("text-shadow:") {
                    styl.text_shadow = i
                        .split(": ")
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("No Font size")
                        .strip_suffix(';')
                        .expect("Broken Font size")
                        .to_string();
                } else if i.starts_with("background-shadow:") {
                    styl.background_color = Color::from_str(
                        i.split(": ")
                            .collect::<Vec<&str>>()
                            .get(1)
                            .expect("No Color ")
                            .strip_suffix(';')
                            .expect("Broken Color"),
                    )
                    .expect("Broken Color");
                } else if i.starts_with("::cue(") {
                    styl.name = Some(
                        i.split(&['(', ')'])
                            .collect::<Vec<&str>>()
                            .get(1)
                            .expect("Name")
                            .to_string(),
                    );
                }
            }
            styles_found += 1;
            if styles_found == 1 {
                sub.styles.clear();
            }
            sub.styles.push(styl);
        } else if i.trim().starts_with("NOTE") || i.trim().starts_with("WEBVTT") {
            continue;
        } else {
            let mut subline = VTTLine::default();
            let subsplit: Vec<&str> = i.split("\r\n").collect();
            if !subsplit
                .first()
                .expect("Failed to parse line number")
                .is_empty()
            {
                let mut idxshift: usize = 0;
                subline.line_number = if !subsplit
                    .first()
                    .expect("Failed to parse line number")
                    .to_string()
                    .contains(" --> ")
                {
                    subsplit
                        .first()
                        .expect("Failed to parse line number")
                        .to_string()
                } else {
                    idxshift += 1;
                    "".to_string()
                };

                let mut timesplit = subsplit
                    .get(1 - idxshift)
                    .expect("Failed to parse times line")
                    .split(" --> ");
                (subline.line_start, subline.line_end) = (
                    time::time_from_string(timesplit.next().unwrap().to_string()),
                    time::time_from_string(
                        timesplit
                            .next()
                            .unwrap()
                            .to_string()
                            .splitn(2, ' ')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .to_string(),
                    ),
                );
                let mut spos = VTTPos::default();
                let posstring: String = subsplit
                    .get(1 - idxshift)
                    .expect("Failed to parse times line")
                    .to_string()
                    .splitn(4, ' ')
                    .collect::<Vec<&str>>()
                    .get(3)
                    .unwrap_or(&"")
                    .to_string();
                let mut poss: HashMap<String, String> = HashMap::new();
                posstring.split(' ').for_each(|x| {
                    poss.insert(
                        x.split(':')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap_or(&"")
                            .to_string(),
                        x.split(':')
                            .collect::<Vec<&str>>()
                            .get(1)
                            .unwrap_or(&"")
                            .to_string(),
                    );
                });
                for (px, py) in poss {
                    if px == "position" {
                        spos.pos = py.replace('%', "").parse::<i32>().expect("number");
                    } else if px == "align" {
                        spos.align = py;
                    } else if px == "size" {
                        spos.size = py.replace('%', "").parse::<i32>().expect("number");
                    } else if px == "line" {
                        spos.line = py.replace('%', "").parse::<i32>().expect("number");
                    }
                }
                subline.position = Some(spos);
                subline.line_text = subsplit
                    .get((2 - idxshift)..)
                    .expect("Couldn't find text")
                    .join("\r\n")
                    .replace("\r\n", "\\N");
                if !line_found {
                    sub.lines.clear();
                    line_found = true;
                }
                sub.lines.push(subline)
            }
        }
    }
    sub
}

impl VTTFile {
    pub fn to_file(self, path: String) -> std::io::Result<()> {
        let mut w = File::options()
            .write(true)
            .create(true)
            .open(path)
            .expect("File can't be created");
        w.write_all("WEBVTT\r\n\r\n".as_bytes())?;
        for i in self.styles {
            let mut style_block: String = "".to_string();
            if i.name.is_some() {
                style_block += &("STYLE\r\n::cue(".to_string() + &i.name.unwrap() + ") {\r\n");
            } else {
                style_block += "STYLE\r\n::cue {\r\n";
            }
            style_block += &("color: ".to_string() + &i.color.to_string() + ";\r\n");
            style_block +=
                &("background-color: ".to_string() + &i.background_color.to_string() + ";\r\n");
            style_block += &("font-family: ".to_string() + &i.font_family + ";\r\n");
            style_block += &("font-size: ".to_string() + &i.font_size.to_string() + ";\r\n");
            style_block += &("text-shadow: ".to_string() + &i.text_shadow.to_string() + ";\r\n");
            style_block += "}\r\n\r\n";
            w.write_all(style_block.as_bytes())?;
        }
        for (i, j) in self.lines.iter().enumerate() {
            let mut line_block: String = "".to_string();
            if j.line_number.is_empty() {
                line_block += &(i.to_string() + "\r\n")
            } else {
                line_block += &(j.line_number.to_string() + "\r\n")
            }
            line_block += &(j.line_start.to_string() + " --> " + &j.line_end.to_string());
            if j.position.is_some() {
                let pos = j.position.clone().unwrap();
                line_block += &format!(
                    " position:{:0>3}% size:{:0>3}% line:{} align:{}\r\n",
                    pos.pos, pos.size, pos.line, pos.align
                );
            } else {
                line_block += "\r\n";
            }
            line_block += &(j.line_text.to_string().replace("\\N", "\r\n") + "\r\n\r\n");
            w.write_all(line_block.as_bytes())?;
        }
        Ok(())
    }
    pub fn to_ass(self) -> SSAFile {
        let mut ssa = SSAFile::default();
        ssa.events.clear();
        for (_ctr, i) in self.styles.into_iter().enumerate() {
            let styl = SSAStyle {
                firstcolor: i.color,
                fontname: i
                    .font_family
                    .split('\"')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .to_string(),
                backgroundcolor: i.background_color,
                name: i.name.unwrap_or_else(|| "Default".to_string()),
                fontsize: i
                    .font_size
                    .strip_suffix("px")
                    .unwrap_or(&i.font_size.to_string())
                    .to_string()
                    .parse::<f32>()
                    .expect("Couldn't parse to float"),
                ..Default::default()
            };
            ssa.styles.push(styl)
        }
        for (_ctr, i) in self.lines.into_iter().enumerate() {
            let mut line = SSAEvent {
                line_end: i.line_end,
                line_start: i.line_start,
                line_text: i.line_text.clone(),
                ..Default::default()
            };
            let invalid_line_text = OVERRIDE.captures_iter(&i.line_text);
            for k in invalid_line_text {
                let tag_main = k.get(0).unwrap().as_str();
                let tag_trigger = k.name("trigger").unwrap().as_str();
                if !["/b", "b", "/i", "i", "/u", "u"].contains(&tag_trigger) {
                    line.line_text = line.line_text.clone().replace(tag_main, "");
                }
            }
            ssa.events.push(line);
        }
        ssa
    }
    pub fn to_srt(self) -> SRTFile {
        let mut srt = SRTFile::default();
        srt.lines.clear();
        for (ctr, i) in self.lines.into_iter().enumerate() {
            let mut line = SRTLine {
                line_number: i.line_number.parse::<i32>().unwrap_or(ctr as i32),
                line_end: i.line_end,
                line_start: i.line_start,
                line_text: i.line_text.clone(),
            };
            let invalid_line_text = OVERRIDE.captures_iter(&i.line_text);
            for k in invalid_line_text {
                let tag_main = k.get(0).unwrap().as_str();
                let tag_trigger = k.name("trigger").unwrap().as_str();
                if !["/b", "b", "/i", "i", "/u", "u"].contains(&tag_trigger) {
                    line.line_text = line.line_text.clone().replace(tag_main, "");
                }
            }
            srt.lines.push(line);
        }
        srt
    }
}
