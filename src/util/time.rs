//! Helper module to parse, print, manipulate timestamps.
//!
//! SSA has a timestamp representation of `00:00:00.00`
//!
//! VTT has a timestamp representation of `00:00:00.000`
//!
//! SRT has a timestamp representation of `00:00:00,000`

use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::{Add, SubAssign};
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    h: u32,
    m: u32,
    s: u32,
    ms: u32,
    fps: f32,
}
impl Eq for Time {}
impl Default for Time {
    fn default() -> Self {
        let t = Time {
            h: 0,
            m: 0,
            s: 0,
            ms: 0,
            fps: 0.0,
        };
        t
    }
}

impl FromStr for Time {
    type Err = std::num::ParseIntError;
    fn from_str(str: &str) -> Result<Self, <Time as FromStr>::Err> {
        let mut t = Time::default();
        let splits = str.split(':').collect::<Vec<&str>>();
        match splits.len() {
            2 => {
                t.h = 0;
                t.m = splits.first().unwrap_or(&"0").to_string().parse::<u32>()?;
                let sms = splits
                    .get(1)
                    .unwrap_or(&"0.0")
                    .replace(',', ".")
                    .trim()
                    .parse::<f32>()
                    .unwrap();
                let fms = format!("{sms:.3}");
                let msf = fms.split('.').collect::<Vec<&str>>();
                t.s = msf.first().unwrap_or(&"0").to_string().parse::<u32>()?;
                t.ms = msf.get(1).unwrap_or(&"0").to_string().parse::<u32>()?;
            }
            3 => {
                t.h = splits.first().unwrap_or(&"0").to_string().parse::<u32>()?;
                t.m = splits.get(1).unwrap_or(&"0").to_string().parse::<u32>()?;
                let sms = splits
                    .get(2)
                    .unwrap_or(&"0.0")
                    .replace(',', ".")
                    .trim()
                    .parse::<f32>()
                    .unwrap();
                let fms = format!("{sms:.3}");
                let msf = fms.split('.').collect::<Vec<&str>>();
                t.s = msf.first().unwrap_or(&"0").to_string().parse::<u32>()?;
                t.ms = msf.get(1).unwrap_or(&"0").to_string().parse::<u32>()?;
            }
            _ => {
                panic!("Bad Time")
            }
        }

        Ok(t)
    }
}

impl std::error::Error for Time {}
impl Time {
    pub fn from_ms(ms: u32) -> Self {
        let mut this = Self::default();
        this.set_ms(ms);
        this
    }

    pub fn total_ms(&self) -> u32 {
        self.h * 3600000 + self.m * 60000 + self.s * 1000 + self.ms
    }
    pub fn frames(&self) -> u32 {
        if self.fps == 0.0 {
            0
        } else {
            ((self.total_ms() as f32) * self.fps / 1000.0).round() as u32
        }
    }
    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
    }
    pub fn set_ms(&mut self, ms: u32) {
        const HOUR_DIVIDER: u32 = 1000 * 60 * 60;
        const MINUTE_DIVIDER: u32 = 1000 * 60;
        const SECOND_DIVIDER: u32 = 1000;

        self.h = ms / HOUR_DIVIDER;
        self.m = (ms - (self.h * HOUR_DIVIDER)) / MINUTE_DIVIDER;
        self.s = (ms - (self.h * HOUR_DIVIDER) - (self.m * MINUTE_DIVIDER)) / SECOND_DIVIDER;
        self.ms =
            ms - (self.h * HOUR_DIVIDER) - (self.m * MINUTE_DIVIDER) - (self.s * SECOND_DIVIDER);
    }

    pub fn to_ass_string(&self) -> String {
        format!(
            "{:0>1}:{:0>2}:{:0>2}.{:0>2}",
            self.h,
            self.m,
            self.s,
            self.ms / 10
        )
    }
    pub fn to_srt_string(&self) -> String {
        format!(
            "{:0>2}:{:0>2}:{:0>2},{:0>3}",
            self.h, self.m, self.s, self.ms
        )
    }
    pub fn to_vtt_string(&self) -> String {
        format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
            self.h, self.m, self.s, self.ms
        )
    }
}

macro_rules! impl_ops {
    ($($type:ty)*) => {
        $(
            impl Add<$type> for Time {
                type Output = Time;
                fn add(mut self, other: $type) -> Self::Output {
                    self.set_ms(self.total_ms() + other as u32);
                    self
                }
            }
            impl Add<$type> for &mut Time {
                type Output = Self;
                fn add(self, other: $type) -> Self::Output {
                    self.set_ms(self.total_ms() + other as u32);
                    self
                }
            }
            impl AddAssign<$type> for Time {
                fn add_assign(&mut self, other: $type) {
                    self.set_ms(self.total_ms() + other as u32)
                }
            }
            impl Sub<$type> for Time {
                type Output = Time;
                fn sub(mut self, other: $type) -> Self::Output {
                    self.set_ms(self.total_ms() - other as u32);
                    self
                }
            }
            impl Sub<$type> for &mut Time {
                type Output = Self;
                fn sub(self, other: $type) -> Self::Output {
                    self.set_ms(self.total_ms() - other as u32);
                    self
                }
            }
            impl SubAssign<$type> for Time {
                fn sub_assign(&mut self, other: $type) {
                    self.set_ms(self.total_ms() - other as u32)
                }
            }
        )*
    };
}

impl_ops! {
    i8 i16 i32 i64 i128 u8 u16 u32 u64 u128
}

// Displays the time as `hh:mm:ss.mss`
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_vtt_string())
    }
}
