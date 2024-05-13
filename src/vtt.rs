//! Implements helpers for `.vtt`.
//!
//! It describes the [VTTStyle], [VTT] and [VTTLine] structs and
//! provides the [parse] function.

use super::srt::{SRTLine, SRT};
use super::ssa::{SSAEvent, SSAInfo, SSAStyle, SSA};
use crate::error;
use crate::util::{Alignment, Color, BLACK, TRANSPARENT, WHITE};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use time::Time;

/// The VTTStyle contains information that generally composes the `::cue` header
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct VTTStyle {
    pub selector: Option<String>,
    pub entries: HashMap<String, String>,
}

/// The VTTLine contains information about the line itself as well as the positional information of the line
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct VTTLine {
    pub identifier: Option<String>,
    pub start: Time,
    pub end: Time,
    pub settings: HashMap<String, Option<String>>,
    pub text: String,
}

impl Default for VTTLine {
    fn default() -> Self {
        Self {
            identifier: None,
            start: Time::from_hms(0, 0, 0).unwrap(),
            end: Time::from_hms(0, 0, 0).unwrap(),
            settings: Default::default(),
            text: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VTTRegion {
    pub id: Option<String>,
    pub width: Option<f32>,
    pub lines: Option<u32>,
    pub region_anchor: Option<(f32, f32)>,
    pub viewport_anchor: Option<(f32, f32)>,
    pub scroll: bool,
}
impl Eq for VTTRegion {}

/// Contains [VTTStyle]s and [VTTLine]s
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct VTT {
    pub regions: Vec<VTTRegion>,
    pub styles: Vec<VTTStyle>,
    pub lines: Vec<VTTLine>,
}

impl VTT {
    /// Parses the given [String] into as [VTT].
    pub fn parse<S: AsRef<str>>(content: S) -> Result<VTT, VTTError> {
        let mut line_num = 0;

        let mut regions = vec![];
        let mut styles = vec![];
        let mut lines = vec![];

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

        parse::parse_start(blocks.remove(0).into_iter())
            .map_err(|e| VTTError::new(e.kind, line_num + e.line))?;

        line_num += 1;
        for mut block in blocks {
            line_num += 1;

            let block_len = block.len();
            let (first_word, _) = block[0].split_once(' ').unwrap_or((block[0], ""));

            match first_word {
                // parsing the 'NOTE' block is very easy, but it cannot be useful represented how the
                // VTT struct is structured, so it gets just skipped
                "NOTE" => (),
                "REGION" => {
                    block.remove(0);
                    line_num += 1;
                    regions.push(
                        parse::parse_region_block(block.into_iter())
                            .map_err(|e| VTTError::new(e.kind, line_num + e.line))?,
                    )
                }
                "STYLE" => {
                    block[0] = &block[0][5..];
                    styles.push(
                        parse::parse_style_block(block.join("\n").trim())
                            .map_err(|e| VTTError::new(e.kind, line_num + e.line))?,
                    );
                }
                _ => lines.push(
                    parse::parse_cue_block(block.into_iter())
                        .map_err(|e| VTTError::new(e.kind, line_num + e.line))?,
                ),
            }

            line_num += block_len
        }

        Ok(VTT {
            regions,
            styles,
            lines,
        })
    }

    /// When converting to SSAFile, information about the VTTStyles is maintained but not applied.
    pub fn to_ssa(&self) -> SSA {
        let speaker_regex: Regex = Regex::new(r"(?m)^<v.*?\s(?P<speaker>.*?)>").unwrap();
        let xml_replace_regex: Regex = Regex::new(r"(?m)<.*?>").unwrap();

        let mut default_style = SSAStyle {
            name: "Default".to_string(),
            fontname: "Arial".to_string(),
            fontsize: 20.0,
            primary_color: WHITE,
            secondary_color: BLACK,
            outline_color: TRANSPARENT,
            back_color: TRANSPARENT,
            alignment: Alignment::BottomCenter,
            ..Default::default()
        };
        for style in &self.styles {
            // style settings that doesn't apply for whole lines cannot be represented as SSAStyle
            if style.selector.is_some() {
                continue;
            }
            // text color. skips if the VTT color can't be read
            if let Some(color) = style.entries.get("color") {
                if let Ok(primary_color) = Color::from_vtt(color) {
                    default_style.primary_color = primary_color
                }
            }
            // background color. skips if the VTT color can't be read
            if let Some(background_color) = style.entries.get("background-color") {
                if let Ok(back_color) = Color::from_vtt(background_color) {
                    default_style.back_color = back_color
                }
            }
            // font size. can only be converted to SSA if it is given as pixels, in all other
            // cases it will be skipped
            if let Some(font_size) = style.entries.get("font-size") {
                let font_size = font_size.trim_end_matches("px");
                if let Ok(font_size) = font_size.parse() {
                    default_style.fontsize = font_size
                }
            }
            // italic text
            if style
                .entries
                .get("font-style")
                .is_some_and(|fs| fs == "italic")
            {
                default_style.italic = true;
            }
            // bold text
            if style
                .entries
                .get("font-weight")
                .is_some_and(|fw| fw.starts_with("bold"))
            {
                default_style.bold = true;
            }
            // underline & strikeout
            if let Some(text_decoration) = style.entries.get("text-decoration") {
                if text_decoration.contains("underline") {
                    default_style.underline = true
                }
                if text_decoration.contains("line-through") {
                    default_style.strikeout = true
                }
            }
            // spacing between characters. can only be converted to SSA if it is given as pixels, in
            // all other cases it will be skipped
            if let Some(letter_spacing) = style.entries.get("letter-spacing") {
                let letter_spacing = letter_spacing.trim_end_matches("px");
                if let Ok(letter_spacing) = letter_spacing.parse() {
                    default_style.spacing = letter_spacing
                }
            }
        }

        let mut events = vec![];
        for line in &self.lines {
            let mut captures = speaker_regex.captures_iter(&line.text);
            let first_capture = captures.next();
            let second_capture = captures.next();

            let (mut text, speaker) = if first_capture.is_some() && second_capture.is_some() {
                (speaker_regex.replace_all(&line.text, "").to_string(), None)
            } else if let Some(c) = first_capture {
                let text = line.text[c.get(0).unwrap().end()..].to_string();
                let speaker = c.name("speaker").unwrap().as_str().to_string();
                (text, Some(speaker))
            } else {
                (line.text.clone(), None)
            };

            text = text
                .replace("<b>", "{\\b1}")
                .replace("</b>", "{\\b0}")
                .replace("<i>", "{\\i1}")
                .replace("</i>", "{\\i0}")
                .replace("<s>", "{\\s1}")
                .replace("</s>", "{\\s0}")
                .replace("<u>", "{\\u1}")
                .replace("</u>", "{\\u0}");
            text = xml_replace_regex.replace_all(&text, "").to_string();

            events.push(SSAEvent {
                start: line.start,
                end: line.end,
                style: "Default".to_string(),
                name: speaker.unwrap_or_default(),
                text: text.replace("\r\n", "\\N").replace('\n', "\\N"),
                ..Default::default()
            })
        }

        SSA {
            info: SSAInfo {
                ..Default::default()
            },
            styles: vec![default_style],
            events,
            fonts: vec![],
            graphics: vec![],
        }
    }
    /// SRT is basically a VTT without the styles
    pub fn to_srt(&self) -> SRT {
        let speaker_regex: Regex = Regex::new(r"(?m)^<v.*?>").unwrap();

        let mut lines = vec![];

        for (i, line) in self.lines.iter().enumerate() {
            let text = speaker_regex
                .replace_all(line.text.as_str(), "")
                .to_string();

            lines.push(SRTLine {
                sequence_number: i as u32 + 1,
                start: line.start,
                end: line.end,
                text,
            })
        }

        SRT { lines }
    }
}

impl Display for VTT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks = vec![];

        blocks.push(vec!["WEBVTT".to_string()]);

        for style in &self.styles {
            let mut block = vec![];
            block.push("STYLE".to_string());
            if let Some(selector) = &style.selector {
                block.push(format!("::cue({selector}) {{"))
            } else {
                block.push("::cue {".to_string())
            }
            for (id, value) in &style.entries {
                block.push(format!("{id}: {value}"))
            }
            block.push("}".to_string());

            blocks.push(block)
        }

        for line in &self.lines {
            let mut block = vec![];
            if let Some(identifier) = &line.identifier {
                block.push(identifier.clone())
            }

            if !line.settings.is_empty() {
                block.push(format!(
                    "{} --> {} {}",
                    line.start.format(parse::TIME_FORMAT).unwrap(),
                    line.end.format(parse::TIME_FORMAT).unwrap(),
                    line.settings
                        .iter()
                        .map(|(k, v)| v.as_ref().map_or(k.clone(), |v| format!("{k}: {v}")))
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
            } else {
                block.push(format!(
                    "{} --> {}",
                    line.start.format(parse::TIME_FORMAT).unwrap(),
                    line.end.format(parse::TIME_FORMAT).unwrap()
                ))
            }
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

mod parse {
    use super::*;
    use time::format_description::BorrowedFormatItem;
    use time::macros::format_description;

    pub(super) struct Error {
        pub(super) line: usize,
        pub(super) kind: VTTErrorKind,
    }

    pub(super) const TIME_FORMAT: &[BorrowedFormatItem] =
        format_description!("[hour]:[minute]:[second].[subsecond digits:3]");

    type Result<T> = std::result::Result<T, Error>;

    pub(super) fn parse_start<'a, I: Iterator<Item = &'a str>>(mut block_lines: I) -> Result<()> {
        let line = block_lines.next().unwrap();
        if !line.starts_with("WEBVTT") || block_lines.next().is_some() {
            return Err(Error {
                line: 1,
                kind: VTTErrorKind::InvalidFormat,
            });
        }
        Ok(())
    }
    pub(super) fn parse_region_block<'a, I: Iterator<Item = &'a str>>(
        block_lines: I,
    ) -> Result<VTTRegion> {
        let mut region = VTTRegion {
            id: None,
            width: None,
            lines: None,
            region_anchor: None,
            viewport_anchor: None,
            scroll: false,
        };

        for (i, line) in block_lines.enumerate() {
            let (name, value) = line.split_once(':').ok_or(Error {
                line: 1 + i,
                kind: VTTErrorKind::Parse("delimiter ':' missing".to_string()),
            })?;

            match name {
                "id" => region.id = Some(value.to_string()),
                "width" => {
                    region.width = Some(parse_percentage(value).ok_or(Error {
                        line: 1 + i,
                        kind: VTTErrorKind::Parse(format!("invalid percentage '{value}'")),
                    })?)
                }
                "lines" => {
                    region.lines = Some(value.parse::<u32>().map_err(|e| Error {
                        line: 1 + i,
                        kind: VTTErrorKind::Parse(e.to_string()),
                    })?)
                }
                "regionanchor" => {
                    let Some((a, b)) = value.split_once(',') else {
                        return Err(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse("delimiter ',' missing".to_string()),
                        });
                    };
                    region.region_anchor = Some((
                        parse_percentage(a).ok_or(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse(format!("invalid percentage '{value}'")),
                        })?,
                        parse_percentage(b).ok_or(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse(format!("invalid percentage '{value}'")),
                        })?,
                    ))
                }
                "viewportanchor" => {
                    let Some((a, b)) = value.split_once(',') else {
                        return Err(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse("delimiter ',' missing".to_string()),
                        });
                    };
                    region.viewport_anchor = Some((
                        parse_percentage(a).ok_or(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse(format!("invalid percentage '{value}'")),
                        })?,
                        parse_percentage(b).ok_or(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse(format!("invalid percentage '{value}'")),
                        })?,
                    ))
                }
                "scroll" => {
                    region.scroll = if value == "up" {
                        true
                    } else {
                        return Err(Error {
                            line: 1 + i,
                            kind: VTTErrorKind::Parse("only allowed value is 'up'".to_string()),
                        });
                    }
                }
                _ => continue,
            }
        }

        Ok(region)
    }
    pub(super) fn parse_style_block(block: &str) -> Result<VTTStyle> {
        let mut selector = None;
        let mut entries = HashMap::new();

        // check for `::cue` prefix
        let Some(mut block) = block.strip_prefix("::cue") else {
            return Err(Error {
                line: 1,
                kind: VTTErrorKind::Parse("missing '::cue' prefix".to_string()),
            });
        };

        // check if block ends with curly bracket
        if block.ends_with('}') {
            block = &block[..block.len() - 1]
        } else {
            return Err(Error {
                line: block.split('\n').count(),
                kind: VTTErrorKind::Parse("missing '}' suffix".to_string()),
            });
        }

        // extract selector in brackets if existent
        block = block.trim_start();
        if block.starts_with('(') {
            let Some(closing_idx) = block.find(|c| c == ')') else {
                return Err(Error {
                    line: 1,
                    kind: VTTErrorKind::Parse("selector isn't closed".to_string()),
                });
            };
            selector = Some(block[1..closing_idx].to_string());
            block = &block[closing_idx + 1..]
        }

        // check for open curly brace
        let Some(mut block) = block.trim_start().strip_prefix('{') else {
            return Err(Error {
                line: 1,
                kind: VTTErrorKind::Parse("missing '{'".to_string()),
            });
        };

        let mut line_num = 0;
        // a newline might occur here
        if block.starts_with('\n') {
            line_num += 1;
            block = &block[1..];
        }

        for line in block.split('\n') {
            line_num += 1;

            for item in line.split(';') {
                if item.is_empty() {
                    continue;
                }

                let Some((name, value)) = item.split_once(':') else {
                    return Err(Error {
                        line: 1 + line_num,
                        kind: VTTErrorKind::Parse("delimiter ':' missing".to_string()),
                    });
                };
                entries.insert(name.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(VTTStyle { selector, entries })
    }
    pub(super) fn parse_cue_block<'a, I: Iterator<Item = &'a str>>(
        mut block_lines: I,
    ) -> Result<VTTLine> {
        let mut identifier = None;
        let mut settings = HashMap::new();

        // extracts the first line, which is either an identifier or the start & end times (but the
        // variable is called 'timing_line' for convenience)
        let mut timing_line = block_lines.next().unwrap();
        // check if the first line contains an identifier instead of the start & end times
        if !timing_line.contains("-->") {
            identifier = Some(timing_line.to_string());
            timing_line = block_lines.next().ok_or(Error {
                line: 2,
                kind: VTTErrorKind::Parse("missing subtitle timing".to_string()),
            })?;
        }

        // split the line at '-->'. the first item contains only a timestamp, the second item
        // contains a timestamp + an optional list of settings for this cue block
        let (start_str, mut end_str) = timing_line.split_once("-->").ok_or(Error {
            line: 1 + identifier.is_some() as usize,
            kind: VTTErrorKind::Parse("missing '-->'".to_string()),
        })?;
        // get the start time. because the parse functionality of the `time` crate isn't capable of
        // parsing optional literals or templates that only contains minutes, seconds and subseconds
        // the hour part must be prepended if not existent
        let start = if start_str.chars().filter(|c| *c == ':').count() < 2 {
            let start_str = format!("00:{}", start_str.trim());
            Time::parse(&start_str, TIME_FORMAT).map_err(|e| Error {
                line: 1 + identifier.is_some() as usize,
                kind: VTTErrorKind::Parse(e.to_string()),
            })?
        } else {
            Time::parse(start_str.trim(), TIME_FORMAT).map_err(|e| Error {
                line: 1 + identifier.is_some() as usize,
                kind: VTTErrorKind::Parse(e.to_string()),
            })?
        };
        // if the end string contains a whitespace, it probably also will contain a settings list
        // that is parsed in the if block
        if end_str.trim().contains(' ') {
            let settings_str;
            (end_str, settings_str) = end_str.trim().split_once(' ').unwrap();

            for setting in settings_str.split(' ') {
                if let Some((id, value)) = setting.split_once(':') {
                    settings.insert(id.to_string(), Some(value.to_string()));
                } else {
                    settings.insert(setting.to_string(), None);
                }
            }
        }
        // get the end time. because the parse functionality of the `time` crate isn't capable of
        // parsing optional literals or templates that only contains minutes, seconds and subseconds
        // the hour part must be prepended if not existent
        let end = if end_str.chars().filter(|c| *c == ':').count() < 2 {
            let end_str = format!("00:{}", end_str.trim());
            Time::parse(&end_str, TIME_FORMAT).map_err(|e| Error {
                line: 1 + identifier.is_some() as usize,
                kind: VTTErrorKind::Parse(e.to_string()),
            })?
        } else {
            Time::parse(end_str.trim(), TIME_FORMAT).map_err(|e| Error {
                line: 1 + identifier.is_some() as usize,
                kind: VTTErrorKind::Parse(e.to_string()),
            })?
        };

        Ok(VTTLine {
            identifier,
            start,
            end,
            settings,
            text: block_lines.collect::<Vec<&str>>().join("\n"),
        })
    }

    fn parse_percentage(s: &str) -> Option<f32> {
        if !s.ends_with('%') {
            return None;
        }
        s[..s.len() - 1].parse().ok()
    }
}

error! {
    VTTError => VTTErrorKind {
        InvalidFormat,
        Parse(String),
    }
}
