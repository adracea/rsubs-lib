#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rsubs_lib;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rsubs_lib::vtt::parse(s.to_string());
    }
});
