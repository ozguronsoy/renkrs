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
    fn test_rgba_to_rgb_cross_type_conversion() {
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
}
