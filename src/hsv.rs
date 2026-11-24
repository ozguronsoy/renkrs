use crate::{CMYK, HSL, RGB, RGBA};

/// Stores color information as hue, saturation, and value.
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    /// Hue. Ranges from 0 to 360.
    pub h: f32,
    /// Saturation. Ranges from 0 to 1.
    pub s: f32,
    /// Value. Ranges from 0 to 1.
    pub v: f32,
}

impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
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

impl From<CMYK> for HSV {
    fn from(cmyk: CMYK) -> Self {
        RGB::<f32>::from(cmyk).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb_conversion() {
        let epsilon = 0.001;

        let hsv = HSV::new(210.0, 0.8, 1.0);

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

        let hsv = HSV::new(210.0, 0.8, 1.0);

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

        let hsv = HSV::new(45.0, 0.75, 0.8);
        let hsl: HSL = hsv.into();
        assert!((hsl.h - 45.0).abs() < epsilon);
        assert!((hsl.s - 0.6).abs() < epsilon);
        assert!((hsl.l - 0.5).abs() < epsilon);
    }

    #[test]
    fn test_hsv_to_cmyk_conversion() {
        let epsilon = 0.002;

        let hsv_purple = HSV::new(270.0, 0.666, 0.6);
        let cmyk: CMYK = hsv_purple.into();
        assert!((cmyk.c - 0.333).abs() < epsilon);
        assert!((cmyk.m - 0.667).abs() < epsilon);
        assert!((cmyk.y - 0.0).abs() < epsilon);
        assert!((cmyk.k - 0.4).abs() < epsilon);
    }
}
