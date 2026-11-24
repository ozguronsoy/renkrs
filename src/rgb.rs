use crate::{Cmyk, ColorChannel, Hsl, Hsv, ParseColorError, Rgba};

/// Stores color information as red, green, and blue.
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb<T: ColorChannel> {
    /// Red.
    pub r: T,
    /// Green.
    pub g: T,
    /// Blue.
    pub b: T,
}

impl<T: ColorChannel> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl Default for Rgb<u8> {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Default for Rgb<f32> {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl From<Rgb<u8>> for Rgb<f32> {
    fn from(rgb: Rgb<u8>) -> Self {
        Self {
            r: (rgb.r as f32) / 255.0,
            g: (rgb.g as f32) / 255.0,
            b: (rgb.b as f32) / 255.0,
        }
    }
}

impl From<Rgb<f32>> for Rgb<u8> {
    fn from(rgb: Rgb<f32>) -> Self {
        Self {
            r: (rgb.r * 255.0).round() as u8,
            g: (rgb.g * 255.0).round() as u8,
            b: (rgb.b * 255.0).round() as u8,
        }
    }
}

impl From<Rgba<u8>> for Rgb<u8> {
    fn from(rgba: Rgba<u8>) -> Self {
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

impl From<Rgba<u8>> for Rgb<f32> {
    fn from(rgba: Rgba<u8>) -> Self {
        Self {
            r: (rgba.r as f32) / 255.0,
            g: (rgba.g as f32) / 255.0,
            b: (rgba.b as f32) / 255.0,
        }
    }
}

impl From<Rgba<f32>> for Rgb<u8> {
    fn from(rgba: Rgba<f32>) -> Self {
        Self {
            r: (rgba.r * 255.0).round() as u8,
            g: (rgba.g * 255.0).round() as u8,
            b: (rgba.b * 255.0).round() as u8,
        }
    }
}

impl From<Rgba<f32>> for Rgb<f32> {
    fn from(rgba: Rgba<f32>) -> Self {
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

impl From<Hsl> for Rgb<u8> {
    fn from(hsl: Hsl) -> Self {
        Rgb::<f32>::from(hsl).into()
    }
}

impl From<Hsl> for Rgb<f32> {
    fn from(hsl: Hsl) -> Self {
        let c = (1.0 - (2.0 * hsl.l - 1.0).abs()) * hsl.s;
        let x = c * (1.0 - ((hsl.h / 60.0) % 2.0 - 1.0).abs());
        let m = hsl.l - c / 2.0;

        let (r_prime, g_prime, b_prime) = if hsl.h >= 0.0 && hsl.h < 60.0 {
            (c, x, 0.0)
        } else if hsl.h >= 60.0 && hsl.h < 120.0 {
            (x, c, 0.0)
        } else if hsl.h >= 120.0 && hsl.h < 180.0 {
            (0.0, c, x)
        } else if hsl.h >= 180.0 && hsl.h < 240.0 {
            (0.0, x, c)
        } else if hsl.h >= 240.0 && hsl.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: r_prime + m,
            g: g_prime + m,
            b: b_prime + m,
        }
    }
}

impl From<Hsv> for Rgb<u8> {
    fn from(hsv: Hsv) -> Self {
        Rgb::<f32>::from(hsv).into()
    }
}

impl From<Hsv> for Rgb<f32> {
    fn from(hsv: Hsv) -> Self {
        let c = hsv.v * hsv.s;
        let x = c * (1.0 - ((hsv.h / 60.0) % 2.0 - 1.0).abs());
        let m = hsv.v - c;

        let (r_prime, g_prime, b_prime) = if hsv.h >= 0.0 && hsv.h < 60.0 {
            (c, x, 0.0)
        } else if hsv.h >= 60.0 && hsv.h < 120.0 {
            (x, c, 0.0)
        } else if hsv.h >= 120.0 && hsv.h < 180.0 {
            (0.0, c, x)
        } else if hsv.h >= 180.0 && hsv.h < 240.0 {
            (0.0, x, c)
        } else if hsv.h >= 240.0 && hsv.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: r_prime + m,
            g: g_prime + m,
            b: b_prime + m,
        }
    }
}

impl From<Cmyk> for Rgb<u8> {
    fn from(cmyk: Cmyk) -> Self {
        Rgb::<f32>::from(cmyk).into()
    }
}

impl From<Cmyk> for Rgb<f32> {
    fn from(cmyk: Cmyk) -> Self {
        Self {
            r: (1.0 - cmyk.c) * (1.0 - cmyk.k),
            g: (1.0 - cmyk.m) * (1.0 - cmyk.k),
            b: (1.0 - cmyk.y) * (1.0 - cmyk.k),
        }
    }
}

impl std::str::FromStr for Rgb<u8> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() == 6 {
            let r =
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            let g =
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            let b =
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseColorError::InvalidFormat)?;
            Ok(Self { r, g, b })
        } else if hex.len() == 8 {
            let rgba: Rgba<u8> = hex.parse()?;
            Ok(rgba.into())
        } else {
            Err(ParseColorError::InvalidLength)
        }
    }
}

impl std::str::FromStr for Rgb<f32> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb: Rgb<u8> = s.parse()?;
        Ok(rgb.into())
    }
}

impl std::fmt::LowerHex for Rgb<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }
}

impl std::fmt::LowerHex for Rgb<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: Rgb<u8> = (*self).into();
        std::fmt::LowerHex::fmt(&rgb, f)
    }
}

impl std::fmt::UpperHex for Rgb<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If the user uses the alternate flag `{:#X}`, prefix with '#'
        if f.alternate() {
            write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }
}

impl std::fmt::UpperHex for Rgb<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: Rgb<u8> = (*self).into();
        std::fmt::UpperHex::fmt(&rgb, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_defaults() {
        let cu8 = Rgb::<u8>::default();
        let cf32 = Rgb::<f32>::default();
        assert_eq!(cu8.r, 0);
        assert_eq!(cu8.g, 0);
        assert_eq!(cu8.b, 0);
        assert_eq!(cf32.r, 0.0);
        assert_eq!(cf32.g, 0.0);
        assert_eq!(cf32.b, 0.0);
    }

    #[test]
    fn test_rgb_generic_conversion() {
        let cu8_1 = Rgb::<u8>::new(18, 86, 205);
        let cf32_1: Rgb<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 18.0 / 255.0);
        assert_eq!(cf32_1.g, 86.0 / 255.0);
        assert_eq!(cf32_1.b, 205.0 / 255.0);

        let cf32_2 = Rgb::<f32>::new(0.10, 0.25, 0.40);
        let cu8_2: Rgb<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.10_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.25_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.40_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgb_to_rgba_conversion() {
        let cu8_1 = Rgb::<u8>::new(37, 56, 152);
        let cu8_2: Rgba<u8> = cu8_1.into();
        assert_eq!(cu8_1.r, cu8_2.r);
        assert_eq!(cu8_1.g, cu8_2.g);
        assert_eq!(cu8_1.b, cu8_2.b);
        assert_eq!(cu8_2.a, 255);

        let cf32_1 = Rgb::<f32>::new(0.65, 0.11, 0.28);
        let cf32_2: Rgba<f32> = cf32_1.into();
        assert_eq!(cf32_1.r, cf32_2.r);
        assert_eq!(cf32_1.g, cf32_2.g);
        assert_eq!(cf32_1.b, cf32_2.b);
        assert_eq!(cf32_2.a, 1.0);
    }

    #[test]
    fn test_rgb_to_rgba_generic_conversion() {
        // Rgb<u8> -> Rgba<f32>
        let cu8_1 = Rgb::<u8>::new(100, 150, 200);
        let cf32_1: Rgba<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 100.0 / 255.0);
        assert_eq!(cf32_1.g, 150.0 / 255.0);
        assert_eq!(cf32_1.b, 200.0 / 255.0);
        assert_eq!(cf32_1.a, 1.0);

        let cf32_2 = Rgb::<f32>::new(0.20, 0.50, 0.80);
        let cu8_2: Rgba<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.20_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.50_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.80_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.a, 255);
    }

    #[test]
    fn test_rgb_to_hsl_conversion() {
        let epsilon = 0.001;

        let cf32 = Rgb::<f32>::new(0.2, 0.6, 1.0);
        let hsl_f32: Hsl = cf32.into();
        assert!((hsl_f32.h - 210.0).abs() < epsilon);
        assert!((hsl_f32.s - 1.0).abs() < epsilon);
        assert!((hsl_f32.l - 0.6).abs() < epsilon);

        let cu8 = Rgb::<u8>::new(51, 153, 255);
        let hsl_u8: Hsl = cu8.into();
        assert!((hsl_u8.h - 210.0).abs() < epsilon);
        assert!(
            (hsl_u8.s - 1.0).abs() < epsilon,
            "Rgb<u8> Saturation failed"
        );
        assert!((hsl_u8.l - 0.6).abs() < epsilon);
    }

    #[test]
    fn test_rgb_to_hsv_conversion() {
        let epsilon = 0.001;

        let cf32 = Rgb::<f32>::new(0.2, 0.6, 1.0);
        let hsv_f32: Hsv = cf32.into();
        assert!((hsv_f32.h - 210.0).abs() < epsilon);
        assert!((hsv_f32.s - 0.8).abs() < epsilon);
        assert!((hsv_f32.v - 1.0).abs() < epsilon);

        let cu8 = Rgb::<u8>::new(51, 153, 255);
        let hsv_u8: Hsv = cu8.into();
        assert!((hsv_u8.h - 210.0).abs() < epsilon);
        assert!((hsv_u8.s - 0.8).abs() < epsilon);
        assert!((hsv_u8.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_rgb_to_cmyk_conversion() {
        let epsilon = 0.001;

        let rgb_red = Rgb::<f32>::new(1.0, 0.0, 0.0);
        let cmyk_red: Cmyk = rgb_red.into();
        assert!((cmyk_red.c - 0.0).abs() < epsilon);
        assert!((cmyk_red.m - 1.0).abs() < epsilon);
        assert!((cmyk_red.y - 1.0).abs() < epsilon);
        assert!((cmyk_red.k - 0.0).abs() < epsilon);

        let rgb_black = Rgb::<f32>::default();
        let cmyk_black: Cmyk = rgb_black.into();
        assert!((cmyk_black.c - 0.0).abs() < epsilon);
        assert!((cmyk_black.m - 0.0).abs() < epsilon);
        assert!((cmyk_black.y - 0.0).abs() < epsilon);
        assert!((cmyk_black.k - 1.0).abs() < epsilon);

        let rgb_purple = Rgb::<u8>::new(102, 51, 153);
        let cmyk_purple: Cmyk = rgb_purple.into();
        assert!((cmyk_purple.c - 0.333).abs() < epsilon);
        assert!((cmyk_purple.m - 0.666).abs() < epsilon);
        assert!((cmyk_purple.y - 0.0).abs() < epsilon);
        assert!((cmyk_purple.k - 0.400).abs() < epsilon);
    }

    #[test]
    fn test_rgb_parse_hex() {
        let cu8_1: Rgb<u8> = "#FF5733".parse().unwrap();
        assert_eq!(cu8_1.r, 255);
        assert_eq!(cu8_1.g, 87);
        assert_eq!(cu8_1.b, 51);

        let cu8_2: Rgb<u8> = "00FF00AA".parse().unwrap();
        assert_eq!(cu8_2.r, 0);
        assert_eq!(cu8_2.g, 255);
        assert_eq!(cu8_2.b, 0);

        let cf32_1: Rgb<f32> = "#FFFFFF".parse().unwrap();
        assert_eq!(cf32_1.r, 1.0);
        assert_eq!(cf32_1.g, 1.0);
        assert_eq!(cf32_1.b, 1.0);

        assert_eq!(
            "#FF573".parse::<Rgb<u8>>(),
            Err(ParseColorError::InvalidLength)
        );
        assert_eq!(
            "#XX5733".parse::<Rgb<u8>>(),
            Err(ParseColorError::InvalidFormat)
        );
    }

    #[test]
    fn test_rgb_to_hex() {
        let cu8 = Rgb::<u8>::new(255, 87, 51);
        assert_eq!(format!("{:X}", cu8), "FF5733");
        assert_eq!(format!("{:x}", cu8), "ff5733");
        assert_eq!(format!("{:#X}", cu8), "#FF5733");
        assert_eq!(format!("{:#x}", cu8), "#ff5733");

        let cf32 = Rgb::<f32>::new(1.0, 0.0, 1.0);
        assert_eq!(format!("{:X}", cf32), "FF00FF");
        assert_eq!(format!("{:#x}", cf32), "#ff00ff");
    }
}
