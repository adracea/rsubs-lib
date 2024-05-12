use rsubs_lib::util::Color;
use rsubs_lib::VTT;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("tests/fixtures/test.srt")?;
    let mut vtt = VTT::parse(content)?;

    for style in &mut vtt.styles {
        style.entries.insert(
            "color".to_string(),
            Color::new(255, 0, 0, 255).to_vtt_string(),
        );
    }
    for line in &mut vtt.lines {
        line.text.push_str("!!!!!!")
    }

    fs::write("result.vtt", vtt.to_string())?;
    Ok(())
}
