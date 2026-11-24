//! # renkrs
//!
//! A zero-cost, high-performance color library.
//!
//! `renkrs` provides conversions between RGB, RGBA, HSL, HSV, and CMYK color spaces.
//!
//! ## Example
//! ```rust
//! use renkrs::{Rgb, Rgba, Hsl, Hsv, Cmyk};
//!
//! let color_u8: Rgba<u8> = "#FF5733".parse().unwrap();
//! let color_f32: Rgb<f32> = color_u8.into();
//! let hsl: Hsl = color_u8.into();
//! let hsv: Hsv = color_f32.into();
//! let cmyk: Cmyk = hsv.into();
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
