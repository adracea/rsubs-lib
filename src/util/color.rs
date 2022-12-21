use core::panic;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
pub const TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const WHITET: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 0,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 227,
    g: 193,
    b: 41,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 227,
    g: 111,
    b: 4,
    a: 255,
};
pub const PINK: Color = Color {
    r: 255,
    g: 192,
    b: 203,
    a: 255,
};
impl Error for Color {}
impl Default for Color {
    fn default() -> Self {
        WHITE
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn fmt_ass(&self) -> String {
        format!(
            "&H{:0>2X}{:0>2X}{:0>2X}{:0>2X}",
            self.a, self.b, self.g, self.r
        )
    }
    pub fn fmt_ssa(&self) -> String {
        format!("&H{:0>2X}{:0>2X}{:0>2X}", self.b, self.g, self.r)
    }
    pub fn from_ass(str: &str) -> Result<Self, <Color as FromStr>::Err> {
        if str.len() == 5 {
            Ok(Color {
                r: u8::from_str_radix(&str[2..4], 16)?,
                g: 0,
                b: 0,
                a: 255,
            })
        } else if str.len() == 7 {
            Ok(Color {
                r: u8::from_str_radix(&str[4..6], 16)?,
                g: u8::from_str_radix(&str[2..4], 16)?,
                b: 0,
                a: 255,
            })
        } else if str.len() == 9 {
            Ok(Color {
                a: u8::from_str_radix(&str[2..4], 16)?,
                b: u8::from_str_radix(&str[4..6], 16)?,
                g: u8::from_str_radix(&str[6..8], 16)?,
                r: u8::from_str_radix(&str[8..10], 16)?,
            })
        } else {
            panic!("bad color")
        }
    }
}

impl FromStr for Color {
    type Err = std::num::ParseIntError;
    fn from_str(str: &str) -> Result<Self, <Color as FromStr>::Err> {
        if str.starts_with('#') {
            if str.len() == 3 {
                Ok(Color {
                    r: u8::from_str_radix(&str[1..3], 16)?,
                    g: 0,
                    b: 0,
                    a: 0,
                })
            } else if str.len() == 5 {
                Ok(Color {
                    r: u8::from_str_radix(&str[1..3], 16)?,
                    g: u8::from_str_radix(&str[3..5], 16)?,
                    b: 0,
                    a: 0,
                })
            } else if str.len() == 7 {
                Ok(Color {
                    r: u8::from_str_radix(&str[1..3], 16)?,
                    g: u8::from_str_radix(&str[3..5], 16)?,
                    b: u8::from_str_radix(&str[5..7], 16)?,
                    a: 255,
                })
            } else if str.len() == 9 {
                Ok(Color {
                    r: u8::from_str_radix(&str[1..3], 16)?,
                    g: u8::from_str_radix(&str[3..5], 16)?,
                    b: u8::from_str_radix(&str[5..7], 16)?,
                    a: u8::from_str_radix(&str[7..9], 16)?,
                })
            } else {
                panic!("No Color Detected")
            }
        } else if str.starts_with('&') {
            if str.len() == 4 {
                Ok(Color {
                    r: u8::from_str_radix(&str[2..4], 16)?,
                    g: 0,
                    b: 0,
                    a: 0,
                })
            } else if str.len() == 6 {
                Ok(Color {
                    r: u8::from_str_radix(&str[4..6], 16)?,
                    g: u8::from_str_radix(&str[2..4], 16)?,
                    b: 0,
                    a: 0,
                })
            } else if str.len() == 8 {
                Ok(Color {
                    a: 0,
                    b: u8::from_str_radix(&str[4..6], 16)?,
                    g: u8::from_str_radix(&str[6..8], 16)?,
                    r: u8::from_str_radix(&str[8..10], 16)?,
                })
            } else if str.len() == 10 {
                Ok(Color {
                    a: u8::from_str_radix(&str[2..4], 16)?,
                    b: u8::from_str_radix(&str[4..6], 16)?,
                    g: u8::from_str_radix(&str[6..8], 16)?,
                    r: u8::from_str_radix(&str[8..10], 16)?,
                })
            } else {
                panic!("No Color Detected")
            }
        } else {
            panic!("No Color Detected")
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
            self.a, self.r, self.g, self.b
        )
    }
}
#[derive(Debug)]
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
