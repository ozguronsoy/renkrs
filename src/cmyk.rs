use crate::{Hsl, Hsv, Rgb, Rgba};

/// Stores color information as cyan, magenta, yellow, and key.
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyk {
    /// Cyan. Ranges from 0 to 1.
    pub c: f32,
    /// Magenta. Ranges from 0 to 1.
    pub m: f32,
    /// Yellow. Ranges from 0 to 1.
    pub y: f32,
    /// Key. Ranges from 0 to 1.
    pub k: f32,
}

impl Cmyk {
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }
}

impl Default for Cmyk {
    fn default() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 0.0,
        }
    }
}

impl From<Rgb<u8>> for Cmyk {
    fn from(rgb: Rgb<u8>) -> Self {
        Rgb::<f32>::from(rgb).into()
    }
}

impl From<Rgb<f32>> for Cmyk {
    fn from(rgb: Rgb<f32>) -> Self {
        let k = 1.0 - rgb.r.max(rgb.g).max(rgb.b);
        if (k - 1.0).abs() < f32::EPSILON {
            Self {
                c: 0.0,
                m: 0.0,
                y: 0.0,
                k: 1.0,
            }
        } else {
            Self {
                c: (1.0 - rgb.r - k) / (1.0 - k),
                m: (1.0 - rgb.g - k) / (1.0 - k),
                y: (1.0 - rgb.b - k) / (1.0 - k),
                k,
            }
        }
    }
}

impl From<Rgba<u8>> for Cmyk {
    fn from(rgba: Rgba<u8>) -> Self {
        Rgb::<f32>::from(rgba).into()
    }
}

impl From<Rgba<f32>> for Cmyk {
    fn from(rgba: Rgba<f32>) -> Self {
        Rgb::<f32>::from(rgba).into()
    }
}

impl From<Hsl> for Cmyk {
    fn from(hsl: Hsl) -> Self {
        Rgb::<f32>::from(hsl).into()
    }
}

impl From<Hsv> for Cmyk {
    fn from(hsv: Hsv) -> Self {
        Rgb::<f32>::from(hsv).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmyk_to_rgb_conversion() {
        let epsilon = 0.001;

        let cmyk_cyan = Cmyk::new(1.0, 0.0, 0.0, 0.0);
        let rgb_cyan: Rgb<f32> = cmyk_cyan.into();
        assert!((rgb_cyan.r - 0.0).abs() < epsilon);
        assert!((rgb_cyan.g - 1.0).abs() < epsilon);
        assert!((rgb_cyan.b - 1.0).abs() < epsilon);

        let cmyk_mixed = Cmyk::new(0.5, 0.25, 0.0, 0.2);
        let rgb_u8: Rgb<u8> = cmyk_mixed.into();
        assert_eq!(rgb_u8.r, 102);
        assert_eq!(rgb_u8.g, 153);
        assert_eq!(rgb_u8.b, 204);
    }

    #[test]
    fn test_cmyk_to_rgba_conversion() {
        let epsilon = 0.001;

        let cmyk_cyan = Cmyk::new(1.0, 0.0, 0.0, 0.0);
        let rgba_cyan: Rgba<f32> = cmyk_cyan.into();
        assert!((rgba_cyan.r - 0.0).abs() < epsilon);
        assert!((rgba_cyan.g - 1.0).abs() < epsilon);
        assert!((rgba_cyan.b - 1.0).abs() < epsilon);

        let cmyk_mixed = Cmyk::new(0.5, 0.25, 0.0, 0.2);
        let rgba_u8: Rgba<u8> = cmyk_mixed.into();
        assert_eq!(rgba_u8.r, 102);
        assert_eq!(rgba_u8.g, 153);
        assert_eq!(rgba_u8.b, 204);
        assert_eq!(rgba_u8.a, 255);
    }

    #[test]
    fn test_cmyk_to_hsl_conversion() {
        let epsilon = 0.002;

        let cmyk_purple = Cmyk::new(1.0 / 3.0, 2.0 / 3.0, 0.0, 0.4);
        let hsl: Hsl = cmyk_purple.into();
        assert!((hsl.h - 270.0).abs() < epsilon);
        assert!((hsl.s - 0.5).abs() < epsilon);
        assert!((hsl.l - 0.4).abs() < epsilon);
    }

    #[test]
    fn test_cmyk_to_hsv_conversion() {
        let epsilon = 0.002;

        let cmyk_purple = Cmyk::new(1.0 / 3.0, 2.0 / 3.0, 0.0, 0.4);
        let hsv: Hsv = cmyk_purple.into();
        assert!((hsv.h - 270.0).abs() < epsilon);
        assert!((hsv.s - 0.666).abs() < epsilon, "HSV Saturation failed");
        assert!((hsv.v - 0.6).abs() < epsilon);
    }
}
