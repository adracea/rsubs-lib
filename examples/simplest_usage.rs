use std::str::FromStr;

use rsubs_lib::{vtt::VTTFile, Subtitle};

fn main() {
    let mut sub: VTTFile = Subtitle::from_str("tests/fixtures/test.srt")
        .unwrap()
        .into();
    for style in sub.styles.iter_mut() {
        style.color = rsubs_lib::util::color::ColorType::VTTColor0A(rsubs_lib::util::color::RED);
    }
    for line in sub.lines.iter_mut() {
        line.line_text = line.line_text.clone() + "!!!!!!";
    }
    sub.to_file("result.vtt").unwrap();
}
