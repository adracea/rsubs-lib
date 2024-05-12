use rsubs_lib::util::Color;
use rsubs_lib::VTT;

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
    let mut vtt = VTT::parse(VTT)?;

    for style in &mut vtt.styles {
        style.entries.insert(
            "color".to_string(),
            Color::new(255, 0, 0, 255).to_vtt_string(),
        );
    }
    for line in &mut vtt.lines {
        line.text.push_str("!!!!!!")
    }

    println!("\n\nVTT altered:\n{vtt}");
    Ok(())
}
