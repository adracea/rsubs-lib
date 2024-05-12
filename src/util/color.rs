//! This module represents mostly Color related helpers.
//!
//! SSA Colors start with `&H` and can be found in multiple forms:
//!
//! `&HRR`,`&HGGRR`,`&HBBGGRR` or `&HAABBGGRR`.
//!
//! VTT Colors start with `#` and are the usual ARGB or RGB hex formats.
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

/// Generic ARGB color struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub(crate) const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub(crate) const TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub(crate) fn from_ssa(mut color: &str) -> Result<Self, String> {
        if !color.starts_with("&H") || color.len() != 10 {
            return Err(format!("invalid color: #{color}"));
        }
        color = &color[2..];
        Ok(Self {
            r: u8::from_str_radix(&color[6..8], 16).map_err(|e| e.to_string())?,
            g: u8::from_str_radix(&color[4..6], 16).map_err(|e| e.to_string())?,
            b: u8::from_str_radix(&color[2..4], 16).map_err(|e| e.to_string())?,
            a: u8::from_str_radix(&color[0..2], 16).map_err(|e| e.to_string())?,
        })
    }

    pub(crate) fn from_vtt(color: &str) -> Result<Self, String> {
        if let Some(color) = color.strip_prefix('#') {
            let color = match color.len().cmp(&8) {
                Ordering::Greater => return Err(format!("invalid hex color: #{color}")),
                Ordering::Less if color.len() < 7 => format!("#{:0>6}FF", color),
                Ordering::Less => format!("#{:F>8}", color),
                _ => color.to_string(),
            };
            Ok(Self {
                r: u8::from_str_radix(&color[0..2], 16).map_err(|e| e.to_string())?,
                g: u8::from_str_radix(&color[2..4], 16).map_err(|e| e.to_string())?,
                b: u8::from_str_radix(&color[4..6], 16).map_err(|e| e.to_string())?,
                a: u8::from_str_radix(&color[6..8], 16).map_err(|e| e.to_string())?,
            })
        } else {
            // command to get all colors:
            // curl -s https://www.w3schools.com/colors/color_tryit.asp | python3 -c "import re, sys; [print(f'\"{match['name'].lower()}\" => Self::new({int(match[\"hex\"][0:2], 16)}, {int(match[\"hex\"][2:4], 16)}, {int(match[\"hex\"][4:6], 16)}, 255),') for match in re.finditer(r\"<td style='color:#(?P<hex>([0-9A-F]){6})'><b>\s(?P<name>\w+)</b></td>\", sys.stdin.read())]"
            Ok(match color.to_lowercase().as_str() {
                "aliceblue" => Self::new(240, 248, 255, 255),
                "antiquewhite" => Self::new(250, 235, 215, 255),
                "aqua" => Self::new(0, 255, 255, 255),
                "aquamarine" => Self::new(127, 255, 212, 255),
                "azure" => Self::new(240, 255, 255, 255),
                "beige" => Self::new(245, 245, 220, 255),
                "bisque" => Self::new(255, 228, 196, 255),
                "black" => Self::new(0, 0, 0, 255),
                "blanchedalmond" => Self::new(255, 235, 205, 255),
                "blue" => Self::new(0, 0, 255, 255),
                "blueviolet" => Self::new(138, 43, 226, 255),
                "brown" => Self::new(165, 42, 42, 255),
                "burlywood" => Self::new(222, 184, 135, 255),
                "cadetblue" => Self::new(95, 158, 160, 255),
                "chartreuse" => Self::new(127, 255, 0, 255),
                "chocolate" => Self::new(210, 105, 30, 255),
                "coral" => Self::new(255, 127, 80, 255),
                "cornflowerblue" => Self::new(100, 149, 237, 255),
                "cornsilk" => Self::new(255, 248, 220, 255),
                "crimson" => Self::new(220, 20, 60, 255),
                "cyan" => Self::new(0, 255, 255, 255),
                "darkblue" => Self::new(0, 0, 139, 255),
                "darkcyan" => Self::new(0, 139, 139, 255),
                "darkgoldenrod" => Self::new(184, 134, 11, 255),
                "darkgray" => Self::new(169, 169, 169, 255),
                "darkgrey" => Self::new(169, 169, 169, 255),
                "darkgreen" => Self::new(0, 100, 0, 255),
                "darkkhaki" => Self::new(189, 183, 107, 255),
                "darkmagenta" => Self::new(139, 0, 139, 255),
                "darkolivegreen" => Self::new(85, 107, 47, 255),
                "darkorange" => Self::new(255, 140, 0, 255),
                "darkorchid" => Self::new(153, 50, 204, 255),
                "darkred" => Self::new(139, 0, 0, 255),
                "darksalmon" => Self::new(233, 150, 122, 255),
                "darkseagreen" => Self::new(143, 188, 143, 255),
                "darkslateblue" => Self::new(72, 61, 139, 255),
                "darkslategray" => Self::new(47, 79, 79, 255),
                "darkslategrey" => Self::new(47, 79, 79, 255),
                "darkturquoise" => Self::new(0, 206, 209, 255),
                "darkviolet" => Self::new(148, 0, 211, 255),
                "deeppink" => Self::new(255, 20, 147, 255),
                "deepskyblue" => Self::new(0, 191, 255, 255),
                "dimgray" => Self::new(105, 105, 105, 255),
                "dimgrey" => Self::new(105, 105, 105, 255),
                "dodgerblue" => Self::new(30, 144, 255, 255),
                "firebrick" => Self::new(178, 34, 34, 255),
                "floralwhite" => Self::new(255, 250, 240, 255),
                "forestgreen" => Self::new(34, 139, 34, 255),
                "fuchsia" => Self::new(255, 0, 255, 255),
                "gainsboro" => Self::new(220, 220, 220, 255),
                "ghostwhite" => Self::new(248, 248, 255, 255),
                "gold" => Self::new(255, 215, 0, 255),
                "goldenrod" => Self::new(218, 165, 32, 255),
                "gray" => Self::new(128, 128, 128, 255),
                "grey" => Self::new(128, 128, 128, 255),
                "green" => Self::new(0, 128, 0, 255),
                "greenyellow" => Self::new(173, 255, 47, 255),
                "honeydew" => Self::new(240, 255, 240, 255),
                "hotpink" => Self::new(255, 105, 180, 255),
                "ivory" => Self::new(255, 255, 240, 255),
                "khaki" => Self::new(240, 230, 140, 255),
                "lavender" => Self::new(230, 230, 250, 255),
                "lavenderblush" => Self::new(255, 240, 245, 255),
                "lawngreen" => Self::new(124, 252, 0, 255),
                "lemonchiffon" => Self::new(255, 250, 205, 255),
                "lightblue" => Self::new(173, 216, 230, 255),
                "lightcoral" => Self::new(240, 128, 128, 255),
                "lightcyan" => Self::new(224, 255, 255, 255),
                "lightgoldenrodyellow" => Self::new(250, 250, 210, 255),
                "lightgray" => Self::new(211, 211, 211, 255),
                "lightgrey" => Self::new(211, 211, 211, 255),
                "lightgreen" => Self::new(144, 238, 144, 255),
                "lightpink" => Self::new(255, 182, 193, 255),
                "lightsalmon" => Self::new(255, 160, 122, 255),
                "lightseagreen" => Self::new(32, 178, 170, 255),
                "lightskyblue" => Self::new(135, 206, 250, 255),
                "lightslategray" => Self::new(119, 136, 153, 255),
                "lightslategrey" => Self::new(119, 136, 153, 255),
                "lightsteelblue" => Self::new(176, 196, 222, 255),
                "lightyellow" => Self::new(255, 255, 224, 255),
                "lime" => Self::new(0, 255, 0, 255),
                "limegreen" => Self::new(50, 205, 50, 255),
                "linen" => Self::new(250, 240, 230, 255),
                "magenta" => Self::new(255, 0, 255, 255),
                "maroon" => Self::new(128, 0, 0, 255),
                "mediumaquamarine" => Self::new(102, 205, 170, 255),
                "mediumblue" => Self::new(0, 0, 205, 255),
                "mediumorchid" => Self::new(186, 85, 211, 255),
                "mediumpurple" => Self::new(147, 112, 219, 255),
                "mediumseagreen" => Self::new(60, 179, 113, 255),
                "mediumslateblue" => Self::new(123, 104, 238, 255),
                "mediumspringgreen" => Self::new(0, 250, 154, 255),
                "mediumturquoise" => Self::new(72, 209, 204, 255),
                "mediumvioletred" => Self::new(199, 21, 133, 255),
                "midnightblue" => Self::new(25, 25, 112, 255),
                "mintcream" => Self::new(245, 255, 250, 255),
                "mistyrose" => Self::new(255, 228, 225, 255),
                "moccasin" => Self::new(255, 228, 181, 255),
                "navajowhite" => Self::new(255, 222, 173, 255),
                "navy" => Self::new(0, 0, 128, 255),
                "oldlace" => Self::new(253, 245, 230, 255),
                "olive" => Self::new(128, 128, 0, 255),
                "olivedrab" => Self::new(107, 142, 35, 255),
                "orange" => Self::new(255, 165, 0, 255),
                "orangered" => Self::new(255, 69, 0, 255),
                "orchid" => Self::new(218, 112, 214, 255),
                "palegoldenrod" => Self::new(238, 232, 170, 255),
                "palegreen" => Self::new(152, 251, 152, 255),
                "paleturquoise" => Self::new(175, 238, 238, 255),
                "palevioletred" => Self::new(219, 112, 147, 255),
                "papayawhip" => Self::new(255, 239, 213, 255),
                "peachpuff" => Self::new(255, 218, 185, 255),
                "peru" => Self::new(205, 133, 63, 255),
                "pink" => Self::new(255, 192, 203, 255),
                "plum" => Self::new(221, 160, 221, 255),
                "powderblue" => Self::new(176, 224, 230, 255),
                "purple" => Self::new(128, 0, 128, 255),
                "rebeccapurple" => Self::new(102, 51, 153, 255),
                "red" => Self::new(255, 0, 0, 255),
                "rosybrown" => Self::new(188, 143, 143, 255),
                "royalblue" => Self::new(65, 105, 225, 255),
                "saddlebrown" => Self::new(139, 69, 19, 255),
                "salmon" => Self::new(250, 128, 114, 255),
                "sandybrown" => Self::new(244, 164, 96, 255),
                "seagreen" => Self::new(46, 139, 87, 255),
                "seashell" => Self::new(255, 245, 238, 255),
                "sienna" => Self::new(160, 82, 45, 255),
                "silver" => Self::new(192, 192, 192, 255),
                "skyblue" => Self::new(135, 206, 235, 255),
                "slateblue" => Self::new(106, 90, 205, 255),
                "slategray" => Self::new(112, 128, 144, 255),
                "slategrey" => Self::new(112, 128, 144, 255),
                "snow" => Self::new(255, 250, 250, 255),
                "springgreen" => Self::new(0, 255, 127, 255),
                "steelblue" => Self::new(70, 130, 180, 255),
                "tan" => Self::new(210, 180, 140, 255),
                "teal" => Self::new(0, 128, 128, 255),
                "thistle" => Self::new(216, 191, 216, 255),
                "tomato" => Self::new(255, 99, 71, 255),
                "turquoise" => Self::new(64, 224, 208, 255),
                "violet" => Self::new(238, 130, 238, 255),
                "wheat" => Self::new(245, 222, 179, 255),
                "white" => Self::new(255, 255, 255, 255),
                "whitesmoke" => Self::new(245, 245, 245, 255),
                "yellow" => Self::new(255, 255, 0, 255),
                "yellowgreen" => Self::new(154, 205, 50, 255),
                _ => return Err(format!("unknown color name: {color}")),
            })
        }
    }

    pub fn to_ssa_string(&self) -> String {
        if self.a == 255 {
            format!("&H{:0>2X}{:0>2X}{:0>2X}", self.b, self.g, self.r)
        } else {
            format!(
                "&H{:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                self.a, self.b, self.g, self.r
            )
        }
    }

    pub fn to_vtt_string(&self) -> String {
        if self.a == 255 {
            format!("#{:0>2X}{:0>2X}{:0>2X}", self.r, self.g, self.b)
        } else {
            format!(
                "#{:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                self.r, self.g, self.b, self.a
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    BottomLeft = 1,
    BottomCenter = 2,
    BottomRight = 3,
    MiddleLeft = 4,
    MiddleCenter = 5,
    MiddleRight = 6,
    TopLeft = 7,
    TopCenter = 8,
    TopRight = 9,
}

impl Alignment {
    pub fn infer_from_str(str: &str) -> Result<Self, &'static str> {
        match str {
            "1" => Ok(Alignment::BottomLeft),
            "2" => Ok(Alignment::BottomCenter),
            "3" => Ok(Alignment::BottomRight),
            "4" => Ok(Alignment::MiddleLeft),
            "5" => Ok(Alignment::MiddleCenter),
            "6" => Ok(Alignment::MiddleRight),
            "7" => Ok(Alignment::TopLeft),
            "8" => Ok(Alignment::TopCenter),
            "9" => Ok(Alignment::TopRight),
            &_ => Err("ParseIntError"),
        }
    }
}
