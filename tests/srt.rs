use rsubs_lib::{SRTErrorKind, SRT};
use std::fs;

#[test]
fn from_file_to_ssa() {
    let content = fs::read_to_string("./tests/fixtures/test.srt").unwrap();
    let srt = SRT::parse(content).unwrap();
    fs::write("./tests/fixtures/from_srt.ass", srt.to_ssa().to_string()).unwrap()
}

#[test]
fn from_file_to_vtt() {
    let content = fs::read_to_string("./tests/fixtures/test.srt").unwrap();
    let srt = SRT::parse(content).unwrap();
    fs::write("./tests/fixtures/from_srt.vtt", srt.to_vtt().to_string()).unwrap()
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
