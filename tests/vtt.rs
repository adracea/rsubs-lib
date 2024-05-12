use rsubs_lib::{VTTErrorKind, VTT};
use std::fs;

#[test]
fn from_file_to_srt() {
    let content = fs::read_to_string("./tests/fixtures/test.vtt").unwrap();
    let vtt = VTT::parse(content).unwrap();
    fs::write("./tests/fixtures/from_vtt.srt", vtt.to_srt().to_string()).unwrap()
}

#[test]
fn from_file_to_ssa() {
    let content = fs::read_to_string("./tests/fixtures/test.vtt").unwrap();
    let vtt = VTT::parse(content).unwrap();
    fs::write("./tests/fixtures/from_vtt.ass", vtt.to_ssa().to_string()).unwrap()
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
