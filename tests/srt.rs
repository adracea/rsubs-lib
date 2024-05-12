use rsubs_lib::{SRTErrorKind, SRT};

const SIMPLE: &str = r#"1
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
And with me is Neil deGrasse Tyson

5
00:00:20,000 --> 00:00:22,000
Astrophysicist, Director of the Hayden Planetarium
"#;

#[test]
fn convert_simple_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColor,BackColour,Bold,Italic,Underline,StrikeOut,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,&HFFFFFF,&H000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:11.00,0:00:13.00,Default,,0,0,0,,We are in New York City
Dialogue: 0,0:00:13.00,0:00:16.00,Default,,0,0,0,,We’re actually at the Lucern Hotel, just down the street
Dialogue: 0,0:00:16.00,0:00:18.00,Default,,0,0,0,,from the American Museum of Natural History
Dialogue: 0,0:00:18.00,0:00:20.00,Default,,0,0,0,,And with me is Neil deGrasse Tyson
Dialogue: 0,0:00:20.00,0:00:22.00,Default,,0,0,0,,Astrophysicist, Director of the Hayden Planetarium";
    let srt = SRT::parse(SIMPLE).unwrap();
    assert_eq!(srt.to_ssa().to_string(), expected)
}

#[test]
fn convert_simple_to_vtt() {
    let expected = r"WEBVTT

1
00:00:11.000 --> 00:00:13.000
We are in New York City

2
00:00:13.000 --> 00:00:16.000
We’re actually at the Lucern Hotel, just down the street

3
00:00:16.000 --> 00:00:18.000
from the American Museum of Natural History

4
00:00:18.000 --> 00:00:20.000
And with me is Neil deGrasse Tyson

5
00:00:20.000 --> 00:00:22.000
Astrophysicist, Director of the Hayden Planetarium";
    let srt = SRT::parse(SIMPLE).unwrap();
    assert_eq!(srt.to_vtt().to_string(), expected)
}

const STYLING: &str = "1
00:00:32,000 --> 00:00:35,500
No! No no no no; 'cos 'cos obviously 'cos

2
00:00:32,500 --> 00:00:33,500
<i>Laughs</i>

3
00:00:35,500 --> 00:00:38,000
You know I’m so excited my glasses are falling off here.";

#[test]
fn convert_styling_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColor,BackColour,Bold,Italic,Underline,StrikeOut,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,&HFFFFFF,&H000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:32.00,0:00:35.50,Default,,0,0,0,,No! No no no no; 'cos 'cos obviously 'cos
Dialogue: 0,0:00:32.50,0:00:33.50,Default,,0,0,0,,{\i1}Laughs{\i0}
Dialogue: 0,0:00:35.50,0:00:38.00,Default,,0,0,0,,You know I’m so excited my glasses are falling off here.";
    let srt = SRT::parse(STYLING).unwrap();
    assert_eq!(srt.to_ssa().to_string(), expected)
}

#[test]
fn convert_styling_to_vtt() {
    let expected = r"WEBVTT

1
00:00:32.000 --> 00:00:35.500
No! No no no no; 'cos 'cos obviously 'cos

2
00:00:32.500 --> 00:00:33.500
<i>Laughs</i>

3
00:00:35.500 --> 00:00:38.000
You know I’m so excited my glasses are falling off here.";
    let srt = SRT::parse(STYLING).unwrap();
    assert_eq!(srt.to_vtt().to_string(), expected)
}

const MULTILINE: &str = "1
00:00:30,500 --> 00:00:32,500
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?";

#[test]
fn convert_multiline_to_ssa() {
    let expected = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColor,BackColour,Bold,Italic,Underline,StrikeOut,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,20,&HFFFFFF,&H000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0,0,20,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:30.50,0:00:32.50,Default,,0,0,0,,Didn’t we talk about enough in that conversation?\NDidn’t we talk about enough in that conversation?\NDidn’t we talk about enough in that conversation?";
    let srt = SRT::parse(MULTILINE).unwrap();
    assert_eq!(srt.to_ssa().to_string(), expected)
}

#[test]
fn convert_multiline_to_vtt() {
    let expected = r"WEBVTT

1
00:00:30.500 --> 00:00:32.500
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?";
    let srt = SRT::parse(MULTILINE).unwrap();
    assert_eq!(srt.to_vtt().to_string(), expected)
}

#[test]
fn serde() {
    let srt = r#"1
00:00:11,000 --> 00:00:13,000
We are in New York City
"#;

    let deserialized = serde_json::to_value(SRT::parse(srt).unwrap()).unwrap();
    let _: SRT = serde_json::from_value(deserialized).unwrap();
}

#[test]
fn missing_sequence_number() {
    let srt = r#"1
00:00:11,000 --> 00:00:13,000
We are in New York City

2
00:00:13,000 --> 00:00:16,000
We’re actually at the Lucern Hotel, just down the street

00:00:16,000 --> 00:00:18,000
from the American Museum of Natural History

4
00:00:18,000 --> 00:00:20,000
And with me is Neil deGrasse Tyson
"#;

    let err = SRT::parse(srt).unwrap_err();
    assert_eq!(err.line(), 9);
    assert!(matches!(err.kind(), SRTErrorKind::Parse(_)))
}

#[test]
fn missing_time_range() {
    let srt = r#"1
We are in New York City
"#;

    let err = SRT::parse(srt).unwrap_err();
    assert_eq!(err.line(), 2);
    assert!(matches!(err.kind(), SRTErrorKind::Parse(_)))
}

#[test]
fn invalid_from_time() {
    let srt = r#"1
00:00:11,000 -->
We are in New York City
"#;

    let err = SRT::parse(srt).unwrap_err();
    assert_eq!(err.line(), 2);
    assert!(matches!(err.kind(), SRTErrorKind::Parse(_)))
}

#[test]
fn invalid_to_time() {
    let srt = r#"1
--> 00:00:13,000
We are in New York City
"#;

    let err = SRT::parse(srt).unwrap_err();
    assert_eq!(err.line(), 2);
    assert!(matches!(err.kind(), SRTErrorKind::Parse(_)))
}
