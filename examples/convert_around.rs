use rsubs_lib::{SRT, SSA, VTT};

const SRT: &str = "1
00:00:11,000 --> 00:00:13,000
We are in New York City

2
00:00:13,000 --> 00:00:16,000
We’re actually at the Lucern Hotel, just down the street

3
00:00:16,000 --> 00:00:18,000
from the American Museum of Natural History

4
00:00:18,000 --> 00:00:20,000
And with me is Neil deGrasse Tyson

5
00:00:20,000 --> 00:00:22,000
Astrophysicist, Director of the Hayden Planetarium

6
00:00:22,000 --> 00:00:24,000
at the AMNH.

7
00:00:24,000 --> 00:00:26,000
Thank you for walking down here.

8
00:00:27,000 --> 00:00:30,000
And I want to do a follow-up on the last conversation we did.

9
00:00:30,000 --> 00:00:31,500
When we e-mailed—

10
00:00:30,500 --> 00:00:32,500
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?
Didn’t we talk about enough in that conversation?

11
00:00:32,000 --> 00:00:35,500
No! No no no no; 'cos 'cos obviously 'cos

12
00:00:32,500 --> 00:00:33,500
<i>Laughs</i>

13
00:00:35,500 --> 00:00:38,000
You know I’m so excited my glasses are falling off here.";

const SSA: &str = r#"[Script Info]
Collisions: Normal
Synch Point: 
PlayResX: 640
WrapStyle: 0
ScriptType: V4.00+
Title: subtitle
PlayResY: 480

[V4+ Styles]
Format: Name,Fontname,Fontsize,PrimaryColour,SecondaryColour,OutlineColour,BackColour,Bold,Italic,Underline,Strikeout,ScaleX,ScaleY,Spacing,Angle,BorderStyle,Outline,Shadow,Alignment,MarginL,MarginR,MarginV,Encoding
Style: Default,Arial,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De2,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0
Style: De4,Trebuchet MS,25.5,&H00FFFFFF,&H00000000,&H00000000,&H00000000,-1,0,0,0,120,120,0,0,1,1,1,2,0000,0000,0030,0

[Events]
Format: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
Dialogue: 0,0:00:00.20,0:00:02.20,Default,,0000,0000,0000,,{\i1}Lorem Ipsum1{\i0}
Dialogue: 0,0:00:02.20,0:00:04.20,Default,,0000,0000,0000,,{\c1}Lorem Ipsum2{\c0}
Dialogue: 0,0:00:06.20,0:00:08.20,Default,,0000,0000,0000,,Lorem Ipsum3
Dialogue: 0,0:00:08.20,0:00:10.20,Default,,0000,0000,0000,,Lorem Ipsum4
Dialogue: 0,0:00:12.20,0:00:14.20,Default,,0000,0000,0000,,Lorem Ipsum5
Dialogue: 0,0:00:14.20,0:00:16.20,Default,,0000,0000,0000,,Lorem Ipsum6"#;

const VTT: &str = r#"WEBVTT

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

00:11.000 --> 00:13.000
<v Roger Bingham>We are in New York City

00:13.000 --> 00:16.000
<v Roger Bingham>We’re actually at the Lucern Hotel, just down the street

00:16.000 --> 00:18.000
<v Roger Bingham>from the American Museum of Natural History

00:18.000 --> 00:20.000
<v Roger Bingham>And with me is Neil deGrasse Tyson

00:20.000 --> 00:22.000
<v Roger Bingham>Astrophysicist, Director of the Hayden Planetarium

aa
00:22.000 --> 00:24.000
<v Roger Bingham>at the AMNH.

00:24.000 --> 00:26.000
<v Roger Bingham>Thank you for walking down here.

00:27.000 --> 00:30.000
<v Roger Bingham>And I want to do a follow-up on the last conversation we did.

00:30.000 --> 00:31.500 position:30% left align:right size:50% line:12
<v Roger Bingham>When we e-mailed—

00:30.500 --> 00:32.500 align:left size:50% position:030%
<v Neil deGrasse Tyson>Didn’t we talk about enough in that conversation?
<v Neil deGrasse Tyson>Didn’t we talk about enough in that conversation?
<v Neil deGrasse Tyson>Didn’t we talk about enough in that conversation?

00:32.000 --> 00:35.500 align:right size:50%
<v Roger Bingham>No! No no no no; 'cos 'cos obviously 'cos

00:32.500 --> 00:33.500 align:left size:50%
<v Neil deGrasse Tyson><i>Laughs</i>

00:35.500 --> 00:38.000
<v Roger Bingham>You know I’m so excited my glasses are falling off here."#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let srt = SRT::parse(SRT)?;
    println!("\n\nSRT as VTT:\n{}", srt.to_vtt());

    let vtt = VTT::parse(VTT)?;
    println!("\n\nVTT as SSA:\n{}", vtt.to_ssa());

    let ssa = SSA::parse(SSA)?;
    println!("\n\nSSA as SRT:\n{}", ssa.to_srt());

    Ok(())
}
