#[cfg(test)]

mod tests {
    // use core::time;

    use std::{mem::take, str::FromStr};

    use rsubs_lib::{
        srt::SRTFile,
        ssa::SSAFile,
        util::{
            color::{Alignment, Color},
            time::Time,
        },
        vtt::VTTFile,
        Subtitle,
    };

    #[test]
    fn test_ssa_from_file_to_srt_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_srt()
            .to_file("./tests/fixtures/res6.srt")
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_vtt_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res5.vtt")
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_ass_file() {
        use rsubs_lib::ssa;
        let ssafile = ssa::parse("./tests/fixtures/test.ass".to_string()).unwrap();
        ssafile
            .to_file("./tests/fixtures/res4.ass")
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_ass_file() {
        use rsubs_lib::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
        ssafile
            .to_ass()
            .to_file("./tests/fixtures/res3.ass")
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_vtt_file() {
        use rsubs_lib::srt;
        let ssafile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
        ssafile
            .to_vtt()
            .to_file("./tests/fixtures/res2.vtt")
            .expect("Couldn't write");
    }
    #[test]
    fn test_ssa_from_file_to_default_file() {
        use rsubs_lib::ssa::SSAFile;
        let ssafile = SSAFile::default();
        ssafile
            .to_file("./tests/fixtures/res1.ass")
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
            .to_file("./tests/fixtures/res7.srt")
            .expect("Couldn't write");
    }
    #[test]
    fn test_srt_from_file_to_srt_file() {
        use rsubs_lib::srt;
        let srtfile = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
        srtfile.to_file("./tests/fixtures/res8.srt").unwrap();
    }
    #[test]
    fn test_srt_from_file_to_srt_file2() {
        use rsubs_lib::srt;
        srt::parse("./tests/fixtures/test.srt".to_string())
            .unwrap()
            .to_vtt()
            .to_file("./tests/fixtures/res14.srt")
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
        let mut srt = rsubs_lib::srt::SRTFile::from_str(&fi).unwrap();
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
    fn test_gen() {
        let fi = r#"1
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
You know I’m so excited my glasses are falling off here."#;
        let fi2 = r#"WEBVTT

NOTE This is safe

STYLE
::cue(Default){
color: #ffffff;
font-family: "Arial", sans-serif;
font-size: 12px;
}

STYLE
::cue(Default2){
color: #ffffff;
font-family: "Arial", sans-serif;
background-color: #00000000;
font-size: 020px;
text-shadow: #000000ff -2px 0px 2px, #000000ff 0px 2px 2px, #000000ff 0px -2px 2px, #000000ff 2px 0px 2px;
}

STYLE
::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City

"#;
        let fi3 = r#"[Script Info]
Collisions: Normal
Synch Point: 
PlayResX: 640
WrapStyle: 0
ScriptType: V4.00+
Title: subtitle
PlayResY: 480
Script Updated By: rsubs lib

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De2,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De4,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,{\c1}Lorem Ipsum2{\c0}"#;
        let sub2: Subtitle = Subtitle::from_str("./tests/fixtures/test.srt").unwrap();
        let sub3: SRTFile = sub2.clone().into();
        let sub4: rsubs_lib::ssa::SSAFile = sub2.clone().into();
        let sub5: VTTFile = sub2.clone().into();
        let _sub6 = Subtitle::from_str("./tests/fixtures/test.ass").unwrap();
        let _sub7 = Subtitle::from_str("./tests/fixtures/test.vtt").unwrap();
        let _sub8: SRTFile = Subtitle::from_str(fi).unwrap().into();
        let _sub9: rsubs_lib::ssa::SSAFile = Subtitle::from_str(fi).unwrap().into();
        let _sub9: rsubs_lib::ssa::SSAFile = Subtitle::from_str(fi2).unwrap().into();
        let _sub9: rsubs_lib::ssa::SSAFile = Subtitle::from_str(fi3).unwrap().into();
        let _sub9: VTTFile = Subtitle::from_str(fi).unwrap().into();
        let _sub9: VTTFile = Subtitle::from_str(fi2).unwrap().into();
        let _sub9: VTTFile = Subtitle::from_str(fi3).unwrap().into();
        let _sub9: SRTFile = Subtitle::from_str(fi).unwrap().into();
        let _sub9: SRTFile = Subtitle::from_str(fi2).unwrap().into();
        let _sub9: SRTFile = Subtitle::from_str(fi3).unwrap().into();
        let _sub10: VTTFile = Subtitle::from_str(fi).unwrap().into();
        let _sub11 = Subtitle::from_str(fi2).unwrap().to_string();
        let _sub11 = Subtitle::from_str(fi3).unwrap().to_string();
        let _sub12 = rsubs_lib::ssa::parse(fi3.replace("\r\n", "\n")).unwrap();
        assert!(sub2.to_string().contains("We are in New York City"));
        assert!(sub3.to_string().contains("We are in New York City"));
        assert!(sub4.to_string().contains("We are in New York City"));
        assert!(sub5.to_string().contains("We are in New York City"));
    }
    #[test]
    #[should_panic]
    fn panic_test() {
        let fi4 = r#"[Script Info]
Collisions: Normal
Synch Point: 
PlayResX: 640
WrapStyle: 0
ScriptType: V4.00+
Title: subtitle
PlayResY: 480
Script Updated By: rsubs lib


STYLE
::cue(Default){
color: #ffffff;
font-family: "Arial", sans-serif;
font-size: 12px;
}

STYLE
::cue(Default2){
color: #ffffff;
font-family: "Arial", sans-serif;
background-color: #00000000;
font-size: 020px;
text-shadow: #000000ff -2px 0px 2px, #000000ff 0px 2px 2px, #000000ff 0px -2px 2px, #000000ff 2px 0px 2px;
}

STYLE
::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City

"#;
        // let sub9: rsubs_lib::ssa::SSAFile = Subtitle::from_str(fi4).unwrap().into();
        // let sub10: VTTFile = Subtitle::from_str(fi4).unwrap().into();
        // let sub9: SRTFile = Subtitle::from_str(fi4).unwrap().into();
        let _sub9: VTTFile = VTTFile::from_str(fi4).unwrap();
    }
    #[test]
    #[should_panic]
    fn panic_sub3() {
        let _sub = Subtitle::SRT(None).to_string();
    }
    #[test]
    #[should_panic]
    fn panic_sub4() {
        let _sub: SSAFile = Subtitle::SRT(None).into();
    }
    #[test]
    #[should_panic]
    fn panic_sub5() {
        let _sub: VTTFile = Subtitle::SRT(None).into();
    }
    #[test]
    #[should_panic]
    fn panic_sub6() {
        let _sub: SRTFile = Subtitle::SRT(None).into();
    }
    // #[test]
    // #[should_panic]
    // fn panic_sub7() {}
    #[test]
    #[should_panic]
    fn panic_vtt2() {
        let fi2 = r#"WEBVTT

NOTE This is safe

STYLE
::cue(Default){
color: #ffffff;
font-family: "Arial", sans-serif;
font-size: 12px;
}

STYLE
::cue(Default2){
color: #ffffff;
font-family: "Arial", sans-serif;
background-color: #00000000;
font-size: 020px;
text-shadow: #000000ff -2px 0px 2px, #000000ff 0px 2px 2px, #000000ff 0px -2px 2px, #000000ff 2px 0px 2px;
}

STYLE
::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

::cue{
color: #ffffff;
font-family: "Arial", sans-serif;
}

00:11.000 --> 00:0:0:0
<v Roger Bingham>We are in New York City

"#;
        let _sub9: VTTFile = VTTFile::from_str(fi2).unwrap();
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
        println!("{b}");
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
            .to_file("./tests/fixtures/res15.srt")
            .unwrap();
        let _file_value = srt::parse("./tests/fixtures/test.srt".to_string()).unwrap();
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
        srtfile.to_file("./tests/fixtures/res9.srt").unwrap();
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
        srtfile1.to_file("./tests/fixtures/res13.srt").unwrap();
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
        assert_eq!(file_value.to_string(), format!("{srtfile}"));
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
            .to_file("./tests/fixtures/res10.vtt")
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
            .to_file("./tests/fixtures/res11.ass")
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
            .to_file("./tests/fixtures/res12.srt")
            .expect("Ok");
    }
    #[test]
    fn test_parse_vtt_from_file_to_srt() {
        use rsubs_lib::vtt;
        vtt::parse("./tests/fixtures/test.vtt".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res16.srt")
            .expect("Ok");
    }
    #[test]
    #[should_panic]
    fn test_parse_vtt_from_file_to_srt_panic() {
        use rsubs_lib::vtt;
        vtt::parse("./tests/fixtures/test.srt".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res12.srt")
            .expect("Ok");
    }
    #[test]
    #[should_panic]
    fn test_parse_vtt_from_empty_to_srt_panic() {
        use rsubs_lib::vtt;
        vtt::parse("".to_owned())
            .unwrap()
            .to_srt()
            .to_file("./tests/fixtures/res_panic1.srt")
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
        let str = format!("{h}");
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
