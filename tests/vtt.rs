use rsubs_lib::{VTTErrorKind, VTT};

const SIMPLE: &str = r"WEBVTT

00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City

00:13.000 --> 00:16.000
<v Roger Bingham>We’re actually at the Lucern Hotel, just down the street

00:16.000 --> 00:18.000
<v Roger Bingham>from the American Museum of Natural History

00:18.000 --> 00:20.000
<v Roger Bingham>And with me is Neil deGrasse Tyson";

#[test]
fn convert_simple_to_srt() {
    let expected = r"1
00:00:11,000 --> 00:00:13,000
We are in New York City

2
00:00:13,000 --> 00:00:16,000
We’re actually at the Lucern Hotel, just down the street

3
00:00:16,000 --> 00:00:18,000
from the American Museum of Natural History

4
00:00:18,000 --> 00:00:20,000
And with me is Neil deGrasse Tyson"
        .replace('\n', "\r\n");
    let vtt = VTT::parse(SIMPLE).unwrap();
    assert_eq!(vtt.to_srt().to_string(), expected)
}

#[test]
fn convert_simple_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,,,,,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:11.00,0:00:13.00,Default,Roger Bingham,0,0,0,,We are in New York City
Dialogue: 0,0:00:13.00,0:00:16.00,Default,Roger Bingham,0,0,0,,We’re actually at the Lucern Hotel, just down the street
Dialogue: 0,0:00:16.00,0:00:18.00,Default,Roger Bingham,0,0,0,,from the American Museum of Natural History
Dialogue: 0,0:00:18.00,0:00:20.00,Default,Roger Bingham,0,0,0,,And with me is Neil deGrasse Tyson";
    let vtt = VTT::parse(SIMPLE).unwrap();
    assert_eq!(vtt.to_ssa().to_string(), expected)
}

const STYLING_INLINE: &str = r"WEBVTT

00:32.500 --> 00:33.500 align:left size:50%
<v Neil deGrasse Tyson><i>Laughs</i>

00:35.500 --> 00:38.000
<v Roger Bingham>You know I’m so excited my glasses are falling off here.";

#[test]
fn convert_styling_inline_to_srt() {
    let expected = r"1
00:00:32,500 --> 00:00:33,500
<i>Laughs</i>

2
00:00:35,500 --> 00:00:38,000
You know I’m so excited my glasses are falling off here."
        .replace('\n', "\r\n");
    let vtt = VTT::parse(STYLING_INLINE).unwrap();
    assert_eq!(vtt.to_srt().to_string(), expected)
}

#[test]
fn convert_styling_inline_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,,,,,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:32.50,0:00:33.50,Default,Neil deGrasse Tyson,0,0,0,,{\i1}Laughs{\i0}
Dialogue: 0,0:00:35.50,0:00:38.00,Default,Roger Bingham,0,0,0,,You know I’m so excited my glasses are falling off here.";
    let vtt = VTT::parse(STYLING_INLINE).unwrap();
    assert_eq!(vtt.to_ssa().to_string(), expected)
}

const STYLING_GLOBAL: &str = r#"WEBVTT

STYLE
::cue {
    color: blue;
    font-family: "Arial", sans-serif;
    font-size: 12px;
}

00:22.000 --> 00:24.000
<v Roger Bingham>at the AMNH.

00:24.000 --> 00:26.000
<v Roger Bingham>Thank you for walking down here.

00:27.000 --> 00:30.000
<v Roger Bingham>And I want to do a follow-up on the last conversation we did.

00:30.000 --> 00:31.500 position:30% left align:right size:50% line:12
<v Roger Bingham>When we e-mailed—"#;

#[test]
fn convert_styling_global_to_srt() {
    let expected = r"1
00:00:22,000 --> 00:00:24,000
at the AMNH.

2
00:00:24,000 --> 00:00:26,000
Thank you for walking down here.

3
00:00:27,000 --> 00:00:30,000
And I want to do a follow-up on the last conversation we did.

4
00:00:30,000 --> 00:00:31,500
When we e-mailed—"
        .replace('\n', "\r\n");
    let vtt = VTT::parse(STYLING_GLOBAL).unwrap();
    assert_eq!(vtt.to_srt().to_string(), expected)
}

#[test]
fn convert_styling_global_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,12,&HFF0000,,,,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:22.00,0:00:24.00,Default,Roger Bingham,0,0,0,,at the AMNH.
Dialogue: 0,0:00:24.00,0:00:26.00,Default,Roger Bingham,0,0,0,,Thank you for walking down here.
Dialogue: 0,0:00:27.00,0:00:30.00,Default,Roger Bingham,0,0,0,,And I want to do a follow-up on the last conversation we did.
Dialogue: 0,0:00:30.00,0:00:31.50,Default,Roger Bingham,0,0,0,,When we e-mailed—";
    let vtt = VTT::parse(STYLING_GLOBAL).unwrap();
    assert_eq!(vtt.to_ssa().to_string(), expected)
}

const MULTILINE: &str = r"WEBVTT

00:00.000 --> 00:02.000 align:left size:50% position:030%
Lorem Ipsum 1
Lorem Ipsum 2
Lorem Ipsum 3";

#[test]
fn convert_multiline_to_srt() {
    let expected = r"1
00:00:00,000 --> 00:00:02,000
Lorem Ipsum 1
Lorem Ipsum 2
Lorem Ipsum 3"
        .replace('\n', "\r\n");
    let vtt = VTT::parse(MULTILINE).unwrap();
    assert_eq!(vtt.to_srt().to_string(), expected)
}

#[test]
fn convert_multiline_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,,,,,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.00,0:00:02.00,Default,,0,0,0,,Lorem Ipsum 1\NLorem Ipsum 2\NLorem Ipsum 3";
    let vtt = VTT::parse(MULTILINE).unwrap();
    assert_eq!(vtt.to_ssa().to_string(), expected)
}

#[test]
fn serde() {
    let vtt = r#"WEBVTT
 
NOTE This is a note

STYLE ::cue {}

REGION
id:test
width:100%
lines:1
regionanchor:10%,10%
viewportanchor:10%,10%
scroll:up
test:test

00:11.000 --> 00:13.000
test
"#;

    let deserialized = serde_json::to_value(VTT::parse(vtt).unwrap()).unwrap();
    let _: VTT = serde_json::from_value(deserialized).unwrap();
}

#[test]
fn invalid_start() {
    let vtt = r#"00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City
    "#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 1);
    assert_eq!(err.kind(), &VTTErrorKind::InvalidFormat)
}

#[test]
fn region_missing_delimiter() {
    let vtt = r#"WEBVTT
    
REGION
id
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 4);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn region_invalid_values() {
    let pairs = [
        ("width", "test"),
        ("lines", "test"),
        ("regionanchor", "test"),
        ("regionanchor", "10%,test"),
        ("regionanchor", "test,10%"),
        ("viewportanchor", "test"),
        ("viewportanchor", "10%,test"),
        ("viewportanchor", "test,10%"),
        ("scroll", "test"),
    ];
    for (k, v) in pairs {
        let vtt = format!("WEBVTT\n\nREGION\n{k}:{v}");
        let err = VTT::parse(vtt).unwrap_err();
        assert_eq!(err.line(), 4);
        assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
    }
}

#[test]
fn style_missing_cue_prefix() {
    let vtt = r#"WEBVTT

STYLE {}
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn style_missing_selector_closing_bracket() {
    let vtt = r#"WEBVTT

STYLE ::cue(selector {}
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn style_missing_block_opening_bracket() {
    let vtt = r#"WEBVTT

STYLE ::cue }
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn style_missing_block_closing_bracket() {
    let vtt = r#"WEBVTT

STYLE ::cue {
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn style_missing_value() {
    let vtt = r#"WEBVTT

STYLE
::cue {
  color
}
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 5);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_missing_timing() {
    let vtt = r#"WEBVTT

<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 4);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_missing_timing_with_id() {
    let vtt = r#"WEBVTT

1
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 4);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_missing_start_time() {
    let vtt = r#"WEBVTT

--> 00:13.000
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_missing_end_time() {
    let vtt = r#"WEBVTT

00:11.000 -->
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_timings_minute() {
    let vtt = r#"WEBVTT

00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City
"#;

    VTT::parse(vtt).unwrap();
}

#[test]
fn cue_timings_hour() {
    let vtt = r#"WEBVTT

00:00:11.000 --> 00:00:13.000
<v Roger Bingham>We are in New York City
"#;

    VTT::parse(vtt).unwrap();
}

#[test]
fn cue_invalid_start_time_minute() {
    let vtt = r#"WEBVTT

00:11 --> 00:13.000
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_invalid_end_time_minute() {
    let vtt = r#"WEBVTT

00:11.000 --> 00:.000
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_invalid_start_time_hour() {
    let vtt = r#"WEBVTT

00:00:11 --> 00:00:13.000
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}

#[test]
fn cue_invalid_end_time_hour() {
    let vtt = r#"WEBVTT

00:00:11.000 --> 00:00:.000
<v Roger Bingham>We are in New York City
"#;

    let err = VTT::parse(vtt).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), &VTTErrorKind::Parse(_)))
}
