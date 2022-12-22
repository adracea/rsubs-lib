use core::panic;
use std::{
    borrow::Borrow,
    collections::HashMap,
    io::{Read, Write},
    str::FromStr,
};

use crate::util::{
    color::{self, Alignment, Color},
    time::{time_from_string, Time},
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;

use super::srt::SRTLine;
use super::{srt::SRTFile, vtt::VTTFile, vtt::VTTLine, vtt::VTTStyle};

#[derive(Debug)]
pub struct SSAStyle {
    pub name: String,
    pub fontname: String,
    pub fontsize: f32,
    pub firstcolor: color::Color,
    pub secondcolor: color::Color,
    pub outlinecolor: color::Color,
    pub backgroundcolor: color::Color,
    pub bold: bool,
    pub italic: bool,
    pub unerline: bool,
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

impl Default for SSAStyle {
    fn default() -> Self {
        SSAStyle {
            name: "Default".to_string(),
            fontname: "Trebuchet MS".to_string(),
            fontsize: 25.5,
            firstcolor: color::WHITET,
            secondcolor: color::TRANSPARENT,
            outlinecolor: color::TRANSPARENT,
            backgroundcolor: color::TRANSPARENT,
            bold: false,
            italic: true,
            unerline: true,
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

#[derive(Debug, Clone)]
pub struct SSAEvent {
    pub layer: i32,
    pub line_start: Time,
    pub line_end: Time,
    pub style: String,
    pub name: String,
    pub lmargin: f32,
    pub rmargin: f32,
    pub vmargin: f32,
    pub effect: String,
    pub linetype: String,
    pub line_text: String,
}

impl Default for SSAEvent {
    fn default() -> Self {
        SSAEvent {
            layer: 0,
            line_start: time_from_string("0:00:00.20".to_string()),
            line_end: time_from_string("0:00:02.20".to_string()),
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

pub static OVERRIDE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{[^}]*\}").expect("Timestamp regex failure"));

#[derive(Debug)]
pub struct SSAFile {
    pub events: Vec<SSAEvent>,
    pub styles: Vec<SSAStyle>,
    pub info: HashMap<String, String>,
    pub format: String,
}
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

impl SSAFile {
    pub fn to_srt(&self) -> SRTFile {
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
    pub fn to_vtt(self) -> VTTFile {
        let mut a = VTTFile::default();
        a.lines.clear();
        let regex =
            Regex::new(r"(?P<main>\{\\(?P<type>.)(?P<trigger>.*?)\})").expect("Regex broke");
        let mut stylctr = 1;
        for i in self.styles {
            let styl = VTTStyle {
                color: i.firstcolor,
                font_family: format!("\"{}\"", i.fontname),
                name: Some(i.name.replace(' ', "")),
                font_size: i.fontsize.to_string() + "px",
                background_color: i.backgroundcolor,
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
    pub fn to_file(self, path: String) -> std::io::Result<()> {
        let mut w = File::options()
            .write(true)
            .create(true)
            .open(path)
            .expect("File can't be created");
        let mut str = "[Script Info]\r\n".to_string();
        for (i, j) in self.info {
            str += &format!("{}: {}\r\n", i, j).to_string();
        }
        str += "\r\n[V4+ Styles]\r\nFormat: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding\r\n";
        for i in self.styles {
            str += &("Style: ".to_string()
                + &i.name
                + ","
                + &i.fontname
                + ","
                + &i.fontsize.to_string()
                + ","
                + &i.firstcolor.fmt_ass()
                + ","
                + &i.secondcolor.fmt_ass()
                + ","
                + &i.outlinecolor.fmt_ass()
                + ","
                + &i.backgroundcolor.fmt_ass()
                + ","
                + &i.bold
                    .then(|| "0".to_string())
                    .or_else(|| Some("-1".to_string()))
                    .expect("Proper")
                + ","
                + &i.italic
                    .then(|| "0".to_string())
                    .or_else(|| Some("-1".to_string()))
                    .expect("Proper")
                + ","
                + &i.unerline
                    .then(|| "0".to_string())
                    .or_else(|| Some("-1".to_string()))
                    .expect("Proper")
                + ","
                + &i.strikeout
                    .then(|| "0".to_string())
                    .or_else(|| Some("-1".to_string()))
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
        for i in self.events {
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
        w.write_all(str.as_bytes()).expect("Couldn't write");
        Ok(())
    }
}

pub fn parse(path_or_content: String) -> Result<SSAFile, std::io::Error> {
    let mut b: String = "".to_string();
    let mut sub: SSAFile = SSAFile::default();
    if std::fs::read(&path_or_content).is_ok() {
        let mut f = File::open(path_or_content)?;
        f.read_to_string(&mut b)?;
    } else {
        b = path_or_content;
    }
    let c: Vec<&str> = b.split("\r\n\r\n").collect();
    for i in c {
        if i.contains("Styles]") {
            sub.styles.clear();
            let mut style: HashMap<String, Vec<&str>> = HashMap::new();
            let keys = i
                .split("\r\n")
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
                .split("\r\n")
                .filter(|&x| x.starts_with("Style: "))
                .map(|x| x.strip_prefix("Style: ").unwrap().borrow())
                .collect::<Vec<&str>>();
            for (i, j) in keys2.into_iter().enumerate() {
                style.insert(
                    j.to_string(),
                    values2.get(i).unwrap().split(',').collect::<Vec<&str>>(),
                );
            }
            // for _ in (&style.clone().get(&"Format".to_string()).unwrap()).into_iter() {}
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
                    firstcolor: Color::from_str(l.get(3).expect("missing_name")).expect("msg"),
                    secondcolor: Color::from_str(l.get(4).expect("missing_name")).expect("msg"),
                    outlinecolor: Color::from_str(l.get(5).expect("missing_name")).expect("msg"),
                    backgroundcolor: Color::from_str(l.get(6).expect("missing_name")).expect("msg"),
                    bold: l.get(7).expect("missing value") == &"-1",
                    italic: l.get(8).expect("missing value") == &"-1",
                    unerline: l.get(9).expect("missing value") == &"-1",
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
            for j in i.split("\r\n").collect::<Vec<&str>>().iter() {
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
            for j in i.split("\r\n") {
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
                    ev.line_start = time_from_string(line.get(1).unwrap().to_string());
                    ev.line_end = time_from_string(line.get(2).unwrap().to_string());
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
    if true {
        Ok(sub)
    } else {
        panic!("test")
    }
}
