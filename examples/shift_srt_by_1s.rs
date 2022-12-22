// In this example we read a .srt file and add 1s(1000ms) to each line
// Afterwards we print the result to stdout.

fn main() {
    let mut srt = rsubs_lib::srt::parse("tests/fixtures/test.srt".to_string());
    for line in srt.lines.iter_mut() {
        line.line_end += 1000;
        line.line_start += 1000;
    }
    println!("{}", srt.stringify());
}
