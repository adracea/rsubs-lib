//! Implements helpers for `.ass` and `.ssa`.
//!
//! It describes the [SSAFile], [SSAEvent] and [SSAStyle] structs and
//! provides the [parse] function.

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Display,
    fs,
    io::{Read, Write},
    str::FromStr,
};

use crate::util::{
    color::{self, Alignment, Color},
    time::Time,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use super::srt::SRTLine;
use super::{srt::SRTFile, vtt::VTTFile, vtt::VTTLine, vtt::VTTStyle};

/// [SSAStyle] describes each part of the `Format: ` side of a `.ssa` or `.ass` subtitle.
///
/// It holds [color::ColorType] for handling colors and exposes parameters for every part of the
/// SSA Style header.
///
/// Currently only supports `.ass`, more precisely `ScriptType: V4.00+` and `[V4+ Styles]`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SSAStyle {
    pub name: String,
    pub fontname: String,
    pub fontsize: f32,
    pub firstcolor: color::ColorType,
    pub secondcolor: color::ColorType,
    pub outlinecolor: color::ColorType,
    pub backgroundcolor: color::ColorType,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub scalex: f32,
    pub scaley: f32,
    pub spacing: f32,
    pub angle: f32,
    pub borderstyle: i8,
    pub outline: f32,
    pub shadow: f32,
    pub alignment: color::Alignment,
    pub lmargin: i32,
    pub rmargin: i32,
    pub vmargin: i32,
    pub alpha: i32,
    pub encoding: i32,
    pub drawing: bool,
}
impl Eq for SSAStyle {}
impl Default for SSAStyle {
    fn default() -> Self {
        SSAStyle {
            name: "Default".to_string(),
            fontname: "Trebuchet MS".to_string(),
            fontsize: 25.5,
            firstcolor: color::ColorType::SSAColor(color::WHITET),
            secondcolor: color::ColorType::SSAColor(color::TRANSPARENT),
            outlinecolor: color::ColorType::SSAColor(color::TRANSPARENT),
            backgroundcolor: color::ColorType::SSAColor(color::TRANSPARENT),
            bold: false,
            italic: true,
            underline: true,
            strikeout: true,
            scalex: 120.0,
            scaley: 120.0,
            spacing: 0.0,
            angle: 0.0,
            borderstyle: 1,
            outline: 1.0,
            shadow: 1.0,
            alignment: Alignment::BottomCenter,
            lmargin: 0,
            rmargin: 0,
            vmargin: 30,
            alpha: 0,
            encoding: 0,
            drawing: false,
        }
    }
}

/// Describes each individual element of an `Event` line in the `.ass` format
///
/// Each element can be individually changed.
///
/// Because of its comma separated values in the event line, the timestamp looks like
/// `00:00:20.00` and it can be represented using [Time::to_ass_string]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SSAEvent {
    /// Defaults to 0
    pub layer: i32,
    /// [Time] Value representing the start time of the line being displayed
    pub line_start: Time,
    /// [Time] Value representing the end time of the line being displayed
    pub line_end: Time,
    /// String value relating to an [SSAStyle]
    pub style: String,
    /// Generally this is used for "speaker name", in most cases it's an unused field
    pub name: String,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub lmargin: f32,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub rmargin: f32,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub vmargin: f32,
    /// SSA Documentation describes it, it's here, no idea what it does, but you can write it if you wish
    pub effect: String,
    /// SSA Documentation describes it, it's here, no idea what it does, but you can write it if you wish
    pub linetype: String,
    /// The line's text.
    pub line_text: String,
}
impl Eq for SSAEvent {}
impl Default for SSAEvent {
    fn default() -> Self {
        SSAEvent {
            layer: 0,
            line_start: Time::from_str("0:00:00.20").unwrap(),
            line_end: Time::from_str("0:00:02.20").unwrap(),
            style: "Default".to_string(),
            name: "".to_string(),
            lmargin: 0.0,
            rmargin: 0.0,
            vmargin: 0.0,
            effect: "".to_string(),
            linetype: "Dialogue".to_string(),
            line_text: "Lorem Ipsum".to_string(),
        }
    }
}
/// Contains the styles,events and info as well as a format mentioning wether it's `.ass` or `.ssa`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SSAFile {
    pub events: Vec<SSAEvent>,
    pub styles: Vec<SSAStyle>,
    pub info: HashMap<String, String>,
    pub format: String,
}
/// The Default SSAFile contains a list of Script Info headers populated with safe and usable default values
///
/// In general tests, `ScaledBorderAndShadows: yes` seems to be somewhat required for subtitles to display properly
impl Default for SSAFile {
    fn default() -> Self {
        let mut default_info: HashMap<String, String> = HashMap::new();
        default_info.insert("Title".to_string(), "subtitle".to_string());
        default_info.insert("Synch Point".to_string(), "".to_string());
        default_info.insert("Script Updated By".to_string(), "rsubs lib".to_string());
        default_info.insert("ScriptType".to_string(), "V4.00+".to_string());
        default_info.insert("Collisions".to_string(), "Normal".to_string());
        default_info.insert("WrapStyle".to_string(), "0".to_string());
        default_info.insert("ScaledBorderAndShadows".to_string(), "yes".to_string());
        default_info.insert("PlayResX".to_string(), "640".to_string());
        default_info.insert("PlayResY".to_string(), "480".to_string());
        SSAFile {
            events: vec![SSAEvent::default()],
            styles: vec![SSAStyle::default()],
            info: default_info,
            format: ".ass".to_string(),
        }
    }
}

impl From<VTTFile> for SSAFile {
    fn from(a: VTTFile) -> Self {
        a.to_ass()
    }
}
impl From<SRTFile> for SSAFile {
    fn from(a: SRTFile) -> Self {
        a.to_ass()
    }
}
impl SSAFile {
    /// Converts the SSAFile to a SRTFile. Due to `.srt` being a far less complex
    /// format, most styles are being ignored.
    ///
    /// Styling of the text can happen with `{i1}aaa{i0}` tags where `i` represents
    ///  the style and `0`/`1` represent the on/off triggers.
    ///
    /// `.srt` supports HTML-like tags for `i`,`b`,`u`, representing italic, bold, underline.
    ///
    /// If found, ssa specific triggers for those supported tags are replaced with their `.srt` alternatives.
    ///
    pub fn to_srt(self) -> SRTFile {
        let mut a = SRTFile::default();
        let regex =
            Regex::new(r"(?P<main>\{\\(?P<type>.)(?P<trigger>.*?)\})").expect("Regex broke");
        for (i, j) in self.events.iter().enumerate() {
            let mut line = SRTLine {
                line_number: (i + 1) as i32,
                line_start: j.line_start.clone(),
                line_end: j.line_end.clone(),
                line_text: "".to_string(),
            };

            line.line_text = j.line_text.replace("\\N", "\r\n");

            for k in regex.captures_iter(&line.line_text.clone()) {
                let tag_type = k.name("type").unwrap().as_str();
                let tag_main = k.name("main").unwrap().as_str();
                let tag_trigger = k.name("trigger").unwrap().as_str();
                if tag_type.chars().all(|x| ['b', 'i', 'u'].contains(&x)) {
                    if tag_trigger == "0" {
                        line.line_text = line
                            .line_text
                            .replace(tag_main, &("</".to_string() + tag_type + ">"));
                    } else if tag_trigger == "1" {
                        line.line_text = line
                            .line_text
                            .replace(tag_main, &("<".to_string() + tag_type + ">"));
                    }
                }
            }
            a.lines.push(line);
        }
        a
    }
    /// Converts the SSAFile to a VTTFile.
    ///
    /// Styling of the text can happen with `{i1}aaa{i0}` tags where `i` represents
    ///  the style and `0`/`1` represent the on/off triggers.
    ///
    /// `.vtt` supports HTML-like tags for `i`,`b`,`u`, representing italic, bold, underline.
    ///
    /// If found, ssa specific triggers for those supported tags are replaced with their `.vtt` alternatives.
    ///
    /// In addition, if an SSAEvent has a related SSAStyle, the SSAStyle is converted to a VTTStyle that will be wrapped around the lines indicating it.
    pub fn to_vtt(self) -> VTTFile {
        let mut a = VTTFile::default();
        a.lines.clear();
        let regex =
            Regex::new(r"(?P<main>\{\\(?P<type>.)(?P<trigger>.*?)\})").expect("Regex broke");
        let mut stylctr = 1;
        for i in self.styles {
            let styl = VTTStyle {
                color: color::ColorType::VTTColor0A(i.firstcolor.get_color()),
                font_family: format!("\"{}\"", i.fontname),
                name: Some(i.name.replace(' ', "")),
                font_size: i.fontsize.to_string() + "px",
                background_color: color::ColorType::VTTColor(i.backgroundcolor.get_color()),
                ..Default::default()
            };
            if stylctr == 1 {
                stylctr += 1;
                a.styles.clear();
            }
            a.styles.push(styl);
        }
        for (i, j) in self.events.iter().enumerate() {
            let mut line = VTTLine {
                line_number: (i + 1).to_string(),
                style: Some(j.style.to_string().replace(' ', "")),
                position: None,
                line_start: j.line_start.clone(),
                line_end: j.line_end.clone(),
                line_text: "".to_string(),
            };
            line.line_text = j.line_text.replace("\\N", "\r\n");

            for k in regex.captures_iter(&line.line_text.clone()) {
                let tag_type = k.name("type").unwrap().as_str();
                let tag_main = k.name("main").unwrap().as_str();
                let tag_trigger = k.name("trigger").unwrap().as_str();
                if tag_type.chars().all(|x| ['b', 'i', 'u'].contains(&x)) {
                    if tag_trigger == "0" {
                        line.line_text = line
                            .line_text
                            .replace(tag_main, &("</".to_string() + tag_type + ">"));
                    } else if tag_trigger == "1" {
                        line.line_text = line
                            .line_text
                            .replace(tag_main, &("<".to_string() + tag_type + ">"));
                    }
                } else {
                    line.line_text = line.line_text.replace(tag_main, "");
                }
            }
            line.line_text = "<".to_string()
                + &line.clone().style.unwrap().to_string()
                + ">"
                + &line.clone().line_text
                + "</"
                + &line.clone().style.unwrap().to_string()
                + ">";
            a.lines.push(line);
        }
        a
    }

    /// Writes the SSAFile to a file specified by a path String.
    pub fn to_file<P: AsRef<Path>>(self, path: P) -> std::io::Result<()> {
        let mut w = File::options()
            .create(true)
            .write(true)
            .open(path)
            .expect("File can't be created");
        write!(w, "{self}")?;
        Ok(())
    }
}

impl Display for SSAFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "[Script Info]\r\n".to_string();
        for (i, j) in self.info.clone() {
            str += &format!("{i}: {j}\r\n").to_string();
        }
        str += "\r\n[V4+ Styles]\r\nFormat: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding\r\n";
        for i in self.styles.clone() {
            str += &("Style: ".to_string()
                + &i.name
                + ","
                + &i.fontname
                + ","
                + &i.fontsize.to_string()
                + ","
                + &i.firstcolor.to_string()
                + ","
                + &i.secondcolor.to_string()
                + ","
                + &i.outlinecolor.to_string()
                + ","
                + &i.backgroundcolor.to_string()
                + ","
                + &i.bold
                    .then(|| "-1".to_string())
                    .or_else(|| Some("0".to_string()))
                    .expect("Proper")
                + ","
                + &i.italic
                    .then(|| "-1".to_string())
                    .or_else(|| Some("0".to_string()))
                    .expect("Proper")
                + ","
                + &i.underline
                    .then(|| "-1".to_string())
                    .or_else(|| Some("0".to_string()))
                    .expect("Proper")
                + ","
                + &i.strikeout
                    .then(|| "-1".to_string())
                    .or_else(|| Some("0".to_string()))
                    .expect("Proper")
                + ","
                + &(i.scalex as i32).to_string()
                + ","
                + &(i.scaley as i32).to_string()
                + ","
                + &(i.spacing as i32).to_string()
                + ","
                + &(i.angle as i32).to_string()
                + ","
                + &i.borderstyle.to_string()
                + ","
                + &(i.outline as i32).to_string()
                + ","
                + &(i.shadow as i32).to_string()
                + ","
                + &(i.alignment as i32).to_string()
                + ","
                + &format!("{:0>4}", i.lmargin.to_string())
                + ","
                + &format!("{:0>4}", i.rmargin.to_string())
                + ","
                + &format!("{:0>4}", i.vmargin.to_string())
                + ","
                + &i.encoding.to_string()
                + "\r\n");
        }

        str += "\r\n[Events]\r\nFormat: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text\r\n";
        for i in self.events.clone() {
            str += &(i.linetype
                + ": "
                + &i.layer.to_string()
                + ","
                + &i.line_start.to_ass_string()
                + ","
                + &i.line_end.to_ass_string()
                + ","
                + &i.style
                + ","
                + &i.name
                + ","
                + &format!("{:0>4}", i.lmargin.to_string())
                + ","
                + &format!("{:0>4}", i.rmargin.to_string())
                + ","
                + &format!("{:0>4}", i.vmargin.to_string())
                + ","
                + &i.effect
                + ","
                + &i.line_text
                + "\r\n");
        }
        write!(f, "{str}")
    }
}

impl FromStr for SSAFile {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path_or_content = s.to_string();
        let mut b: String = "".to_string();
        let mut sub: SSAFile = SSAFile::default();
        if !path_or_content.contains('\n') {
            if std::fs::read(&path_or_content).is_ok() {
                let mut f = File::open(path_or_content)?;
                f.read_to_string(&mut b)?;
            }
        } else {
            b = path_or_content;
        }
        let (split, ssplit) = if b.split("\r\n\r\n").count() < 2 {
            ("\n\n", "\n")
        } else {
            ("\r\n\r\n", "\r\n")
        };
        let c: Vec<&str> = b.split(split).collect();
        for i in c {
            if i.contains("Styles]") {
                sub.styles.clear();
                let mut style: HashMap<String, Vec<&str>> = HashMap::new();
                let keys = i
                    .split(ssplit)
                    .filter(|x| x.starts_with("Format:"))
                    .collect::<String>();
                let fmtheaders = keys.strip_prefix("Format: ").unwrap().replace(' ', "");
                let finalheaders = fmtheaders.split(',').collect::<Vec<&str>>();
                style.insert("Format".to_string(), finalheaders);

                let keys2 = i
                    .split('\n')
                    .filter(|&x| x.starts_with("Style: "))
                    .map(|x| {
                        <&str>::clone(
                            x.strip_prefix("Style: ")
                                .unwrap()
                                .split(',')
                                .collect::<Vec<&str>>()
                                .first()
                                .unwrap(),
                        )
                    })
                    .collect::<Vec<&str>>();
                let values2 = i
                    .split(ssplit)
                    .filter(|&x| x.starts_with("Style: "))
                    .map(|x| x.strip_prefix("Style: ").unwrap().borrow())
                    .collect::<Vec<&str>>();
                for (i, j) in keys2.into_iter().enumerate() {
                    style.insert(
                        j.to_string(),
                        values2.get(i).unwrap().split(',').collect::<Vec<&str>>(),
                    );
                }
                for (k, l) in style.clone().into_iter() {
                    if k == *"Format" {
                        continue;
                    }
                    let styl = SSAStyle {
                        name: l.first().expect("missing_name").to_string(),
                        fontname: l.get(1).expect("missing_name").to_string(),
                        fontsize: l
                            .get(2)
                            .expect("missing_name")
                            .to_string()
                            .parse::<f32>()
                            .expect("msg"),
                        firstcolor: color::ColorType::SSAColor(
                            Color::from_str(l.get(3).expect("missing_name")).expect("msg"),
                        ),
                        secondcolor: color::ColorType::SSAColor(
                            Color::from_str(l.get(4).expect("missing_name")).expect("msg"),
                        ),
                        outlinecolor: color::ColorType::SSAColor(
                            Color::from_str(l.get(5).expect("missing_name")).expect("msg"),
                        ),
                        backgroundcolor: color::ColorType::SSAColor(
                            Color::from_str(l.get(6).expect("missing_name")).expect("msg"),
                        ),
                        bold: l.get(7).expect("missing value") == &"-1",
                        italic: l.get(8).expect("missing value") == &"-1",
                        underline: l.get(9).expect("missing value") == &"-1",
                        strikeout: l.get(10).expect("missing value") == &"-1",
                        scalex: l
                            .get(11)
                            .expect("Not provided ScaleX")
                            .parse::<f32>()
                            .expect("ScaleX value not proper"),
                        scaley: l
                            .get(12)
                            .expect("Not provided ScaleY")
                            .parse::<f32>()
                            .expect("ScaleY value not proper"),
                        spacing: l
                            .get(13)
                            .expect("Not provided Spacing")
                            .parse::<f32>()
                            .expect("Spacing value not proper"),
                        angle: l
                            .get(14)
                            .expect("Not provided Spacing")
                            .parse::<f32>()
                            .expect("Spacing value not proper"),
                        borderstyle: l
                            .get(15)
                            .expect("Not provided borderstyle")
                            .parse::<i8>()
                            .expect("borderstyle value not proper"),
                        outline: l
                            .get(16)
                            .expect("Not provided Spacing")
                            .parse::<f32>()
                            .expect("Spacing value not proper"),
                        shadow: l
                            .get(17)
                            .expect("Not provided Spacing")
                            .parse::<f32>()
                            .expect("Spacing value not proper"),
                        alignment: Alignment::infer_from_str(
                            l.get(18).expect("Not provided Spacing"),
                        )
                        .unwrap(),
                        lmargin: l
                            .get(19)
                            .expect("Not provided lmargin")
                            .parse::<i32>()
                            .expect("lmargin value not proper"),
                        rmargin: l
                            .get(20)
                            .expect("Not provided rmargin")
                            .parse::<i32>()
                            .expect("rmargin value not proper"),
                        vmargin: l
                            .get(21)
                            .expect("Not provided vmargin")
                            .parse::<i32>()
                            .expect("vmargin value not proper"),
                        alpha: 0,
                        encoding: l
                            .get(22)
                            .expect("Not provided encoding")
                            .parse::<i32>()
                            .expect("encoding value not proper"),
                        drawing: false,
                    };
                    sub.styles.push(styl);
                }
            }
            if i.contains("[Script Info]") {
                sub.info.clear();
                for j in i.split(ssplit).collect::<Vec<&str>>().iter() {
                    let line = j.split_once(':').unwrap_or(("", ""));
                    sub.info
                        .insert(line.0.to_string(), line.1.trim().to_string());
                }
                sub.info.remove("");
                if !sub.info.contains_key("ScaledBorderAndShadows") {
                    sub.info
                        .insert("ScaledBorderAndShadows".to_string(), "yes".to_string());
                }
            }
            if i.contains("[Events]") {
                sub.events.clear();
                for j in i.split(ssplit) {
                    if j.starts_with("Dialogue:") {
                        let mut ev = SSAEvent::default();
                        let line = j
                            .strip_prefix("Dialogue: ")
                            .unwrap()
                            .splitn(10, ',')
                            .collect::<Vec<&str>>();
                        ev.layer = line
                            .first()
                            .unwrap()
                            .parse::<i32>()
                            .expect("Failed to parse layer");
                        ev.line_start = Time::from_str(line.get(1).unwrap()).unwrap();
                        ev.line_end = Time::from_str(line.get(2).unwrap()).unwrap();
                        ev.style = line.get(3).unwrap().to_string();
                        ev.name = line.get(4).unwrap().to_string();
                        ev.lmargin = line
                            .get(5)
                            .unwrap()
                            .to_string()
                            .parse::<f32>()
                            .expect("couldn't conv to float");
                        ev.rmargin = line
                            .get(6)
                            .unwrap()
                            .to_string()
                            .parse::<f32>()
                            .expect("couldn't conv to float");
                        ev.vmargin = line
                            .get(7)
                            .unwrap()
                            .to_string()
                            .parse::<f32>()
                            .expect("couldn't conv to float");
                        ev.effect = line.get(8).unwrap().to_string();
                        ev.line_text = line.get(9).unwrap().to_string();
                        sub.events.push(ev);
                    }
                }
            }
        }
        Ok(sub)
    }
}

/// Parses the given [String] into a [SSAFile].
pub fn parse(content: String) -> SSAFile {
    let mut sub: SSAFile = SSAFile::default();
    let (split, ssplit) = if content.split("\r\n\r\n").count() < 2 {
        ("\n\n", "\n")
    } else {
        ("\r\n\r\n", "\r\n")
    };
    let c: Vec<&str> = content.split(split).collect();
    for i in c {
        if i.contains("Styles]") {
            sub.styles.clear();
            let mut style: HashMap<String, Vec<&str>> = HashMap::new();
            let keys = i
                .split(ssplit)
                .filter(|x| x.starts_with("Format:"))
                .collect::<String>();
            let fmtheaders = keys.strip_prefix("Format: ").unwrap().replace(' ', "");
            let finalheaders = fmtheaders.split(',').collect::<Vec<&str>>();
            style.insert("Format".to_string(), finalheaders);

            let keys2 = i
                .split('\n')
                .filter(|&x| x.starts_with("Style: "))
                .map(|x| {
                    <&str>::clone(
                        x.strip_prefix("Style: ")
                            .unwrap()
                            .split(',')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap(),
                    )
                })
                .collect::<Vec<&str>>();
            let values2 = i
                .split(ssplit)
                .filter(|&x| x.starts_with("Style: "))
                .map(|x| x.strip_prefix("Style: ").unwrap().borrow())
                .collect::<Vec<&str>>();
            for (i, j) in keys2.into_iter().enumerate() {
                style.insert(
                    j.to_string(),
                    values2.get(i).unwrap().split(',').collect::<Vec<&str>>(),
                );
            }
            for (k, l) in style.clone().into_iter() {
                if k == *"Format" {
                    continue;
                }
                let styl = SSAStyle {
                    name: l.first().expect("missing_name").to_string(),
                    fontname: l.get(1).expect("missing_name").to_string(),
                    fontsize: l
                        .get(2)
                        .expect("missing_name")
                        .to_string()
                        .parse::<f32>()
                        .expect("msg"),
                    firstcolor: color::ColorType::SSAColor(
                        Color::from_str(l.get(3).expect("missing_name")).expect("msg"),
                    ),
                    secondcolor: color::ColorType::SSAColor(
                        Color::from_str(l.get(4).expect("missing_name")).expect("msg"),
                    ),
                    outlinecolor: color::ColorType::SSAColor(
                        Color::from_str(l.get(5).expect("missing_name")).expect("msg"),
                    ),
                    backgroundcolor: color::ColorType::SSAColor(
                        Color::from_str(l.get(6).expect("missing_name")).expect("msg"),
                    ),
                    bold: l.get(7).expect("missing value") == &"-1",
                    italic: l.get(8).expect("missing value") == &"-1",
                    underline: l.get(9).expect("missing value") == &"-1",
                    strikeout: l.get(10).expect("missing value") == &"-1",
                    scalex: l
                        .get(11)
                        .expect("Not provided ScaleX")
                        .parse::<f32>()
                        .expect("ScaleX value not proper"),
                    scaley: l
                        .get(12)
                        .expect("Not provided ScaleY")
                        .parse::<f32>()
                        .expect("ScaleY value not proper"),
                    spacing: l
                        .get(13)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper"),
                    angle: l
                        .get(14)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper"),
                    borderstyle: l
                        .get(15)
                        .expect("Not provided borderstyle")
                        .parse::<i8>()
                        .expect("borderstyle value not proper"),
                    outline: l
                        .get(16)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper"),
                    shadow: l
                        .get(17)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper"),
                    alignment: Alignment::infer_from_str(l.get(18).expect("Not provided Spacing"))
                        .unwrap(),
                    lmargin: l
                        .get(19)
                        .expect("Not provided lmargin")
                        .parse::<i32>()
                        .expect("lmargin value not proper"),
                    rmargin: l
                        .get(20)
                        .expect("Not provided rmargin")
                        .parse::<i32>()
                        .expect("rmargin value not proper"),
                    vmargin: l
                        .get(21)
                        .expect("Not provided vmargin")
                        .parse::<i32>()
                        .expect("vmargin value not proper"),
                    alpha: 0,
                    encoding: l
                        .get(22)
                        .expect("Not provided encoding")
                        .parse::<i32>()
                        .expect("encoding value not proper"),
                    drawing: false,
                };
                sub.styles.push(styl);
            }
        }
        if i.contains("[Script Info]") {
            sub.info.clear();
            for j in i.split(ssplit).collect::<Vec<&str>>().iter() {
                let line = j.split_once(':').unwrap_or(("", ""));
                sub.info
                    .insert(line.0.to_string(), line.1.trim().to_string());
            }
            sub.info.remove("");
            if !sub.info.contains_key("ScaledBorderAndShadows") {
                sub.info
                    .insert("ScaledBorderAndShadows".to_string(), "yes".to_string());
            }
        }
        if i.contains("[Events]") {
            sub.events.clear();
            for j in i.split(ssplit) {
                if j.starts_with("Dialogue:") {
                    let mut ev = SSAEvent::default();
                    let line = j
                        .strip_prefix("Dialogue: ")
                        .unwrap()
                        .splitn(10, ',')
                        .collect::<Vec<&str>>();
                    ev.layer = line
                        .first()
                        .unwrap()
                        .parse::<i32>()
                        .expect("Failed to parse layer");
                    ev.line_start = Time::from_str(line.get(1).unwrap()).unwrap();
                    ev.line_end = Time::from_str(line.get(2).unwrap()).unwrap();
                    ev.style = line.get(3).unwrap().to_string();
                    ev.name = line.get(4).unwrap().to_string();
                    ev.lmargin = line
                        .get(5)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.rmargin = line
                        .get(6)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.vmargin = line
                        .get(7)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.effect = line.get(8).unwrap().to_string();
                    ev.line_text = line.get(9).unwrap().to_string();
                    sub.events.push(ev);
                }
            }
        }
    }
    sub
}

/// Parses the given [Path] into a [SSAFile].
pub fn parse_from_file<P: AsRef<Path>>(file: P) -> Result<SSAFile, std::io::Error> {
    Ok(parse(fs::read_to_string(file)?))
}
