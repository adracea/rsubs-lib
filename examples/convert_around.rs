use std::str::FromStr;

use rsubs_lib::srt;
use rsubs_lib::ssa;
use rsubs_lib::vtt;

fn main() {
    vtt::VTTFile::from(srt::SRTFile::from_str("./tests/fixtures/test.srt").unwrap()) // Can read either a file or a string
        // converts file to WEBVTT
        .to_file("./tests/fixtures/ex_test_1.vtt") // Writes the converted subtitle to a file
        .unwrap();
    ssa::SSAFile::from(vtt::parse_from_file("./tests/fixtures/test.vtt".to_string()).unwrap()) // converts file to SSA/ASS
        .to_file("./tests/fixtures/ex_test_1.ass")
        .unwrap();
    srt::SRTFile::from(ssa::parse_from_file("./tests/fixtures/test.ass".to_string()).unwrap())
        // converts file to SRT
        .to_file("./tests/fixtures/ex_test_1.srt")
        .unwrap();
}
