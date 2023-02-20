# rsubs-lib

![ci](https://github.com/adracea/rsubs-lib/actions/workflows/rust.yml/badge.svg)
![clippy](https://github.com/adracea/rsubs-lib/actions/workflows/rust-clippy.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/rsubs-lib.svg)](https://crates.io/crates/rsubs-lib)
[![Crates.io](https://img.shields.io/docsrs/rsubs-lib)](https://docs.rs/rsubs-lib)
![CodeCoverage](https://img.shields.io/codecov/c/github/adracea/rsubs-lib)

A rust library for converting and changing subtitles
Currently a work in progress but it should be able to produce usable .srt,.vtt and .ssa/ass files from one another.


## Usage:

```rust
use rsubs_lib::srt;
use rsubs_lib::ssa;
use rsubs_lib::vtt;
use rsubs_lib::Subtitle;
use std::str::FromStr;

fn main() {
    srt::parse("./tests/fixtures/test.srt".to_string())
        .unwrap() // Can read either a file or a string
        .to_vtt() // converts file to WEBVTT
        .to_file("./tests/fixtures/test_1.vtt") // Writes the converted subtitle to a file
        .unwrap();
    vtt::parse("./tests/fixtures/test.vtt")
        .unwrap()
        .to_ass() // converts file to SSA/ASS
        .to_file("./tests/fixtures/test_1.ass")
        .unwrap();
    ssa::parse("./tests/fixtures/test.ass".to_string())
        .unwrap()
        .to_srt() // converts file to SRT
        .to_file("./tests/fixtures/test_1.srt")
        .unwrap();
    // OR Using the simpler `Subtitle` enum
    let sub: vtt::VTTFile = Subtitle::from_str("./tests/fixtures/test.ass").unwrap().into();
    sub.to_file("./tests/fixtures/test.vtt").unwrap();
}
```

More examples are provided in the `examples` folder.


## Current features:

- Changing colors
- Shifting timestamps
- Changing line text
- In SSA and VTT format changing line style is also possible