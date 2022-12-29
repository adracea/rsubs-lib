use rsubs_lib::srt;
use rsubs_lib::ssa;
use rsubs_lib::vtt;

fn main() {
    srt::parse("./tests/fixtures/test.srt".to_string())
        .unwrap() // Can read either a file or a string
        .to_vtt() // converts file to WEBVTT
        .to_file("./tests/fixtures/ex_test_1.vtt".to_string()) // Writes the converted subtitle to a file
        .unwrap();
    vtt::parse("./tests/fixtures/test.vtt".to_string())
        .unwrap()
        .to_ass() // converts file to SSA/ASS
        .to_file("./tests/fixtures/ex_test_1.ass".to_string())
        .unwrap();
    ssa::parse("./tests/fixtures/test.ass".to_string())
        .unwrap()
        .to_srt() // converts file to SRT
        .to_file("./tests/fixtures/ex_test_1.srt".to_string())
        .unwrap();
}
