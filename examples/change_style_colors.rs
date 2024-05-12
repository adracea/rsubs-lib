use rsubs_lib::util::Color;
/// In this example we read a .ass file and add change the color of the `Default` style to red
/// Afterwards we write the file to a new file called `result.ass` .
use rsubs_lib::SSA;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("tests/fixtures/test.ass")?;
    let mut ssa = SSA::parse(content)?;

    for style in &mut ssa.styles {
        if style.name == "Default" {
            style.primary_color = Color::new(255, 0, 0, 255)
        }
    }

    fs::write("result.ass", ssa.to_string())?;
    Ok(())
}
