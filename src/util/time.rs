//! Helper module to parse, print, manipulate timestamps.
//!
//! SSA has a timestamp representation of `00:00:00.00`
//!
//! VTT has a timestamp representation of `00:00:00.000`
//!
//! SRT has a timestamp representation of `00:00:00,000`

use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::result::Result;
use std::str::FromStr;
use std::{fmt, ops::Div};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub h: u32,
    pub m: u32,
    pub s: u32,
    pub ms: u32,
    pub frames: u32,
    pub fps: f32,
}
impl Eq for Time {}
impl Default for Time {
    fn default() -> Self {
        let mut t = Time {
            h: 0,
            m: 0,
            s: 0,
            ms: 0,
            frames: 0,
            fps: 0.0,
        };
        t.derive_frames();
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
pub fn frames_to_ms(frames: u32, fps: f32) -> u32 {
    if frames == 0 || fps == 0.0 {
        0
    } else {
        (((frames as f32) * (10000.0)) / fps).round() as u32 / 10
    }
}
pub fn ms_to_frames(ms: u32, fps: f32) -> u32 {
    if fps == 0.0 {
        0
    } else {
        ((ms as f32) * fps / 1000.0).round() as u32
    }
}
pub fn ms_to_timestring(ms: u32) -> String {
    let hms = ms.div(3600000);
    let mms = (ms - hms * 3600000).div(60000);
    let sms = (ms - mms * 60000 - hms * 3600000).div(1000);
    let msms = &format!(
        "0.{:0>3}",
        ((ms - mms * 60000 - hms * 3600000 - sms * 1000) as f32)
    );
    let mmmms = msms
        .split('.')
        .collect::<Vec<&str>>()
        .get(1)
        .or(Some(&"0"))
        .expect("Should be good")
        .to_string();
    format!("{hms:0>2}")
        + ":"
        + &format!("{mms:0>2}")
        + ":"
        + &format!("{sms:0>2}")
        + "."
        + &format!("{mmmms:0>3}")
}

impl std::error::Error for Time {}
impl Time {
    pub fn total_ms(&self) -> u32 {
        self.h * 3600000 + self.m * 60000 + self.s * 1000 + self.ms
    }
    pub fn update_from_fps_frames(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let t = Time::from_str(&ms_to_timestring(frames_to_ms(self.frames, self.fps)))?;
        self.h = t.h;
        self.m = t.m;
        self.s = t.s;
        self.ms = t.ms;
        Ok(())
    }
    pub fn derive_frames(&mut self) {
        self.frames = ms_to_frames(self.total_ms(), self.fps);
    }
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
        self.derive_frames();
    }
    pub fn update_from_ms(&mut self, ms: u32) -> Result<(), Box<dyn std::error::Error>> {
        let t = Time::from_str(&ms_to_timestring(ms))?;
        self.h = t.h;
        self.m = t.m;
        self.s = t.s;
        self.ms = t.ms;
        self.derive_frames();
        Ok(())
    }

    // Adds <u32>ms to `self` and updates.
    pub fn add_ms(&mut self, ms: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.update_from_ms(self.total_ms() + ms)?;
        Ok(())
    }
    // Subtracts <u32>ms from `self` and updates. Panics if total ms < 0
    pub fn sub_ms(&mut self, ms: u32) -> Result<(), &mut Time> {
        if ms > self.total_ms() {
            self.update_from_ms(self.total_ms() - self.total_ms())
                .unwrap();
            Err(self)
        } else {
            self.update_from_ms(self.total_ms() - ms).unwrap();
            Ok(())
        }
    }
    pub fn to_ass_string(self) -> String {
        format!(
            "{:0>1}:{:0>2}:{:0>2}.{:0>2}",
            self.h,
            self.m,
            self.s,
            self.ms / 10
        )
    }
    pub fn to_srt_string(self) -> String {
        format!(
            "{:0>2}:{:0>2}:{:0>2},{:0>3}",
            self.h, self.m, self.s, self.ms
        )
    }
}
// Add <u32>ms to a `Time` struct
impl Add<u32> for Time {
    type Output = Time;
    fn add(mut self, other: u32) -> Time {
        self.add_ms(other).unwrap();
        self
    }
}
impl AddAssign<u32> for Time {
    fn add_assign(&mut self, other: u32) {
        self.add_ms(other).unwrap();
    }
}
impl AddAssign<i32> for Time {
    fn add_assign(&mut self, other: i32) {
        self.add_ms(other.try_into().unwrap()).unwrap();
    }
}
// Subtracts <u32>ms to a `Time` struct
impl Sub<u32> for Time {
    type Output = Self;
    fn sub(mut self, other: u32) -> Self {
        self.sub_ms(other).unwrap_or_default();
        self
    }
} // Subtracts <i32>ms to a `Time` struct
impl Sub<i32> for Time {
    type Output = Self;
    fn sub(mut self, other: i32) -> Self {
        self.sub_ms(other.try_into().unwrap()).unwrap_or_default();
        self
    }
}
impl Sub<i32> for &mut Time {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        self.sub_ms(other.try_into().unwrap()).unwrap_or_default();
        self
    }
}
impl Sub<u32> for &mut Time {
    type Output = Self;
    fn sub(self, other: u32) -> Self {
        self.sub_ms(other).unwrap_or_default();
        self
    }
}
// Add <i32>ms to a `Time` struct
impl Add<i32> for Time {
    type Output = Self;
    fn add(mut self, other: i32) -> Self {
        self.add_ms(other.try_into().unwrap()).unwrap();
        self
    }
}
// Add <i32>ms to a `Time` struct
impl Add<i32> for &mut Time {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        self.add_ms(other as u32).unwrap();
        self
    }
}
// Displays the time as `hh:mm:ss.mss`
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
            self.h, self.m, self.s, self.ms
        )
    }
}
