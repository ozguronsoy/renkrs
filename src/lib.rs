// src/lib.rs

pub trait ColorChannel {}

impl ColorChannel for u8 {}
impl ColorChannel for f32 {}

#[derive(Debug, PartialEq)]
pub enum ParseColorError {
    /// The string was not exactly 6 characters (ignoring the optional '#')
    InvalidLength,
    /// The string contained characters that are not valid hex (0-9, A-F)
    InvalidFormat,
}

impl std::error::Error for ParseColorError {}
impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseColorError::InvalidLength => {
                write!(f, "Hex color string must be 6 or 8 characters long")
            }
            ParseColorError::InvalidFormat => {
                write!(f, "Hex color string contains invalid characters")
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB<T: ColorChannel> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA<T: ColorChannel> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Default for RGB<u8> {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Default for RGB<f32> {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl From<RGB<u8>> for RGB<f32> {
    fn from(rgb: RGB<u8>) -> Self {
        Self {
            r: (rgb.r as f32) / 255.0,
            g: (rgb.g as f32) / 255.0,
            b: (rgb.b as f32) / 255.0,
        }
    }
}

impl From<RGB<f32>> for RGB<u8> {
    fn from(rgb: RGB<f32>) -> Self {
        Self {
            r: (rgb.r * 255.0).round() as u8,
            g: (rgb.g * 255.0).round() as u8,
            b: (rgb.b * 255.0).round() as u8,
        }
    }
}

impl From<RGBA<u8>> for RGB<u8> {
    fn from(rgba: RGBA<u8>) -> Self {
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

impl From<RGBA<u8>> for RGB<f32> {
    fn from(rgba: RGBA<u8>) -> Self {
        Self {
            r: (rgba.r as f32) / 255.0,
            g: (rgba.g as f32) / 255.0,
            b: (rgba.b as f32) / 255.0,
        }
    }
}

impl From<RGBA<f32>> for RGB<f32> {
    fn from(rgba: RGBA<f32>) -> Self {
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

impl From<RGBA<f32>> for RGB<u8> {
    fn from(rgba: RGBA<f32>) -> Self {
        Self {
            r: (rgba.r * 255.0).round() as u8,
            g: (rgba.g * 255.0).round() as u8,
            b: (rgba.b * 255.0).round() as u8,
        }
    }
}

impl From<HSL> for RGB<u8> {
    fn from(hsl: HSL) -> Self {
        RGB::<f32>::from(hsl).into()
    }
}

impl From<HSL> for RGB<f32> {
    fn from(hsl: HSL) -> Self {
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

impl From<HSV> for RGB<u8> {
    fn from(hsv: HSV) -> Self {
        RGB::<f32>::from(hsv).into()
    }
}

impl From<HSV> for RGB<f32> {
    fn from(hsv: HSV) -> Self {
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

impl std::str::FromStr for RGB<u8> {
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
            let rgba: RGBA<u8> = hex.parse()?;
            Ok(rgba.into())
        } else {
            return Err(ParseColorError::InvalidLength);
        }
    }
}

impl std::str::FromStr for RGB<f32> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb: RGB<u8> = s.parse()?;
        Ok(rgb.into())
    }
}

impl std::fmt::LowerHex for RGB<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }
}

impl std::fmt::LowerHex for RGB<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: RGB<u8> = (*self).into();
        std::fmt::LowerHex::fmt(&rgb, f)
    }
}

impl std::fmt::UpperHex for RGB<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If the user uses the alternate flag `{:#X}`, prefix with '#'
        if f.alternate() {
            write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }
}

impl std::fmt::UpperHex for RGB<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: RGB<u8> = (*self).into();
        std::fmt::UpperHex::fmt(&rgb, f)
    }
}

impl Default for RGBA<u8> {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Default for RGBA<f32> {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl From<RGBA<u8>> for RGBA<f32> {
    fn from(rgba: RGBA<u8>) -> Self {
        Self {
            r: (rgba.r as f32) / 255.0,
            g: (rgba.g as f32) / 255.0,
            b: (rgba.b as f32) / 255.0,
            a: (rgba.a as f32) / 255.0,
        }
    }
}

impl From<RGBA<f32>> for RGBA<u8> {
    fn from(rgba: RGBA<f32>) -> Self {
        Self {
            r: (rgba.r * 255.0).round() as u8,
            g: (rgba.g * 255.0).round() as u8,
            b: (rgba.b * 255.0).round() as u8,
            a: (rgba.a * 255.0).round() as u8,
        }
    }
}

impl From<RGB<u8>> for RGBA<u8> {
    fn from(rgb: RGB<u8>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 255,
        }
    }
}

impl From<RGB<u8>> for RGBA<f32> {
    fn from(rgb: RGB<u8>) -> Self {
        Self {
            r: (rgb.r as f32) / 255.0,
            g: (rgb.g as f32) / 255.0,
            b: (rgb.b as f32) / 255.0,
            a: 1.0,
        }
    }
}

impl From<RGB<f32>> for RGBA<f32> {
    fn from(rgb: RGB<f32>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 1.0,
        }
    }
}

impl From<RGB<f32>> for RGBA<u8> {
    fn from(rgb: RGB<f32>) -> Self {
        Self {
            r: (rgb.r * 255.0).round() as u8,
            g: (rgb.g * 255.0).round() as u8,
            b: (rgb.b * 255.0).round() as u8,
            a: 255,
        }
    }
}

impl From<HSL> for RGBA<u8> {
    fn from(hsl: HSL) -> Self {
        RGB::<u8>::from(hsl).into()
    }
}

impl From<HSL> for RGBA<f32> {
    fn from(hsl: HSL) -> Self {
        RGB::<f32>::from(hsl).into()
    }
}

impl From<HSV> for RGBA<u8> {
    fn from(hsv: HSV) -> Self {
        RGB::<f32>::from(hsv).into()
    }
}

impl From<HSV> for RGBA<f32> {
    fn from(hsv: HSV) -> Self {
        RGB::<f32>::from(hsv).into()
    }
}

impl std::str::FromStr for RGBA<u8> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() == 6 {
            let rgb: RGB<u8> = hex.parse()?;
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

impl std::str::FromStr for RGBA<f32> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgba: RGBA<u8> = s.parse()?;
        Ok(rgba.into())
    }
}

impl std::fmt::LowerHex for RGBA<u8> {
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

impl std::fmt::LowerHex for RGBA<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA<u8> = (*self).into();
        std::fmt::LowerHex::fmt(&rgba, f)
    }
}

impl std::fmt::UpperHex for RGBA<u8> {
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

impl std::fmt::UpperHex for RGBA<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: RGBA<u8> = (*self).into();
        std::fmt::UpperHex::fmt(&rgba, f)
    }
}

impl Default for HSL {
    fn default() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        }
    }
}

impl From<RGB<u8>> for HSL {
    fn from(rgb: RGB<u8>) -> Self {
        RGB::<f32>::from(rgb).into()
    }
}

impl From<RGB<f32>> for HSL {
    fn from(rgb: RGB<f32>) -> Self {
        let max = rgb.r.max(rgb.g).max(rgb.b);
        let min = rgb.r.min(rgb.g).min(rgb.b);
        let delta = max - min;
        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let mut h = if delta == 0.0 {
            0.0
        } else if max == rgb.r {
            60.0 * (((rgb.g - rgb.b) / delta) % 6.0)
        } else if max == rgb.g {
            60.0 * (((rgb.b - rgb.r) / delta) + 2.0)
        } else {
            60.0 * (((rgb.r - rgb.g) / delta) + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }
        Self { h, s, l }
    }
}

impl From<RGBA<u8>> for HSL {
    fn from(rgba: RGBA<u8>) -> Self {
        RGB::<f32>::from(rgba).into()
    }
}

impl From<RGBA<f32>> for HSL {
    fn from(rgba: RGBA<f32>) -> Self {
        RGB::<f32>::from(rgba).into()
    }
}

impl From<HSV> for HSL {
    fn from(hsv: HSV) -> Self {
        let l = hsv.v * (1.0 - hsv.s / 2.0);
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (hsv.v - l) / l.min(1.0 - l)
        };
        Self { h: hsv.h, s, l }
    }
}

impl Default for HSV {
    fn default() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }
}

impl From<RGB<u8>> for HSV {
    fn from(rgb: RGB<u8>) -> Self {
        RGB::<f32>::from(rgb).into()
    }
}

impl From<RGB<f32>> for HSV {
    fn from(rgb: RGB<f32>) -> Self {
        let max = rgb.r.max(rgb.g).max(rgb.b);
        let min = rgb.r.min(rgb.g).min(rgb.b);
        let delta = max - min;

        let v = max;
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let mut h = if delta == 0.0 {
            0.0
        } else if max == rgb.r {
            60.0 * (((rgb.g - rgb.b) / delta) % 6.0)
        } else if max == rgb.g {
            60.0 * (((rgb.b - rgb.r) / delta) + 2.0)
        } else {
            60.0 * (((rgb.r - rgb.g) / delta) + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }
        Self { h, s, v }
    }
}

impl From<RGBA<u8>> for HSV {
    fn from(rgba: RGBA<u8>) -> Self {
        RGB::<f32>::from(rgba).into()
    }
}

impl From<RGBA<f32>> for HSV {
    fn from(rgba: RGBA<f32>) -> Self {
        RGB::<f32>::from(rgba).into()
    }
}

impl From<HSL> for HSV {
    fn from(hsl: HSL) -> Self {
        let v = hsl.l + hsl.s * hsl.l.min(1.0 - hsl.l);
        let s = if v == 0.0 {
            0.0
        } else {
            2.0 * (1.0 - hsl.l / v)
        };
        Self { h: hsl.h, s, v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_defaults() {
        let cu8 = RGB::<u8>::default();
        let cf32 = RGB::<f32>::default();
        assert_eq!(cu8.r, 0);
        assert_eq!(cu8.g, 0);
        assert_eq!(cu8.b, 0);
        assert_eq!(cf32.r, 0.0);
        assert_eq!(cf32.g, 0.0);
        assert_eq!(cf32.b, 0.0);
    }

    #[test]
    fn test_rgb_generic_conversion() {
        let cu8_1 = RGB::<u8> {
            r: 18,
            g: 86,
            b: 205,
        };
        let cf32_1: RGB<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 18.0 / 255.0);
        assert_eq!(cf32_1.g, 86.0 / 255.0);
        assert_eq!(cf32_1.b, 205.0 / 255.0);

        let cf32_2 = RGB::<f32> {
            r: 0.10,
            g: 0.25,
            b: 0.40,
        };
        let cu8_2: RGB<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.10_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.25_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.40_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgb_to_rgba_conversion() {
        let cu8_1 = RGB::<u8> {
            r: 37,
            g: 56,
            b: 152,
        };
        let cu8_2: RGBA<u8> = cu8_1.into();
        assert_eq!(cu8_1.r, cu8_2.r);
        assert_eq!(cu8_1.g, cu8_2.g);
        assert_eq!(cu8_1.b, cu8_2.b);
        assert_eq!(cu8_2.a, 255);

        let cf32_1 = RGB::<f32> {
            r: 0.65,
            g: 0.11,
            b: 0.28,
        };
        let cf32_2: RGBA<f32> = cf32_1.into();
        assert_eq!(cf32_1.r, cf32_2.r);
        assert_eq!(cf32_1.g, cf32_2.g);
        assert_eq!(cf32_1.b, cf32_2.b);
        assert_eq!(cf32_2.a, 1.0);
    }

    #[test]
    fn test_rgb_to_rgba_generic_conversion() {
        // RGB<u8> -> RGBA<f32>
        let cu8_1 = RGB::<u8> {
            r: 100,
            g: 150,
            b: 200,
        };
        let cf32_1: RGBA<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 100.0 / 255.0);
        assert_eq!(cf32_1.g, 150.0 / 255.0);
        assert_eq!(cf32_1.b, 200.0 / 255.0);
        assert_eq!(cf32_1.a, 1.0);

        let cf32_2 = RGB::<f32> {
            r: 0.20,
            g: 0.50,
            b: 0.80,
        };
        let cu8_2: RGBA<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.20_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.50_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.80_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.a, 255);
    }

    #[test]
    fn test_rgb_to_hsl_conversion() {
        let epsilon = 0.001;

        let cf32 = RGB::<f32> {
            r: 0.2,
            g: 0.6,
            b: 1.0,
        };
        let hsl_f32: HSL = cf32.into();
        assert!((hsl_f32.h - 210.0).abs() < epsilon);
        assert!((hsl_f32.s - 1.0).abs() < epsilon);
        assert!((hsl_f32.l - 0.6).abs() < epsilon);

        let cu8 = RGB::<u8> {
            r: 51,
            g: 153,
            b: 255,
        };
        let hsl_u8: HSL = cu8.into();
        assert!((hsl_u8.h - 210.0).abs() < epsilon);
        assert!(
            (hsl_u8.s - 1.0).abs() < epsilon,
            "RGB<u8> Saturation failed"
        );
        assert!((hsl_u8.l - 0.6).abs() < epsilon);
    }

    #[test]
    fn test_rgb_to_hsv_conversion() {
        let epsilon = 0.001;

        let cf32 = RGB::<f32> {
            r: 0.2,
            g: 0.6,
            b: 1.0,
        };
        let hsv_f32: HSV = cf32.into();
        assert!((hsv_f32.h - 210.0).abs() < epsilon);
        assert!((hsv_f32.s - 0.8).abs() < epsilon);
        assert!((hsv_f32.v - 1.0).abs() < epsilon);

        let cu8 = RGB::<u8> {
            r: 51,
            g: 153,
            b: 255,
        };
        let hsv_u8: HSV = cu8.into();
        assert!((hsv_u8.h - 210.0).abs() < epsilon);
        assert!((hsv_u8.s - 0.8).abs() < epsilon);
        assert!((hsv_u8.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_rgb_parse_hex() {
        let cu8_1: RGB<u8> = "#FF5733".parse().unwrap();
        assert_eq!(cu8_1.r, 255);
        assert_eq!(cu8_1.g, 87);
        assert_eq!(cu8_1.b, 51);

        let cu8_2: RGB<u8> = "00FF00AA".parse().unwrap();
        assert_eq!(cu8_2.r, 0);
        assert_eq!(cu8_2.g, 255);
        assert_eq!(cu8_2.b, 0);

        let cf32_1: RGB<f32> = "#FFFFFF".parse().unwrap();
        assert_eq!(cf32_1.r, 1.0);
        assert_eq!(cf32_1.g, 1.0);
        assert_eq!(cf32_1.b, 1.0);

        assert_eq!(
            "#FF573".parse::<RGB<u8>>(),
            Err(ParseColorError::InvalidLength)
        );
        assert_eq!(
            "#XX5733".parse::<RGB<u8>>(),
            Err(ParseColorError::InvalidFormat)
        );
    }

    #[test]
    fn test_rgb_to_hex() {
        let cu8 = RGB::<u8> {
            r: 255,
            g: 87,
            b: 51,
        };
        assert_eq!(format!("{:X}", cu8), "FF5733");
        assert_eq!(format!("{:x}", cu8), "ff5733");
        assert_eq!(format!("{:#X}", cu8), "#FF5733");
        assert_eq!(format!("{:#x}", cu8), "#ff5733");

        let cf32 = RGB::<f32> {
            r: 1.0,
            g: 0.0,
            b: 1.0,
        };
        assert_eq!(format!("{:X}", cf32), "FF00FF");
        assert_eq!(format!("{:#x}", cf32), "#ff00ff");
    }

    #[test]
    fn test_rgba_defaults() {
        let cu8 = RGBA::<u8>::default();
        let cf32 = RGBA::<f32>::default();
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
        let cu8_1 = RGBA::<u8> {
            r: 18,
            g: 86,
            b: 205,
            a: 163,
        };
        let cf32_1: RGBA<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 18.0 / 255.0);
        assert_eq!(cf32_1.g, 86.0 / 255.0);
        assert_eq!(cf32_1.b, 205.0 / 255.0);
        assert_eq!(cf32_1.a, 163.0 / 255.0);

        let cf32_2 = RGBA::<f32> {
            r: 0.10,
            g: 0.25,
            b: 0.40,
            a: 0.86,
        };
        let cu8_2: RGBA<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.10_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.25_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.40_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.a, (0.86_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgba_to_rgb_conversion() {
        let cu8_1 = RGBA::<u8> {
            r: 37,
            g: 56,
            b: 152,
            a: 213,
        };
        let cu8_2: RGB<u8> = cu8_1.into();
        assert_eq!(cu8_1.r, cu8_2.r);
        assert_eq!(cu8_1.g, cu8_2.g);
        assert_eq!(cu8_1.b, cu8_2.b);

        let cf32_1 = RGBA::<f32> {
            r: 0.65,
            g: 0.11,
            b: 0.28,
            a: 0.35,
        };
        let cf32_2: RGB<f32> = cf32_1.into();
        assert_eq!(cf32_1.r, cf32_2.r);
        assert_eq!(cf32_1.g, cf32_2.g);
        assert_eq!(cf32_1.b, cf32_2.b);
    }

    #[test]
    fn test_rgba_to_rgb_generic_conversion() {
        let cu8_1 = RGBA::<u8> {
            r: 50,
            g: 100,
            b: 150,
            a: 200,
        };
        let cf32_1: RGB<f32> = cu8_1.into();
        assert_eq!(cf32_1.r, 50.0 / 255.0);
        assert_eq!(cf32_1.g, 100.0 / 255.0);
        assert_eq!(cf32_1.b, 150.0 / 255.0);

        let cf32_2 = RGBA::<f32> {
            r: 0.30,
            g: 0.60,
            b: 0.90,
            a: 0.50,
        };
        let cu8_2: RGB<u8> = cf32_2.into();
        assert_eq!(cu8_2.r, (0.30_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.g, (0.60_f32 * 255.0).round() as u8);
        assert_eq!(cu8_2.b, (0.90_f32 * 255.0).round() as u8);
    }

    #[test]
    fn test_rgba_to_hsl_conversion() {
        let epsilon = 0.001;

        let cu8 = RGBA::<u8> {
            r: 51,
            g: 153,
            b: 255,
            a: 128,
        };
        let hsl_u8: HSL = cu8.into();
        assert!((hsl_u8.h - 210.0).abs() < epsilon);
        assert!((hsl_u8.s - 1.0).abs() < epsilon);
        assert!((hsl_u8.l - 0.6).abs() < epsilon);

        let cf32 = RGBA::<f32> {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        };
        let hsl_f32: HSL = cf32.into();
        assert!((hsl_f32.h - 0.0).abs() < epsilon);
        assert!((hsl_f32.s - 1.0).abs() < epsilon);
        assert!((hsl_f32.l - 0.5).abs() < epsilon);
    }

    #[test]
    fn test_rgba_to_hsv_conversion() {
        let epsilon = 0.001;

        let cf32 = RGBA::<f32> {
            r: 0.2,
            g: 0.6,
            b: 1.0,
            a: 0.35,
        };
        let hsv_f32: HSV = cf32.into();
        assert!((hsv_f32.h - 210.0).abs() < epsilon);
        assert!((hsv_f32.s - 0.8).abs() < epsilon);
        assert!((hsv_f32.v - 1.0).abs() < epsilon);

        let cu8 = RGBA::<u8> {
            r: 51,
            g: 153,
            b: 255,
            a: 172,
        };
        let hsv_u8: HSV = cu8.into();
        assert!((hsv_u8.h - 210.0).abs() < epsilon);
        assert!((hsv_u8.s - 0.8).abs() < epsilon);
        assert!((hsv_u8.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_rgba_parse_hex() {
        let cu8_1: RGBA<u8> = "#FF5733CC".parse().unwrap();
        assert_eq!(cu8_1.r, 255);
        assert_eq!(cu8_1.g, 87);
        assert_eq!(cu8_1.b, 51);
        assert_eq!(cu8_1.a, 204);

        let cu8_2: RGBA<u8> = "00FF00".parse().unwrap();
        assert_eq!(cu8_2.r, 0);
        assert_eq!(cu8_2.g, 255);
        assert_eq!(cu8_2.b, 0);
        assert_eq!(cu8_2.a, 255);

        let cf32_1: RGBA<f32> = "#000000FF".parse().unwrap();
        assert_eq!(cf32_1.r, 0.0);
        assert_eq!(cf32_1.g, 0.0);
        assert_eq!(cf32_1.b, 0.0);
        assert_eq!(cf32_1.a, 1.0);

        assert_eq!(
            "FF5733C".parse::<RGBA<u8>>(),
            Err(ParseColorError::InvalidLength)
        );
        assert_eq!(
            "#FF5733XX".parse::<RGBA<f32>>(),
            Err(ParseColorError::InvalidFormat)
        );
    }

    #[test]
    fn test_rgba_to_hex() {
        let cu8 = RGBA::<u8> {
            r: 0,
            g: 255,
            b: 0,
            a: 170,
        };
        assert_eq!(format!("{:X}", cu8), "00FF00AA");
        assert_eq!(format!("{:x}", cu8), "00ff00aa");
        assert_eq!(format!("{:#X}", cu8), "#00FF00AA");
        assert_eq!(format!("{:#x}", cu8), "#00ff00aa");

        let cf32 = RGBA::<f32> {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        assert_eq!(format!("{:X}", cf32), "000000FF");
        assert_eq!(format!("{:#x}", cf32), "#000000ff");
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let epsilon = 0.001;

        let hsl_red = HSL {
            h: 0.0,
            s: 1.0,
            l: 0.5,
        };
        let rgb_red_f32: RGB<f32> = hsl_red.into();
        assert!((rgb_red_f32.r - 1.0).abs() < epsilon);
        assert!((rgb_red_f32.g - 0.0).abs() < epsilon);
        assert!((rgb_red_f32.b - 0.0).abs() < epsilon);

        let hsl_blue = HSL {
            h: 235.0,
            s: 0.54,
            l: 0.49,
        };
        let rgb_blue_f32: RGB<f32> = hsl_blue.into();
        assert!((rgb_blue_f32.r - 0.2254).abs() < epsilon);
        assert!((rgb_blue_f32.g - 0.2695).abs() < epsilon);
        assert!((rgb_blue_f32.b - 0.7546).abs() < epsilon);

        let rgb_blue_u8: RGB<u8> = hsl_blue.into();
        assert_eq!(rgb_blue_u8.r, 57);
        assert_eq!(rgb_blue_u8.g, 69);
        assert_eq!(rgb_blue_u8.b, 192);
    }

    #[test]
    fn test_hsl_to_rgba_conversion() {
        let epsilon = 0.001;

        let hsl_red = HSL {
            h: 0.0,
            s: 1.0,
            l: 0.5,
        };
        let rgb_red_f32: RGBA<f32> = hsl_red.into();
        assert!((rgb_red_f32.r - 1.0).abs() < epsilon);
        assert!((rgb_red_f32.g - 0.0).abs() < epsilon);
        assert!((rgb_red_f32.b - 0.0).abs() < epsilon);
        assert_eq!(rgb_red_f32.a, 1.0);

        let hsl_blue = HSL {
            h: 235.0,
            s: 0.54,
            l: 0.49,
        };
        let rgb_blue_f32: RGBA<f32> = hsl_blue.into();
        assert!((rgb_blue_f32.r - 0.2254).abs() < epsilon);
        assert!((rgb_blue_f32.g - 0.2695).abs() < epsilon);
        assert!((rgb_blue_f32.b - 0.7546).abs() < epsilon);
        assert_eq!(rgb_blue_f32.a, 1.0);

        let rgb_blue_u8: RGBA<u8> = hsl_blue.into();
        assert_eq!(rgb_blue_u8.r, 57);
        assert_eq!(rgb_blue_u8.g, 69);
        assert_eq!(rgb_blue_u8.b, 192);
        assert_eq!(rgb_blue_u8.a, 255);
    }

    #[test]
    fn test_hsl_to_hsv_conversion() {
        let epsilon = 0.001;

        let hsl = HSL {
            h: 210.0,
            s: 1.0,
            l: 0.6,
        };
        let hsv: HSV = hsl.into();
        assert!((hsv.h - 210.0).abs() < epsilon);
        assert!((hsv.s - 0.8).abs() < epsilon);
        assert!((hsv.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_hsv_to_rgb_conversion() {
        let epsilon = 0.001;

        let hsv = HSV {
            h: 210.0,
            s: 0.8,
            v: 1.0,
        };

        let cf32: RGB<f32> = hsv.into();
        assert!((cf32.r - 0.2).abs() < epsilon);
        assert!((cf32.g - 0.6).abs() < epsilon);
        assert!((cf32.b - 1.0).abs() < epsilon);

        let cu8: RGB<u8> = hsv.into();
        assert_eq!(cu8.r, 51);
        assert_eq!(cu8.g, 153);
        assert_eq!(cu8.b, 255);
    }

    #[test]
    fn test_hsv_to_rgba_conversion() {
        let epsilon = 0.001;

        let hsv = HSV {
            h: 210.0,
            s: 0.8,
            v: 1.0,
        };

        let cf32: RGBA<f32> = hsv.into();
        assert!((cf32.r - 0.2).abs() < epsilon);
        assert!((cf32.g - 0.6).abs() < epsilon);
        assert!((cf32.b - 1.0).abs() < epsilon);
        assert_eq!(cf32.a, 1.0);

        let cu8: RGBA<u8> = hsv.into();
        assert_eq!(cu8.r, 51);
        assert_eq!(cu8.g, 153);
        assert_eq!(cu8.b, 255);
        assert_eq!(cu8.a, 255);
    }

    #[test]
    fn test_hsv_to_hsl_conversion() {
        let epsilon = 0.001;

        let hsv = HSV {
            h: 45.0,
            s: 0.75,
            v: 0.8,
        };
        let hsl: HSL = hsv.into();
        assert!((hsl.h - 45.0).abs() < epsilon);
        assert!((hsl.s - 0.6).abs() < epsilon);
        assert!((hsl.l - 0.5).abs() < epsilon);
    }
}
