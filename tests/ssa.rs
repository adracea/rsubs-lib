use rsubs_lib::{SSAErrorKind, SSA};

const SIMPLE: &str = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:06.20,0:00:08.20,Default,,0000,0000,0000,,Lorem Ipsum1
Dialogue: 0,0:00:08.20,0:00:10.20,Default,,0000,0000,0000,,Lorem Ipsum2
Dialogue: 0,0:00:12.20,0:00:14.20,Default,,0000,0000,0000,,Lorem Ipsum3
Dialogue: 0,0:00:14.20,0:00:16.20,Default,,0000,0000,0000,,Lorem Ipsum4";

#[test]
fn convert_simple_to_srt() {
    let expected = r"1
00:00:06,200 --> 00:00:08,200
Lorem Ipsum1

2
00:00:08,200 --> 00:00:10,200
Lorem Ipsum2

3
00:00:12,200 --> 00:00:14,200
Lorem Ipsum3

4
00:00:14,200 --> 00:00:16,200
Lorem Ipsum4";
    let ssa = SSA::parse(SIMPLE).unwrap();
    assert_eq!(ssa.to_srt().to_string(), expected)
}

#[test]
fn convert_simple_to_vtt() {
    let expected = r"WEBVTT

1
00:00:06.200 --> 00:00:08.200
Lorem Ipsum1

2
00:00:08.200 --> 00:00:10.200
Lorem Ipsum2

3
00:00:12.200 --> 00:00:14.200
Lorem Ipsum3

4
00:00:14.200 --> 00:00:16.200
Lorem Ipsum4";
    let ssa = SSA::parse(SIMPLE).unwrap();
    assert_eq!(ssa.to_vtt().to_string(), expected)
}

const STYLING_INLINE: &str = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,{\u1}Lorem Ipsum2{\u0}
Dialogue: 0,0:00:06.20,0:00:08.20,Default,,0000,0000,0000,,{\c1}Lorem Ipsum3{\c0}
";

#[test]
fn convert_styling_inline_to_srt() {
    let expected = r"1
00:00:00,200 --> 00:00:02,200
<i>Lorem Ipsum1</i>

2
00:00:02,200 --> 00:00:04,200
<u>Lorem Ipsum2</u>

3
00:00:06,200 --> 00:00:08,200
Lorem Ipsum3";
    let ssa = SSA::parse(STYLING_INLINE).unwrap();
    assert_eq!(ssa.to_srt().to_string(), expected)
}

#[test]
fn convert_styling_inline_to_vtt() {
    let expected = r"WEBVTT

1
00:00:00.200 --> 00:00:02.200
<i>Lorem Ipsum1</i>

2
00:00:02.200 --> 00:00:04.200
<u>Lorem Ipsum2</u>

3
00:00:06.200 --> 00:00:08.200
Lorem Ipsum3";
    let ssa = SSA::parse(STYLING_INLINE).unwrap();
    assert_eq!(ssa.to_vtt().to_string(), expected)
}

const STYLING_GLOBAL: &str = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,Lorem Ipsum2
Dialogue: 0,0:00:06.20,0:00:08.20,Default,,0000,0000,0000,,Lorem Ipsum3";

#[test]
fn convert_styling_global_to_srt() {
    let expected = r"1
00:00:00,200 --> 00:00:02,200
<b><i>Lorem Ipsum1</i></b>

2
00:00:02,200 --> 00:00:04,200
<b>Lorem Ipsum2</b>

3
00:00:06,200 --> 00:00:08,200
<b>Lorem Ipsum3</b>";
    let ssa = SSA::parse(STYLING_GLOBAL).unwrap();
    assert_eq!(ssa.to_srt().to_string(), expected)
}

#[test]
fn convert_styling_global_to_vtt() {
    let expected = r"WEBVTT

1
00:00:00.200 --> 00:00:02.200
<b><i>Lorem Ipsum1</i></b>

2
00:00:02.200 --> 00:00:04.200
<b>Lorem Ipsum2</b>

3
00:00:06.200 --> 00:00:08.200
<b>Lorem Ipsum3</b>";
    let ssa = SSA::parse(STYLING_GLOBAL).unwrap();
    assert_eq!(ssa.to_vtt().to_string(), expected)
}

const MULTILINE: &str = r"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,Lorem Ipsum1\NLorem Ipsum2\NLorem Ipsum3
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,Lorem Ipsum4";

#[test]
fn convert_multiline_to_srt() {
    let expected = r"1
00:00:00,200 --> 00:00:02,200
Lorem Ipsum1
Lorem Ipsum2
Lorem Ipsum3

2
00:00:02,200 --> 00:00:04,200
Lorem Ipsum4";
    let ssa = SSA::parse(MULTILINE).unwrap();
    assert_eq!(ssa.to_srt().to_string(), expected)
}

#[test]
fn convert_multiline_to_vtt() {
    let expected = r"WEBVTT

1
00:00:00.200 --> 00:00:02.200
Lorem Ipsum1
Lorem Ipsum2
Lorem Ipsum3

2
00:00:02.200 --> 00:00:04.200
Lorem Ipsum4";
    let ssa = SSA::parse(MULTILINE).unwrap();
    assert_eq!(ssa.to_vtt().to_string(), expected)
}

#[test]
fn serde() {
    let ssa = r#"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,{\c1}Lorem Ipsum2{\c0}
"#;

    let deserialized = serde_json::to_value(SSA::parse(ssa).unwrap()).unwrap();
    let _: SSA = serde_json::from_value(deserialized).unwrap();
}

#[test]
fn invalid_start_block() {
    let ssa = r#"[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,{\c1}Lorem Ipsum2{\c0}
Dialogue: 0,0:00:06.20,0:00:08.20,Default,,0000,0000,0000,,Lorem Ipsum3
Dialogue: 0,0:00:08.20,0:00:10.20,Default,,0000,0000,0000,,Lorem Ipsum4
Dialogue: 0,0:00:12.20,0:00:14.20,Default,,0000,0000,0000,,Lorem Ipsum5
Dialogue: 0,0:00:14.20,0:00:16.20,Default,,0000,0000,0000,,Lorem Ipsum6

[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De2,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De4,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 1);
    assert!(matches!(err.kind(), SSAErrorKind::Invalid))
}

#[test]
fn empty_block() {
    let ssa = r#"[Script Info]


[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 3);
    assert!(matches!(err.kind(), SSAErrorKind::EmptyBlock))
}

#[test]
fn info_missing_delimiter() {
    let ssa = r#"[Script Info]
Collisions
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 2);
    assert!(matches!(err.kind(), SSAErrorKind::Parse(_)))
}

#[test]
fn styles_missing_header() {
    let ssa = r#"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 4);
    assert_eq!(
        err.kind(),
        &SSAErrorKind::MissingHeader("BackColour".to_string())
    )
}

#[test]
fn styles_missing_field() {
    let ssa = r#"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 5);
    assert!(matches!(err.kind(), SSAErrorKind::Parse(_)))
}

#[test]
fn styles_invalid_colors() {
    let colors = [
        "#00000000",
        "&H-1000000",
        "&H00-10000",
        "&H0000-100",
        "&H000000-1",
    ];

    for color in colors {
        let err = SSA::parse(format!(r#"[Script Info]

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,{color},&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
"#)).unwrap_err();
        assert_eq!(err.line(), 5);
        assert!(matches!(err.kind(), SSAErrorKind::Parse(_)))
    }
}

#[test]
fn events_missing_header() {
    let ssa = r#"[Script Info]

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 4);
    assert_eq!(err.kind(), &SSAErrorKind::MissingHeader("Text".to_string()))
}

#[test]
fn events_missing_field() {
    let ssa = r#"[Script Info]

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,,{\i1}Lorem Ipsum1{\i0}
"#;

    let err = SSA::parse(ssa).unwrap_err();
    assert_eq!(err.line(), 5);
    assert!(matches!(err.kind(), SSAErrorKind::Parse(_)))
}
