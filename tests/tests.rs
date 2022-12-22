#[cfg(test)]

mod tests {
    #[test]
    fn test_ssa_from_file_to_srt_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_srt()
            .to_file("./tests/fixtures/res6.srt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_vtt_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res5.vtt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_ass_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_file("./tests/fixtures/res4.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_ass_file() {
        use rsubs_lib::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string());
        ssafile
            .to_ass()
            .to_file("./tests/fixtures/res3.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_vtt_file() {
        use rsubs_lib::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string());
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res2.vtt".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_default_file() {
        use rsubs_lib::ssa::SSAFile;
        let ssafile = SSAFile::default();
        ssafile
            .to_file("./tests/fixtures/res1.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_text_to_srt_file() {
        use rsubs_lib::ssa;
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
        use rsubs_lib::srt;
        let srtfile = srt::parse("./tests/fixtures/test.srt".to_string());
        srtfile
            .to_file("./tests/fixtures/res8.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_srt_from_file_to_srt_file2() {
        use rsubs_lib::srt;
        srt::parse("./tests/fixtures/test.srt".to_string())
            .to_vtt()
            .to_file("./tests/fixtures/res14.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_srt_from_file_to_srt_file3() {
        use rsubs_lib::srt;
        use std::fs::File;
        use std::io::Read;
        srt::parse("./tests/fixtures/test.srt".to_string())
            .to_vtt()
            .to_ass()
            .to_srt()
            .to_file("./tests/fixtures/res15.srt".to_string())
            .unwrap();
        let file_value = srt::parse("./tests/fixtures/test.srt".to_string()).stringify();
        let file_value2: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/res15.srt")
            .expect("WrongFile")
            .read_to_string(file_value2)
            .expect("Couldn't write");
        assert_eq!(file_value, file_value2.to_string());
    }
    #[test]
    fn test_srt_from_text_to_srt_file() {
        use rsubs_lib::srt;
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
        use rsubs_lib::srt;
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
        use rsubs_lib::vtt;
        use rsubs_lib::vtt::VTTFile;
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
        use rsubs_lib::vtt;
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
        use rsubs_lib::vtt;
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
        use rsubs_lib::vtt;
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
    #[test]
    fn colors_test() {
        use rsubs_lib::util::color;
        let a = color::ColorType::SSAColor(rsubs_lib::util::color::WHITE);
        let b = color::ColorType::SSAColor0A(rsubs_lib::util::color::WHITE);
        let c = color::ColorType::VTTColor(rsubs_lib::util::color::WHITE);
        let d = color::ColorType::VTTColor0A(rsubs_lib::util::color::WHITE);
        assert_eq!("&HFFFFFFFF", a.to_string());
        assert_eq!("&HFFFFFF", b.to_string());
        assert_eq!("#FFFFFFFF", c.to_string());
        assert_eq!("#FFFFFF", d.to_string());
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
