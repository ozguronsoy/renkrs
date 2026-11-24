//! # renkrs
//!
//! A zero-cost, high-performance color library.
//!
//! `renkrs` provides conversions between RGB, RGBA, HSL, HSV, and CMYK color spaces.
//!
//! ## Example
//! ```rust
//! use renkrs::{RGB, RGBA, HSL, HSV, CMYK};
//!
//! let color_u8: RGBA<u8> = "#FF5733".parse().unwrap();
//! let color_f32: RGB<f32> = color_u8.into();
//! let hsl: HSL = color_u8.into();
//! let hsv: HSV = color_f32.into();
//! let cmyk: CMYK = hsv.into();
//! let hex_lower: String = format!("{:#x}", color_u8);
//! let hex_upper: String = format!("{:#X}", color_f32);
//! ```

mod cmyk;
mod color_channel;
mod error;
mod hsl;
mod hsv;
mod rgb;
mod rgba;

pub use {cmyk::*, color_channel::*, error::*, hsl::*, hsv::*, rgb::*, rgba::*};
