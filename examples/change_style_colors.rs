/// In this example we read a .ass file and add change the color of the `Default` style to red
/// Afterwards we write the file to a new file called `result.ass` .
use rsubs_lib::util::color;
use rsubs_lib::util::color::ColorType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ssa = rsubs_lib::ssa::parse("tests/fixtures/test.ass".to_string())
        .expect("Encountered Error parsing file");
    for style in ssa.styles.iter_mut() {
        if style.name == "Default" {
            style.firstcolor = ColorType::SSAColor(color::RED);
        }
    }
    ssa.to_file("result.ass".to_string())?;
    Ok(())
}
