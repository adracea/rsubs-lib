#[cfg(test)]

mod tests {
    // use core::time;

    use std::{mem::take, str::FromStr};

    use rsubs_lib::util::{
        color::{Alignment, Color},
        time::Time,
    };

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
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
        ssafile
            .to_ass()
            .to_file("./tests/fixtures/res3.ass".to_string())
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_vtt_file() {
        use rsubs_lib::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
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
        let srtfile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
        srtfile
            .to_file("./tests/fixtures/res8.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_srt_from_file_to_srt_file2() {
        use rsubs_lib::srt;
        srt::parse("./tests/fixtures/test.srt".to_string())
            .unwrap()
            .to_vtt()
            .to_file("./tests/fixtures/res14.srt".to_string())
            .unwrap();
    }
    #[test]
    fn test_time() {
        let fi = "
        1
00:11.000 --> 00:13.000
We are in New York City

2
00:13.000 --> 00:16.000
We’re actually at the Lucern Hotel, just down the street

3
00:16.000 --> 00:18.000
from the American Museum of Natural History

4
00:18.000 --> 00:20.000
And with me is Neil deGrasse Tyson

5
00:20.000 --> 00:22.000
Astrophysicist, Director of the Hayden Planetarium

6
00:22.000 --> 00:24.000
at the AMNH.

7
00:24.000 --> 00:26.000
Thank you for walking down here.

8
00:27.000 --> 00:30.000
And I want to do a follow-up on the last conversation we did.

9
00:30.000 --> 00:31.500
When we e-mailed—

10
00:30.500 --> 00:32.500
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?

11
00:32.000 --> 00:35.500
No! No no no no; 'cos 'cos obviously 'cos

12
00:32.500 --> 00:33.500
<i>Laughs</i>

13
00:35.500 --> 00:38.000
You know I’m so excited my glasses are falling off here.
        "
        .to_string();
        let mut srt = rsubs_lib::srt::parse(fi.to_string()).unwrap();
        let srt2 = rsubs_lib::srt::parse(fi).unwrap();
        for line in srt.lines.iter_mut() {
            line.line_end += 1000;
            line.line_start += 1000;
        }
        for (ctr, line) in srt.lines.iter().enumerate() {
            assert_eq!(
                line.line_start,
                srt2.lines.get(ctr).unwrap().clone().line_start + 1000
            );
            assert_eq!(
                line.line_end,
                srt2.lines.get(ctr).unwrap().clone().line_end + 1000
            );
            assert_eq!(
                line.line_start,
                srt2.lines.get(ctr).unwrap().clone().line_start + 1000_u32
            );
            assert_eq!(
                line.line_end,
                srt2.lines.get(ctr).unwrap().clone().line_end + 1000_u32
            );
        }
    }
    #[test]
    fn test_time_2() {
        use rsubs_lib::util::time;
        let timestr = vec!["00:00:20.000", "00:01:20.011", "0:00:05,100", "00:20,40"];
        for (ctr, i) in timestr.iter().enumerate() {
            match ctr {
                0 => assert_eq!(
                    Time {
                        h: 0,
                        m: 0,
                        s: 20,
                        ms: 0,
                        frames: 0,
                        fps: 0.0,
                    },
                    Time::from_str(i).unwrap()
                ),
                1 => assert_eq!(
                    Time {
                        h: 0,
                        m: 1,
                        s: 20,
                        ms: 11,
                        frames: 0,
                        fps: 0.0,
                    },
                    Time::from_str(i).unwrap()
                ),
                2 => assert_eq!(
                    Time {
                        h: 0,
                        m: 0,
                        s: 5,
                        ms: 100,
                        frames: 0,
                        fps: 0.0,
                    },
                    Time::from_str(i).unwrap()
                ),
                3 => assert_eq!(
                    Time {
                        h: 0,
                        m: 0,
                        s: 20,
                        ms: 400,
                        frames: 0,
                        fps: 0.0,
                    },
                    Time::from_str(i).unwrap()
                ),
                _ => todo!(),
            }
        }
        let mut tr = Time::from_str(timestr.first().unwrap()).unwrap();
        assert_eq!(((tr.clone() + 1000_u32) as Time).total_ms(), 21000_u32);
        assert_eq!(((tr.clone() - 1000_u32) as Time).total_ms(), 19000_u32);
        assert_eq!(((tr.clone() + 1000_i32) as Time).total_ms(), 21000);
        assert_eq!(((tr.clone() - 1000_i32) as Time).total_ms(), 19000);
        tr += 1000_u32;
        assert_eq!(tr.total_ms(), 21000_u32);
        tr += 1000_i32;
        let mut test: &mut Time = &mut tr;
        test = test - 1000_i32;
        assert_eq!(test.total_ms(), 21000_u32);
        test = test - 1000_u32;
        assert_eq!(test.total_ms(), 20000_u32);
        take(test);
        assert_eq!(tr.total_ms(), 0_u32);
        tr += 22000;
        let a: &mut Time = &mut tr;
        let b = a + 100;
        let mut d: &mut Time = &mut Time::default();
        println!("{}", b);
        assert_eq!(b.clone().to_srt_string(), "00:00:22,100".to_string());
        assert_eq!(&mut b.clone() - 100000, d);
        assert_eq!(b - 100000_u32, d);
        d.set_fps(27.9);
        assert_eq!(d.fps, 27.9);
        assert_eq!(d.frames, 0);
        d = d + 10000;
        d.derive_frames();
        assert_eq!(d.frames, 279);
        d.update_from_fps_frames().unwrap();
        assert_eq!(time::frames_to_ms(23, 0.0), 0);
    }

    #[test]
    fn test_srt_from_file_to_srt_file3() {
        use rsubs_lib::srt;
        use std::fs::File;
        use std::io::Read;
        srt::parse("./tests/fixtures/test.srt".to_string())
            .unwrap()
            .to_vtt()
            .to_ass()
            .to_srt()
            .to_file("./tests/fixtures/res15.srt".to_string())
            .unwrap();
        let file_value = srt::parse("./tests/fixtures/test.srt".to_string())
            .unwrap()
            .stringify();
        let file_value2: &mut String = &mut "".to_string();
        File::open("./tests/fixtures/res15.srt")
            .expect("WrongFile")
            .read_to_string(file_value2)
            .expect("Couldn't write");
        // assert_eq!(file_value, file_value2.to_string());
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
        let srtfile = srt::parse(file_value.to_string()).unwrap();
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
        let srtfile1 = srt::parse(file_valuex.to_string()).unwrap();
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
        let srtfile = srt::parse(file_value.to_string()).unwrap();
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
        let _vttfile: VTTFile = vtt::parse(file_value.to_owned()).unwrap();
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
            .unwrap()
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
            .unwrap()
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
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res12.srt".to_string())
            .expect("Ok");
    }
    #[test]
    fn test_parse_vtt_from_file_to_srt() {
        use rsubs_lib::vtt;
        vtt::parse("./tests/fixtures/test.vtt".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res16.srt".to_string())
            .expect("Ok");
    }
    #[test]
    #[should_panic]
    fn test_parse_vtt_from_file_to_srt_panic() {
        use rsubs_lib::vtt;
        vtt::parse("./tests/fixtures/test.srt".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res12.srt".to_string())
            .expect("Ok");
    }
    #[test]
    #[should_panic]
    fn test_parse_vtt_from_empty_to_srt_panic() {
        use rsubs_lib::vtt;
        vtt::parse("".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res_panic1.srt".to_string())
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
        let e = color::Color::default();
        assert_eq!(e, color::WHITE);
        assert_eq!(a.get_color(), color::WHITE);
        assert_eq!(b.get_color(), color::WHITE);
        assert_eq!(c.get_color(), color::WHITE);
        assert_eq!(d.get_color(), color::WHITE);
        let mut h = Color::from_str("&HAA").unwrap();
        assert_eq!(
            h,
            Color {
                r: 170,
                g: 0,
                b: 0,
                a: 255
            }
        );
        h = Color::from_str("&HAAAB").unwrap();
        assert_eq!(
            h,
            Color {
                r: 171,
                g: 170,
                b: 0,
                a: 255
            }
        );
        h = Color::from_str("&HAAABAC").unwrap();
        assert_eq!(
            h,
            Color {
                r: 172,
                g: 171,
                b: 170,
                a: 255
            }
        );
        h = Color::from_str("#AA").unwrap();
        assert_eq!(
            h,
            Color {
                r: 170,
                g: 0,
                b: 0,
                a: 255
            }
        );
        h = Color::from_str("#AAAB").unwrap();
        assert_eq!(
            h,
            Color {
                r: 170,
                g: 171,
                b: 0,
                a: 255
            }
        );
        h = Color::from_str("#AAABAC").unwrap();
        assert_eq!(
            h,
            Color {
                r: 170,
                g: 171,
                b: 172,
                a: 255
            }
        );
        h = Color::from_str("#AAABACAD").unwrap();
        assert_eq!(
            h,
            Color {
                r: 171,
                g: 172,
                b: 173,
                a: 170
            }
        );
        let str = format!("{}", h);
        assert_eq!(str, "#AAABACAD");
    }
    #[test]
    #[should_panic]
    fn colors_panic1() {
        let _h = Color::from_str("#AAABAAAAAAAAC").unwrap();
    }
    #[test]
    #[should_panic]
    fn colors_panic2() {
        let _h = Color::from_str("AAABAAAAAAAAC").unwrap();
    }
    #[test]
    #[should_panic]
    fn colors_panic3() {
        let _h = Color::from_str("&HAAABAAAAAAAAC").unwrap();
    }
    #[test]
    #[should_panic]
    fn alignment_panic() {
        let _h = Alignment::infer_from_str("&HAAABAAAAAAAAC").unwrap();
    }
    #[test]
    fn alignments() {
        let mut a = Alignment::infer_from_str("1").unwrap();
        assert_eq!(a, Alignment::BottomLeft);
        a = Alignment::infer_from_str("2").unwrap();
        assert_eq!(a, Alignment::BottomCenter);
        a = Alignment::infer_from_str("3").unwrap();
        assert_eq!(a, Alignment::BottomRight);
        a = Alignment::infer_from_str("4").unwrap();
        assert_eq!(a, Alignment::MiddleLeft);
        a = Alignment::infer_from_str("5").unwrap();
        assert_eq!(a, Alignment::MiddleCenter);
        a = Alignment::infer_from_str("6").unwrap();
        assert_eq!(a, Alignment::MiddleRight);
        a = Alignment::infer_from_str("7").unwrap();
        assert_eq!(a, Alignment::TopLeft);
        a = Alignment::infer_from_str("8").unwrap();
        assert_eq!(a, Alignment::TopCenter);
        a = Alignment::infer_from_str("9").unwrap();
        assert_eq!(a, Alignment::TopRight);
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
