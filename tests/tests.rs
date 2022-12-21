#[cfg(test)]

mod tests {
    #[test]
    fn test_ssa_from_file_to_srt_file() {
        use rsubs_lib::subs::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_srt()
            .to_file("./tests/fixtures/res6.srt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_vtt_file() {
        use rsubs_lib::subs::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res5.vtt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_ass_file() {
        use rsubs_lib::subs::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_file("./tests/fixtures/res4.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_ass_file() {
        use rsubs_lib::subs::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string());
        ssafile
            .to_ass()
            .to_file("./tests/fixtures/res3.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_vtt_file() {
        use rsubs_lib::subs::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string());
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res2.vtt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_default_file() {
        use rsubs_lib::subs::ssa::SSAFile;
        let ssafile = SSAFile::default();
        ssafile
            .to_file("./tests/fixtures/res1.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_text_to_srt_file() {
        use rsubs_lib::subs::ssa;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.ass")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        let ssafile = ssa::parse(file_value.to_string()).unwrap();
        ssafile
            .to_srt()
            .to_file("./tests/fixtures/res7.srt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_srt_file() {
        use rsubs_lib::subs::srt;
        let srtfile = srt::parse("./tests/fixtures/test.srt".to_string());
        srtfile
            .to_file("./tests/fixtures/res8.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_srt_from_text_to_srt_file() {
        use rsubs_lib::subs::srt;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.srt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        let srtfile = srt::parse(file_value.to_string());
        srtfile
            .to_file("./tests/fixtures/res9.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_srt_from_text_to_srt_string() {
        use rsubs_lib::subs::srt;
        use std::fs::File;
        use std::io::Read;
        let file_valuex: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.srt")
            .expect("WrongFile")
            .read_to_string(file_valuex)
            .expect("Couldn't write");
        let srtfile1 = srt::parse(file_valuex.to_string());
        srtfile1
            .to_file("./tests/fixtures/res13.srt".to_string())
            .unwrap();
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/res13.srt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        let file_value2: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.srt")
            .expect("WrongFile")
            .read_to_string(file_value2)
            .expect("Couldn't write");
        let srtfile = srt::parse(file_value.to_string());
        assert_eq!(file_value.to_string(), srtfile.stringify());
    }
    #[test]
    fn test_parse_vtt() {
        use rsubs_lib::subs::vtt;
        use rsubs_lib::subs::vtt::VTTFile;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.vtt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        let _vttfile: VTTFile = vtt::parse(file_value.to_owned());
    }
    #[test]
    fn test_parse_vtt_write_to_vtt() {
        use rsubs_lib::subs::vtt;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.vtt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        vtt::parse(file_value.to_owned())
            .to_file("./tests/fixtures/res10.vtt".to_string())
            .expect("Ok");
    }
    #[test]
    fn test_parse_vtt_write_to_ssa() {
        use rsubs_lib::subs::vtt;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.vtt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        vtt::parse(file_value.to_owned())
            .to_ass()
            .to_file("./tests/fixtures/res11.ass".to_string())
            .expect("Ok");
    }
    #[test]
    fn test_parse_vtt_write_to_srt() {
        use rsubs_lib::subs::vtt;
        use std::fs::File;
        use std::io::Read;
        let file_value: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/test.vtt")
            .expect("WrongFile")
            .read_to_string(file_value)
            .expect("Couldn't write");
        vtt::parse(file_value.to_owned())
            .to_srt()
            .to_file("./tests/fixtures/res12.srt".to_string())
            .expect("Ok");
    }
}

fn main() -> std::io::Result<()> {
    use std::fs;
    let a = vec![
        "res1.ass",
        "res2.vtt",
        "res3.ass",
        "res4.ass",
        "res5.vtt",
        "res6.srt",
        "res7.srt",
        "res8.srt",
        "res9.srt",
        "res10.vtt",
        "res11.ass",
        "res12.srt",
        "res13.srt",
    ];
    for i in a {
        fs::remove_file(i)?;
    }
    Ok(())
}
