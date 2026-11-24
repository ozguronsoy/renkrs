use crate::{Cmyk, ColorChannel, Hsl, Hsv, ParseColorError, Rgb};

/// Stores color information as red, green, and blue with an alpha channel.
///
/// # Type Parameters
///
/// * `T` - The numeric type of the color channels. This is restricted by the [`ColorChannel`] trait.
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba<T: ColorChannel> {
    /// Red.
    pub r: T,
    /// Green.
    pub g: T,
    /// Blue.
    pub b: T,
    /// Alpha.
    pub a: T,
}

impl<T: ColorChannel> Rgba<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Rgba<u8> {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Default for Rgba<f32> {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl From<Rgba<u8>> for Rgba<f32> {
    fn from(rgba: Rgba<u8>) -> Self {
        Self {
            r: (rgba.r as f32) / 255.0,
            g: (rgba.g as f32) / 255.0,
            b: (rgba.b as f32) / 255.0,
            a: (rgba.a as f32) / 255.0,
        }
    }
}

impl From<Rgba<f32>> for Rgba<u8> {
    fn from(rgba: Rgba<f32>) -> Self {
        Self {
            r: (rgba.r * 255.0).round() as u8,
            g: (rgba.g * 255.0).round() as u8,
            b: (rgba.b * 255.0).round() as u8,
            a: (rgba.a * 255.0).round() as u8,
        }
    }
}

impl From<Rgb<u8>> for Rgba<u8> {
    fn from(rgb: Rgb<u8>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 255,
        }
    }
}

impl From<Rgb<u8>> for Rgba<f32> {
    fn from(rgb: Rgb<u8>) -> Self {
        Self {
            r: (rgb.r as f32) / 255.0,
            g: (rgb.g as f32) / 255.0,
            b: (rgb.b as f32) / 255.0,
            a: 1.0,
        }
    }
}

impl From<Rgb<f32>> for Rgba<f32> {
    fn from(rgb: Rgb<f32>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 1.0,
        }
    }
}

impl From<Rgb<f32>> for Rgba<u8> {
    fn from(rgb: Rgb<f32>) -> Self {
        Self {
            r: (rgb.r * 255.0).round() as u8,
            g: (rgb.g * 255.0).round() as u8,
            b: (rgb.b * 255.0).round() as u8,
            a: 255,
        }
    }
}

impl From<Hsl> for Rgba<u8> {
    fn from(hsl: Hsl) -> Self {
        Rgb::<u8>::from(hsl).into()
    }
}

impl From<Hsl> for Rgba<f32> {
    fn from(hsl: Hsl) -> Self {
        Rgb::<f32>::from(hsl).into()
    }
}

impl From<Hsv> for Rgba<u8> {
    fn from(hsv: Hsv) -> Self {
        Rgb::<f32>::from(hsv).into()
    }
}

impl From<Hsv> for Rgba<f32> {
    fn from(hsv: Hsv) -> Self {
        Rgb::<f32>::from(hsv).into()
    }
}

impl From<Cmyk> for Rgba<u8> {
    fn from(cmyk: Cmyk) -> Self {
        Rgb::<f32>::from(cmyk).into()
    }
}

impl From<Cmyk> for Rgba<f32> {
    fn from(cmyk: Cmyk) -> Self {
        Rgb::<f32>::from(cmyk).into()
    }
}

impl std::str::FromStr for Rgba<u8> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() == 6 {
            let rgb: Rgb<u8> = hex.parse()?;
            Ok(rgb.into())
        } else if hex.len() == 8 {
            let r =
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            let g =
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            let b =
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            let a =
                u8::from_str_radix(&hex[6..8], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            Ok(Self { r, g, b, a })
        } else {
            Err(ParseColorError::InvalidLength)
        }
    }
}

impl std::str::FromStr for Rgba<f32> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgba: Rgba<u8> = s.parse()?;
        Ok(rgba.into())
    }
}

impl std::fmt::LowerHex for Rgba<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "#{:02x}{:02x}{:02x}{:02x}",
                self.r, self.g, self.b, self.a
            )
        } else {
            write!(
                f,
                "{:02x}{:02x}{:02x}{:02x}",
                self.r, self.g, self.b, self.a
            )
        }
    }
}

impl std::fmt::LowerHex for Rgba<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: Rgba<u8> = (*self).into();
        std::fmt::LowerHex::fmt(&rgba, f)
    }
}

impl std::fmt::UpperHex for Rgba<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If the user uses the alternate flag `{:#X}`, prefix with '#'
        if f.alternate() {
            write!(
                f,
                "#{:02X}{:02X}{:02X}{:02X}",
                self.r, self.g, self.b, self.a
            )
        } else {
            write!(
                f,
                "{:02X}{:02X}{:02X}{:02X}",
                self.r, self.g, self.b, self.a
            )
        }
    }
}

impl std::fmt::UpperHex for Rgba<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: Rgba<u8> = (*self).into();
        std::fmt::UpperHex::fmt(&rgba, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_defaults() {
        let cu8 = Rgba::<u8>::default();
        let cf32 = Rgba::<f32>::default();
        assert_eq!(cu8.r, 0);
        assert_eq!(cu8.g, 0);
        assert_eq!(cu8.b, 0);
        assert_eq!(cu8.a, 255);
        assert_eq!(cf32.r, 0.0);
        assert_eq!(cf32.g, 0.0);
        assert_eq!(cf32.a, 1.0);
    }

    #[test]
    fn test_rgba_generic_conversion() {
        let cu8_1 = Rgba::<u8>::new(18, 86, 205, 163);
        let cf32_1: Rgba<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 18.0 / 255.0);
        assert_eq!(cf32_1.g, 86.0 / 255.0);
        assert_eq!(cf32_1.b, 205.0 / 255.0);
        assert_eq!(cf32_1.a, 163.0 / 255.0);

        let cf32_2 = Rgba::<f32>::new(0.10, 0.25, 0.40, 0.86);
        let cu8_2: Rgba<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.10_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.25_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.40_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.a, (0.86_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgba_to_rgb_conversion() {
        let cu8_1 = Rgba::<u8>::new(37, 56, 152, 213);
        let cu8_2: Rgb<u8> = cu8_1.into();
        assert_eq!(cu8_1.r, cu8_2.r);
        assert_eq!(cu8_1.g, cu8_2.g);
        assert_eq!(cu8_1.b, cu8_2.b);

        let cf32_1 = Rgba::<f32>::new(0.65, 0.11, 0.28, 0.35);
        let cf32_2: Rgb<f32> = cf32_1.into();
        assert_eq!(cf32_1.r, cf32_2.r);
        assert_eq!(cf32_1.g, cf32_2.g);
        assert_eq!(cf32_1.b, cf32_2.b);
    }

    #[test]
    fn test_rgba_to_rgb_generic_conversion() {
        let cu8_1 = Rgba::<u8>::new(50, 100, 150, 200);
        let cf32_1: Rgb<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 50.0 / 255.0);
        assert_eq!(cf32_1.g, 100.0 / 255.0);
        assert_eq!(cf32_1.b, 150.0 / 255.0);

        let cf32_2 = Rgba::<f32>::new(0.30, 0.60, 0.90, 0.50);
        let cu8_2: Rgb<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.30_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.60_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.90_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgba_to_hsl_conversion() {
        let epsilon = 0.001;

        let cu8 = Rgba::<u8>::new(51, 153, 255, 128);
        let hsl_u8: Hsl = cu8.into();
        assert!((hsl_u8.h - 210.0).abs() < epsilon);
        assert!((hsl_u8.s - 1.0).abs() < epsilon);
        assert!((hsl_u8.l - 0.6).abs() < epsilon);

        let cf32 = Rgba::<f32>::new(1.0, 0.0, 0.0, 0.5);
        let hsl_f32: Hsl = cf32.into();
        assert!((hsl_f32.h - 0.0).abs() < epsilon);
        assert!((hsl_f32.s - 1.0).abs() < epsilon);
        assert!((hsl_f32.l - 0.5).abs() < epsilon);
    }

    #[test]
    fn test_rgba_to_hsv_conversion() {
        let epsilon = 0.001;

        let cf32 = Rgba::<f32>::new(0.2, 0.6, 1.0, 0.35);
        let hsv_f32: Hsv = cf32.into();
        assert!((hsv_f32.h - 210.0).abs() < epsilon);
        assert!((hsv_f32.s - 0.8).abs() < epsilon);
        assert!((hsv_f32.v - 1.0).abs() < epsilon);

        let cu8 = Rgba::<u8>::new(51, 153, 255, 172);
        let hsv_u8: Hsv = cu8.into();
        assert!((hsv_u8.h - 210.0).abs() < epsilon);
        assert!((hsv_u8.s - 0.8).abs() < epsilon);
        assert!((hsv_u8.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_rgba_to_cmyk_conversion() {
        let epsilon = 0.001;

        let rgba_red = Rgba::<f32>::new(1.0, 0.0, 0.0, 0.3);
        let cmyk_red: Cmyk = rgba_red.into();
        assert!((cmyk_red.c - 0.0).abs() < epsilon);
        assert!((cmyk_red.m - 1.0).abs() < epsilon);
        assert!((cmyk_red.y - 1.0).abs() < epsilon);
        assert!((cmyk_red.k - 0.0).abs() < epsilon);

        let rgba_black = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.7);
        let cmyk_black: Cmyk = rgba_black.into();
        assert!((cmyk_black.c - 0.0).abs() < epsilon);
        assert!((cmyk_black.m - 0.0).abs() < epsilon);
        assert!((cmyk_black.y - 0.0).abs() < epsilon);
        assert!((cmyk_black.k - 1.0).abs() < epsilon);

        let rgba_purple = Rgba::<u8>::new(102, 51, 153, 217);
        let cmyk_purple: Cmyk = rgba_purple.into();
        assert!((cmyk_purple.c - 0.333).abs() < epsilon);
        assert!((cmyk_purple.m - 0.666).abs() < epsilon);
        assert!((cmyk_purple.y - 0.0).abs() < epsilon);
        assert!((cmyk_purple.k - 0.400).abs() < epsilon);
    }

    #[test]
    fn test_rgba_parse_hex() {
        let cu8_1: Rgba<u8> = "#FF5733CC".parse().unwrap();
        assert_eq!(cu8_1.r, 255);
        assert_eq!(cu8_1.g, 87);
        assert_eq!(cu8_1.b, 51);
        assert_eq!(cu8_1.a, 204);

        let cu8_2: Rgba<u8> = "00FF00".parse().unwrap();
        assert_eq!(cu8_2.r, 0);
        assert_eq!(cu8_2.g, 255);
        assert_eq!(cu8_2.b, 0);
        assert_eq!(cu8_2.a, 255);

        let cf32_1: Rgba<f32> = "#000000FF".parse().unwrap();
        assert_eq!(cf32_1.r, 0.0);
        assert_eq!(cf32_1.g, 0.0);
        assert_eq!(cf32_1.b, 0.0);
        assert_eq!(cf32_1.a, 1.0);

        assert_eq!(
            "FF5733C".parse::<Rgba<u8>>(),
            Err(ParseColorError::InvalidLength)
        );
        assert_eq!(
            "#FF5733XX".parse::<Rgba<f32>>(),
            Err(ParseColorError::InvalidFormat)
        );
    }

    #[test]
    fn test_rgba_to_hex() {
        let cu8 = Rgba::<u8>::new(0, 255, 0, 170);
        assert_eq!(format!("{:X}", cu8), "00FF00AA");
        assert_eq!(format!("{:x}", cu8), "00ff00aa");
        assert_eq!(format!("{:#X}", cu8), "#00FF00AA");
        assert_eq!(format!("{:#x}", cu8), "#00ff00aa");

        let cf32 = Rgba::<f32>::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(format!("{:X}", cf32), "000000FF");
        assert_eq!(format!("{:#x}", cf32), "#000000ff");
    }
}
