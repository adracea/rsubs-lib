//! Implements helpers for `.ass` and `.ssa`.
//!
//! It describes the [SSAFile], [SSAEvent] and [SSAStyle] structs and
//! provides the [parse] function.

use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

use crate::util::Alignment;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::ssa::parse::TIME_FORMAT;
use crate::util::Color;
use crate::vtt::VTT;
use time::Time;

use super::srt::{SRTLine, SRT};

/// [SSAInfo] contains headers and general information about the script.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SSAInfo {
    /// Description of the script.
    pub title: Option<String>,
    /// Original author(s) of the script.
    pub original_script: Option<String>,
    /// Original translator of the dialogue.
    pub original_translation: Option<String>,
    /// Original script editor(s).
    pub original_editing: Option<String>,
    /// Whoever timed the original script
    pub original_timing: Option<String>,
    /// Description of where in the video the script should begin playback.
    pub synch_point: Option<String>,
    /// Names of any other subtitling groups who edited the original script.
    pub script_update_by: Option<String>,
    /// The details of any updates to the original script - made by other subtitling groups
    pub update_details: Option<String>,
    /// The SSA script format version.
    pub script_type: Option<String>,
    /// Determines how subtitles are moved, when automatically preventing onscreen collisions.
    /// Allowed values:
    /// - `Normal`: SSA will attempt to position subtitles in the position specified by the
    ///   "margins". However, subtitles can be shifted vertically to prevent onscreen collisions.
    ///   With "normal" collision prevention, the subtitles will "stack up" one above the other -
    ///   but they will always be positioned as close the vertical (bottom) margin as possible -
    ///   filling in "gaps" in other subtitles if one large enough is available.
    /// - `Reverse`: Subtitles will be shifted upwards to make room for subsequent overlapping
    ///   subtitles. This means the subtitles can nearly always be read top-down - but it also means
    ///   that the first subtitle can appear halfway up the screen before the subsequent overlapping
    ///   subtitles appear. It can use a lot of screen area.
    pub collisions: Option<String>,
    /// The height of the screen used by the script's author(s) when playing the script.
    pub play_res_y: Option<u32>,
    /// The width of the screen used by the script's author(s) when playing the script.
    pub play_res_x: Option<u32>,
    /// The color depth used by the script's author(s) when playing the script.
    pub play_depth: Option<u32>,
    /// The Timer Speed for the script, as percentage. So `100` == `100%`.
    pub timer: Option<f32>,
    /// Defines the default wrapping style.
    /// Allowed values are:
    /// - `0`: smart wrapping, lines are evenly broken
    /// - `1`: end-of-line word wrapping, only \N breaks
    /// - `2`: no word wrapping, \n \N both breaks
    /// - `3`: same as 0, but lower line gets wider
    pub wrap_style: Option<u8>,

    /// Additional fields that aren't covered by the ASS spec.
    pub additional_fields: HashMap<String, String>,
}
impl Eq for SSAInfo {}

/// [SSAStyle] describes each part of the `Format: ` side of a `.ssa` or `.ass` subtitle.
///
/// Currently only supports `.ass`, more precisely `ScriptType: V4.00+` and `[V4+ Styles]`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SSAStyle {
    /// Name of the style. Case-sensitive. Cannot include commas.
    pub name: String,
    /// Fontname as used by Windows. Case-sensitive.
    pub fontname: String,
    /// Fontsize.
    pub fontsize: f32,
    /// The color that a subtitle will normally appear in.
    pub primary_color: Option<Color>,
    /// This color may be used instead of the Primary colour when a subtitle is automatically
    /// shifted to prevent an onscreen collision, to distinguish the different subtitles.
    pub secondary_color: Option<Color>,
    /// This color may be used instead of the Primary or Secondary colour when a subtitle is
    /// automatically shifted to prevent an onscreen collision, to distinguish the different
    /// subtitles.
    pub outline_color: Option<Color>,
    /// The color of the subtitle outline or shadow.
    pub back_color: Option<Color>,
    /// Defines whether text is bold or not.
    pub bold: bool,
    /// Defines whether text is italic or not.
    pub italic: bool,
    /// Defines whether text is underlined or not.
    pub underline: bool,
    /// Defines whether text is strikeout or not.
    pub strikeout: bool,
    /// Modifies the width of the font. Value is percentage.
    pub scale_x: f32,
    /// Modifies the height of the font. Value is percentage.
    pub scale_y: f32,
    /// Extra space between characters (in pixels).
    pub spacing: f32,
    /// Origin of the rotation is defined by the alignment (as degrees).
    pub angle: f32,
    /// Border style.
    /// Allowed values are:
    /// - `1`: Outline + drop shadow
    /// - `3`: Opaque box
    pub border_style: u8,
    /// If [SSAStyle::border_style] is `1`, then this specifies the width of the outline around the
    /// text (in pixels).
    /// Values may be `0`, `1`, `2`, `3` or `4`.
    pub outline: f32,
    /// If [SSAStyle::border_style] is `1`, then this specifies the depth of the drop shadow behind
    /// the text (in pixels). Values may be `0`, `1`, `2`, `3` or `4`. Drop shadow is always used in
    /// addition to an outline - SSA will force an outline of 1 pixel if no outline width is given.
    pub shadow: f32,
    /// Sets how text is "justified" within the Left/Right onscreen margins, and also the vertical
    /// placing.
    pub alignment: Alignment,
    /// Defines the Left Margin in pixels.
    pub margin_l: f32,
    /// Defines the Right Margin in pixels.
    pub margin_r: f32,
    /// Defines the Vertical Left Margin in pixels.
    pub margin_v: f32,
    /// Specifies the font character set or encoding and on multilingual Windows installations it
    /// provides access to characters used in multiple than one language. It is usually 0 (zero)
    /// for English (Western, ANSI) Windows.
    pub encoding: f32,
}
impl Eq for SSAStyle {}

impl Default for SSAStyle {
    fn default() -> Self {
        SSAStyle {
            name: "Default".to_string(),
            fontname: "Trebuchet MS".to_string(),
            fontsize: 25.5,
            primary_color: None,
            secondary_color: None,
            outline_color: None,
            back_color: None,
            bold: false,
            italic: false,
            underline: false,
            strikeout: false,
            scale_x: 120.0,
            scale_y: 120.0,
            spacing: 0.0,
            angle: 0.0,
            border_style: 1,
            outline: 1.0,
            shadow: 1.0,
            alignment: Alignment::BottomCenter,
            margin_l: 0.0,
            margin_r: 0.0,
            margin_v: 20.0,
            encoding: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SSAEventLineType {
    Dialogue,
    Comment,
    Other(String),
}

/// Describes each individual element of an `Event` line in the `.ass` format
///
/// Each element can be individually changed.
///
/// Because of its comma separated values in the event line, the timestamp looks like
/// `00:00:20.00` and it can be represented using [Time::to_ass_string]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SSAEvent {
    /// Subtitles having different layer number will be ignored during the collusion detection.
    /// Higher numbered layers will be drawn over the lower numbered.
    pub layer: u32,
    /// Start time of the line being displayed.
    pub start: Time,
    /// End time of the line being displayed
    pub end: Time,
    /// String value relating to an [SSAStyle].
    pub style: String,
    /// Generally this is used for "speaker name", in most cases it's an unused field.
    pub name: String,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub margin_l: f32,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub margin_r: f32,
    /// SSA/ASS documentation describes the l/r/v margins as being floats so...here goes
    /// In practice it gets represented as `0020` and similar `{:0>4}` patterns.
    pub margin_v: f32,
    /// SSA Documentation describes it, it's here, no idea what it does, but you can write it if you
    /// wish.
    pub effect: String,
    /// The line's text.
    pub text: String,
    pub line_type: SSAEventLineType,
}
impl Eq for SSAEvent {}

impl Default for SSAEvent {
    fn default() -> Self {
        SSAEvent {
            layer: 0,
            start: Time::from_hms(0, 0, 0).unwrap(),
            end: Time::from_hms(0, 0, 0).unwrap(),
            style: "Default".to_string(),
            name: "".to_string(),
            margin_l: 0.0,
            margin_r: 0.0,
            margin_v: 0.0,
            effect: "".to_string(),
            text: "".to_string(),
            line_type: SSAEventLineType::Dialogue,
        }
    }
}

/// Parser options for SSA/ASS.
/// - `lenient_style_bools`: if true, accept `1` as `true` in Styles
///   (Bold/Italic/Underline/StrikeOut) in addition to spec `-1/0`.
#[derive(Clone, Copy, Default)]
pub struct SSAParseOptions {
    pub lenient_style_bools: bool,
}

/// Contains the styles, events and info as well as a format mentioning whether it's `.ass` or `.ssa`
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct SSA {
    pub info: SSAInfo,
    pub styles: Vec<SSAStyle>,
    pub events: Vec<SSAEvent>,
    pub fonts: Vec<String>,
    pub graphics: Vec<String>,
}

impl SSA {

    pub fn parse<S: AsRef<str>>(content: S) -> Result<SSA, SSAError> {
        Self::parse_with_options(content, SSAParseOptions::default())
    }

    pub fn parse_lenient<S: AsRef<str>>(s: S) -> Result<SSA, SSAError> {
        Self::parse_with_options(s, SSAParseOptions { lenient_style_bools: true })
    }

    /// Parses the given [String] into [SSA].
    pub fn parse_with_options<S: AsRef<str>>(content: S, opts: SSAParseOptions) -> Result<SSA, SSAError> {
        let mut line_num = 0;

        let mut blocks = vec![vec![]];
        for line in content.as_ref().lines() {
            if line.trim().is_empty() {
                blocks.push(vec![])
            } else {
                blocks.last_mut().unwrap().push(line)
            }
        }

        let mut ssa = SSA::default();

        if blocks[0].first().is_some_and(|l| *l == "[Script Info]") {
            line_num += 1;
            let mut block = blocks.remove(0);
            let block_len = block.len();
            block.remove(0);
            ssa.info = parse::parse_script_info_block(block.into_iter())
                .map_err(|e| SSAError::new(e.kind, line_num + e.line))?;
            line_num += block_len
        } else {
            return Err(SSAError::new(SSAErrorKind::Invalid, 1));
        }

        for mut block in blocks {
            line_num += 1;

            if block.is_empty() {
                return Err(SSAError::new(SSAErrorKind::EmptyBlock, line_num));
            }

            let block_len = block.len();

            match block.remove(0) {
                "[V4+ Styles]" => {
                    ssa.styles = parse::parse_style_block(block.into_iter(), opts)
                        .map_err(|e| SSAError::new(e.kind, line_num + e.line))?
                }
                "[Events]" => {
                    ssa.events = parse::parse_events_block(block.into_iter())
                        .map_err(|e| SSAError::new(e.kind, line_num + e.line))?
                }
                "[Fonts]" => {
                    ssa.fonts = parse::parse_fonts_block(block.into_iter())
                        .map_err(|e| SSAError::new(e.kind, line_num + e.line))?
                }
                "[Graphics]" => {
                    ssa.graphics = parse::parse_graphics_block(block.into_iter())
                        .map_err(|e| SSAError::new(e.kind, line_num + e.line))?
                }
                _ => continue,
            }

            line_num += block_len
        }

        Ok(ssa)
    }

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
    pub fn to_srt(&self) -> SRT {
        let style_remove_regex = Regex::new(r"(?m)\{\\.+?}").unwrap();

        let mut lines = vec![];

        for (i, event) in self.events.iter().enumerate() {
            let mut text = event
                .text
                .replace("{\\b1}", "<b>")
                .replace("{\\b0}", "</b>")
                .replace("{\\i1}", "<i>")
                .replace("{\\i0}", "</i>")
                .replace("{\\u1}", "<u>")
                .replace("{\\u0}", "</u>")
                .replace("\\N", "\r\n");

            if !event.style.is_empty() {
                if let Some(style) = self.styles.iter().find(|s| s.name == event.style) {
                    if style.bold {
                        text = format!("<b>{text}</b>")
                    }
                    if style.italic {
                        text = format!("<i>{text}</i>")
                    }
                    if style.underline {
                        text = format!("<u>{text}</u>")
                    }
                }
            }

            lines.push(SRTLine {
                sequence_number: i as u32 + 1,
                start: event.start,
                end: event.end,
                text: style_remove_regex.replace_all(&text, "").to_string(),
            })
        }

        SRT { lines }
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
    pub fn to_vtt(self) -> VTT {
        self.to_srt().to_vtt()
    }
}

impl Display for SSA {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];

        lines.push("[Script Info]".to_string());
        lines.extend(self.info.title.as_ref().map(|l| format!("Title: {l}")));
        lines.extend(self.info.original_script.as_ref().map(|l| format!("Original Script: {l}")));
        lines.extend(self.info.original_translation.as_ref().map(|l| format!("Original Translation: {l}")));
        lines.extend(self.info.original_editing.as_ref().map(|l| format!("Original Editing: {l}")));
        lines.extend(self.info.original_timing.as_ref().map(|l| format!("Original Timing: {l}")));
        lines.extend(self.info.synch_point.as_ref().map(|l| format!("Synch Point: {l}")));
        lines.extend(self.info.script_update_by.as_ref().map(|l| format!("Script Updated By: {l}")));
        lines.extend(self.info.update_details.as_ref().map(|l| format!("Update Details: {l}")));
        lines.extend(self.info.script_type.as_ref().map(|l| format!("Script Type: {l}")));
        lines.extend(self.info.collisions.as_ref().map(|l| format!("Collisions: {l}")));
        lines.extend(self.info.play_res_y.map(|l| format!("PlayResY: {l}")));
        lines.extend(self.info.play_res_x.map(|l| format!("PlayResX: {l}")));
        lines.extend(self.info.play_depth.map(|l| format!("PlayDepth: {l}")));
        lines.extend(self.info.timer.map(|l| format!("Timer: {l}")));
        lines.extend(self.info.wrap_style.map(|l| format!("WrapStyle: {l}")));
        for (k, v) in &self.info.additional_fields {
            lines.push(format!("{k}: {v}"))
        }

        lines.push("".to_string());
        lines.push("[V4+ Styles]".to_string());
        lines.push("Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,StrikeOut,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding".to_string());
        for style in &self.styles {
            let line = [
                style.name.to_string(),
                style.fontname.to_string(),
                style.fontsize.to_string(),
                style.primary_color.map(|c| c.to_ssa_string()).unwrap_or_default(),
                style.secondary_color.map(|c| c.to_ssa_string()).unwrap_or_default(),
                style.outline_color.map(|c| c.to_ssa_string()).unwrap_or_default(),
                style.back_color.map(|c| c.to_ssa_string()).unwrap_or_default(),
                if style.bold { "-1" } else { "0" }.to_string(),
                if style.italic { "-1" } else { "0" }.to_string(),
                if style.underline { "-1" } else { "0" }.to_string(),
                if style.strikeout { "-1" } else { "0" }.to_string(),
                style.scale_x.to_string(),
                style.scale_y.to_string(),
                style.spacing.to_string(),
                style.angle.to_string(),
                style.border_style.to_string(),
                style.outline.to_string(),
                style.shadow.to_string(),
                (style.alignment as u8).to_string(),
                style.margin_l.to_string(),
                style.margin_r.to_string(),
                style.margin_v.to_string(),
                style.encoding.to_string(),
            ];
            lines.push(format!("Style: {}", line.join(",")))
        }

        lines.push("".to_string());
        lines.push("[Events]".to_string());
        lines.push("Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text".to_string());
        for event in &self.events {
            let line = [
                event.layer.to_string(),
                event.start.format(TIME_FORMAT).unwrap(),
                event.end.format(TIME_FORMAT).unwrap(),
                event.style.to_string(),
                event.name.to_string(),
                event.margin_l.to_string(),
                event.margin_r.to_string(),
                event.margin_v.to_string(),
                event.effect.to_string(),
                event.text.to_string()
            ];
            lines.push(format!("Dialogue: {}", line.join(",")))
        }

        write!(f, "{}", lines.join("\n"))
    }
}

error! {
    SSAError => SSAErrorKind {
        Invalid,
        EmptyBlock,
        Parse(String),
        MissingHeader(String),
    }
}

mod parse {
    use super::*;
    use std::num::{ParseFloatError, ParseIntError};
    use time::format_description::BorrowedFormatItem;
    use time::macros::format_description;

    pub(super) struct Error {
        pub(super) line: usize,
        pub(super) kind: SSAErrorKind,
    }

    pub(super) const TIME_FORMAT: &[BorrowedFormatItem] =
        format_description!("[hour padding:none]:[minute]:[second].[subsecond digits:2]");

    type Result<T> = std::result::Result<T, Error>;

    pub(super) fn parse_script_info_block<'a, I: Iterator<Item = &'a str>>(
        block_lines: I,
    ) -> Result<SSAInfo> {
        let mut info = SSAInfo::default();

        for (i, line) in block_lines.enumerate() {
            if line.starts_with(';') {
                continue;
            }

            let Some((name, mut value)) = line.split_once(':') else {
                return Err(Error {
                    line: 1 + i,
                    kind: SSAErrorKind::Parse("delimiter ':' missing".to_string()),
                });
            };
            value = value.trim();

            if value.is_empty() {
                continue;
            }

            match name {
                "Title" => info.title = Some(value.to_string()),
                "Original Script" => info.original_script = Some(value.to_string()),
                "Original Translation" => info.original_translation = Some(value.to_string()),
                "Original Editing" => info.original_editing = Some(value.to_string()),
                "Original Timing" => info.original_timing = Some(value.to_string()),
                "Synch Point" => info.synch_point = Some(value.to_string()),
                "Script Updated By" => info.script_update_by = Some(value.to_string()),
                "Update Details" => info.update_details = Some(value.to_string()),
                "ScriptType" => info.script_type = Some(value.to_string()),
                "Collisions" => info.collisions = Some(value.to_string()),
                "PlayResY" => {
                    info.play_res_y = value.parse::<u32>().map(Some).map_err(|e| Error {
                        line: 1 + i,
                        kind: SSAErrorKind::Parse(e.to_string()),
                    })?
                }
                "PlayResX" => {
                    info.play_res_x = value.parse::<u32>().map(Some).map_err(|e| Error {
                        line: 1 + i,
                        kind: SSAErrorKind::Parse(e.to_string()),
                    })?
                }
                "PlayDepth" => {
                    info.play_depth = value.parse::<u32>().map(Some).map_err(|e| Error {
                        line: 1 + i,
                        kind: SSAErrorKind::Parse(e.to_string()),
                    })?
                }
                "Timer" => {
                    info.timer = value.parse::<f32>().map(Some).map_err(|e| Error {
                        line: 1 + i,
                        kind: SSAErrorKind::Parse(e.to_string()),
                    })?
                }
                "WrapStyle" => {
                    info.wrap_style = value.parse::<u8>().map(Some).map_err(|e| Error {
                        line: 1 + i,
                        kind: SSAErrorKind::Parse(e.to_string()),
                    })?
                }
                _ => {
                    info.additional_fields
                        .insert(name.to_string(), value.to_string());
                }
            }
        }

        Ok(info)
    }

    pub(super) fn parse_style_block<'a, I: Iterator<Item = &'a str>>(
        mut block_lines: I,
        opts: SSAParseOptions
    ) -> Result<Vec<SSAStyle>> {
        let mut header_line = 1;
        let header = loop {
            let Some(line) = block_lines.next() else {
                return Err(Error {
                    line: 1,
                    kind: SSAErrorKind::EmptyBlock,
                });
            };
            if !line.starts_with(';') {
                break line.to_string();
            }
            header_line += 1;
        };
        let Some(header) = header.strip_prefix("Format:") else {
            return Err(Error {
                line: header_line,
                kind: SSAErrorKind::Parse("styles header must start with 'Format:'".to_string()),
            });
        };
        let headers = header.trim().split(',').collect();

        let mut styles = vec![];

        for (i, line) in block_lines.enumerate() {
            if line.starts_with(';') {
                continue;
            }

            let Some(line) = line.strip_prefix("Style:") else {
                return Err(Error {
                    line: header_line + 1 + i,
                    kind: SSAErrorKind::Parse("styles line must start with 'Style:'".to_string()),
                });
            };
            let line_list: Vec<&str> = line.trim().split(',').collect();

            styles.push(SSAStyle {
                name: get_line_value(
                    &headers,
                    "Name",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                fontname: get_line_value(
                    &headers,
                    "Fontname",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                fontsize: get_line_value(
                    &headers,
                    "Fontsize",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                primary_color: Color::from_ssa(get_line_value(
                    &headers,
                    "PrimaryColour",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?)
                .map_err(|e| Error {
                    line: 2 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                secondary_color: Color::from_ssa(get_line_value(
                    &headers,
                    "SecondaryColour",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?)
                .map_err(|e| Error {
                    line: 2 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                outline_color: Color::from_ssa(get_line_value(
                    &headers,
                    "OutlineColour",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?)
                .map_err(|e| Error {
                    line: 2 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                back_color: Color::from_ssa(get_line_value(
                    &headers,
                    "BackColour",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?)
                .map_err(|e| Error {
                    line: header_line + 1 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                bold: parse_str_to_bool(
                    get_line_value(
                        &headers,
                        "Bold",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    header_line + 1 + i,
                    opts,
                )?,
                italic: parse_str_to_bool(
                    get_line_value(
                        &headers,
                        "Italic",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    header_line + 1 + i,
                    opts,
                )?,
                underline: parse_str_to_bool(
                    get_line_value(
                        &headers,
                        "Underline",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    header_line + 1 + i,
                    opts,
                )?,
                strikeout: parse_str_to_bool(
                    get_line_value(
                        &headers,
                        "StrikeOut",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    header_line + 1 + i,
                    opts,
                )?,
                scale_x: get_line_value(
                    &headers,
                    "ScaleX",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                scale_y: get_line_value(
                    &headers,
                    "ScaleY",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                spacing: get_line_value(
                    &headers,
                    "Spacing",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                angle: get_line_value(
                    &headers,
                    "Angle",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                border_style: get_line_value(
                    &headers,
                    "BorderStyle",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_int_err(e, header_line + 1 + i))?,
                outline: get_line_value(
                    &headers,
                    "Outline",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                shadow: get_line_value(
                    &headers,
                    "Shadow",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                alignment: Alignment::infer_from_str(get_line_value(
                    &headers,
                    "Alignment",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?)
                .unwrap(),
                margin_l: get_line_value(
                    &headers,
                    "MarginL",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                margin_r: get_line_value(
                    &headers,
                    "MarginR",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                margin_v: get_line_value(
                    &headers,
                    "MarginV",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                encoding: get_line_value(
                    &headers,
                    "Encoding",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map(|op: f32| f32::from(op))
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
            })
        }

        Ok(styles)
    }

    pub(super) fn parse_events_block<'a, I: Iterator<Item = &'a str>>(
        mut block_lines: I,
    ) -> Result<Vec<SSAEvent>> {
        let mut header_line = 1;
        let header = loop {
            let Some(line) = block_lines.next() else {
                return Err(Error {
                    line: 1,
                    kind: SSAErrorKind::EmptyBlock,
                });
            };
            if !line.starts_with(';') {
                break line.to_string();
            }
            header_line += 1;
        };
        let Some(header) = header.strip_prefix("Format:") else {
            return Err(Error {
                line: header_line,
                kind: SSAErrorKind::Parse("events header must start with 'Format:'".to_string()),
            });
        };
        let headers = header.trim().split(',').collect();

        let mut events = vec![];

        for (i, line) in block_lines.enumerate() {
            if line.starts_with(';') {
                continue;
            }

            let Some((line_type, line)) = line.split_once(':') else {
                return Err(Error {
                    line: 2 + i,
                    kind: SSAErrorKind::Parse("delimiter ':' missing".to_string()),
                });
            };
            let line_list: Vec<&str> = line.trim().splitn(10, ',').collect();

            events.push(SSAEvent {
                layer: get_line_value(
                    &headers,
                    "Layer",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_int_err(e, header_line + 1 + i))?,
                start: Time::parse(
                    get_line_value(
                        &headers,
                        "Start",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    TIME_FORMAT,
                )
                .map_err(|e| Error {
                    line: header_line + 1 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                end: Time::parse(
                    get_line_value(
                        &headers,
                        "End",
                        &line_list,
                        header_line,
                        header_line + 1 + i,
                    )?,
                    TIME_FORMAT,
                )
                .map_err(|e| Error {
                    line: header_line + 1 + i,
                    kind: SSAErrorKind::Parse(e.to_string()),
                })?,
                style: get_line_value(
                    &headers,
                    "Style",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                name: get_line_value(
                    &headers,
                    "Name",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                margin_l: get_line_value(
                    &headers,
                    "MarginL",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                margin_r: get_line_value(
                    &headers,
                    "MarginR",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                margin_v: get_line_value(
                    &headers,
                    "MarginV",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .parse()
                .map_err(|e| map_parse_float_err(e, header_line + 1 + i))?,
                effect: get_line_value(
                    &headers,
                    "Effect",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                text: get_line_value(
                    &headers,
                    "Text",
                    &line_list,
                    header_line,
                    header_line + 1 + i,
                )?
                .to_string(),
                line_type: match line_type {
                    "Dialogue" => SSAEventLineType::Dialogue,
                    "Comment" => SSAEventLineType::Comment,
                    _ => SSAEventLineType::Other(line_type.to_string()),
                },
            })
        }

        Ok(events)
    }

    pub(super) fn parse_fonts_block<'a, I: Iterator<Item = &'a str>>(
        block_lines: I,
    ) -> Result<Vec<String>> {
        let mut fonts = vec![];

        for (i, line) in block_lines.enumerate() {
            let Some(line) = line.strip_prefix("fontname:") else {
                return Err(Error {
                    line: 1 + i,
                    kind: SSAErrorKind::Parse("fonts line must start with 'fontname:'".to_string()),
                });
            };
            fonts.push(line.trim().to_string())
        }

        Ok(fonts)
    }

    pub(super) fn parse_graphics_block<'a, I: Iterator<Item = &'a str>>(
        block_lines: I,
    ) -> Result<Vec<String>> {
        let mut graphics = vec![];

        for (i, line) in block_lines.enumerate() {
            let Some(line) = line.strip_prefix("filename:") else {
                return Err(Error {
                    line: 1 + i,
                    kind: SSAErrorKind::Parse(
                        "graphics line must start with 'filename:'".to_string(),
                    ),
                });
            };
            graphics.push(line.trim().to_string())
        }

        Ok(graphics)
    }

    #[allow(clippy::ptr_arg)]
    fn get_line_value<'a>(
        headers: &Vec<&str>,
        name: &str,
        list: &'a Vec<&str>,
        header_line: usize,
        current_line: usize,
    ) -> Result<&'a &'a str> {
        let pos = headers
            .iter()
            .position(|h| {
                let value: &str = h.trim();

                value.to_lowercase() == name.to_lowercase()
            })
            .ok_or(Error {
                line: header_line,
                kind: SSAErrorKind::MissingHeader(name.to_string()),
            })?;

        list.get(pos).ok_or(Error {
            line: current_line,
            kind: SSAErrorKind::Parse(format!("no value for header '{}'", name)),
        })
    }
    fn parse_str_to_bool(s: &str, line: usize, opts: SSAParseOptions) -> Result<bool> {
        match s {
            
            "0" => Ok(false),
            "-1" => Ok(true),
            "1"  if opts.lenient_style_bools => Ok(true),
            _ => Err(Error {
                line,
                kind: SSAErrorKind::Parse(
                    "boolean value must be '-1 (true) or '0' (false)".to_string(),
                ),
            }),
        }
    }
    fn map_parse_int_err(e: ParseIntError, line: usize) -> Error {
        Error {
            line,
            kind: SSAErrorKind::Parse(e.to_string()),
        }
    }
    fn map_parse_float_err(e: ParseFloatError, line: usize) -> Error {
        Error {
            line,
            kind: SSAErrorKind::Parse(e.to_string()),
        }
    }
}
