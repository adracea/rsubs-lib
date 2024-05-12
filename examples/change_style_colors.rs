use rsubs_lib::util::Color;
/// In this example we read a .ass file and add change the color of the `Default` style to red
/// Afterwards we write the file to a new file called `result.ass` .
use rsubs_lib::SSA;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ssa = SSA::parse(SSA)?;

    for style in &mut ssa.styles {
        if style.name == "Default" {
            style.primary_color = Color::new(255, 0, 0, 255)
        }
    }

    println!("\n\nSSA with other color:\n{ssa}");

    Ok(())
}
