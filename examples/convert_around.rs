use rsubs_lib::{SRT, SSA, VTT};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let srt_content = fs::read_to_string("./tests/fixtures/test.srt")?;
    let srt = SRT::parse(srt_content)?;
    fs::write("./tests/fixtures/ex_test_1.vtt", srt.to_vtt().to_string())?;

    let vtt_content = fs::read_to_string("./tests/fixtures/test.vtt")?;
    let vtt = VTT::parse(vtt_content)?;
    fs::write("./tests/fixtures/ex_test_1.ass", vtt.to_ssa().to_string())?;

    let ssa_content = fs::read_to_string("./tests/fixtures/test.ass")?;
    let ssa = SSA::parse(ssa_content)?;
    fs::write("./tests/fixtures/ex_test_1.srt", ssa.to_srt().to_string())?;

    Ok(())
}
