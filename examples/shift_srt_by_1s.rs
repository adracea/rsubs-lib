use std::str::FromStr;

use rsubs_lib::{ssa::SSAFile, Subtitle};

/// In this example we read a .srt file and add 1s(1000ms) to each line
/// Afterwards we print the result to stdout.

fn main() {
    let mut srt =
        rsubs_lib::srt::parse("tests/fixtures/test.srt".to_string()).expect("failed parsing");
    for line in srt.lines.iter_mut() {
        line.line_end += 20;
        line.line_start += 50;
        line.line_text.push_str(" Ipsum");
    }
    println!("{srt}");
    let s = Subtitle::from_str("tests/fixtures/test.srt").unwrap();

    let a: SSAFile = s.clone().into(); //convert to `.ass` and then back to `.srt`
    assert_eq!(format!("{}", a.to_srt()), format!("{s}"));
}
