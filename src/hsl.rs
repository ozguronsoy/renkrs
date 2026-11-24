use crate::{Cmyk, Color, Hsv, Rgb, Rgba};

/// Stores color information as hue, saturation, and lightness.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    /// Hue. Ranges from 0 to 360.
    pub h: f32,
    /// Saturation. Ranges from 0 to 1.
    pub s: f32,
    /// Lightness. Ranges from 0 to 1.
    pub l: f32,
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }
}

impl Default for Hsl {
    fn default() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        }
    }
}

impl Color<f32> for Hsl {
    fn as_slice(&self) -> &[f32] {
        // SAFETY: `Hsl` is `#[repr(C)]` and consists of exactly three fields of type `f32`,
        // so it has the same layout as `[f32; 3]`.
        unsafe { std::slice::from_raw_parts((self as *const Self).cast::<f32>(), 3) }
    }
}

impl From<Rgb<u8>> for Hsl {
    fn from(rgb: Rgb<u8>) -> Self {
        Rgb::<f32>::from(rgb).into()
    }
}

impl From<Rgb<f32>> for Hsl {
    fn from(rgb: Rgb<f32>) -> Self {
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

impl From<Rgba<u8>> for Hsl {
    fn from(rgba: Rgba<u8>) -> Self {
        Rgb::<f32>::from(rgba).into()
    }
}

impl From<Rgba<f32>> for Hsl {
    fn from(rgba: Rgba<f32>) -> Self {
        Rgb::<f32>::from(rgba).into()
    }
}

impl From<Hsv> for Hsl {
    fn from(hsv: Hsv) -> Self {
        let l = hsv.v * (1.0 - hsv.s / 2.0);
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (hsv.v - l) / l.min(1.0 - l)
        };
        Self { h: hsv.h, s, l }
    }
}

impl From<Cmyk> for Hsl {
    fn from(cmyk: Cmyk) -> Self {
        Rgb::<f32>::from(cmyk).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_new() {
        let hsl = Hsl::new(235.0, 0.54, 0.49);
        assert_eq!(hsl.h, 235.0);
        assert_eq!(hsl.s, 0.54);
        assert_eq!(hsl.l, 0.49);
    }

    #[test]
    fn test_hsl_default() {
        let hsl = Hsl::default();
        assert_eq!(hsl.h, 0.0);
        assert_eq!(hsl.s, 0.0);
        assert_eq!(hsl.l, 0.0);
    }

    #[test]
    fn test_as_slice() {
        let hsl = Hsl::new(235.0, 0.54, 0.49);
        assert_eq!(hsl.as_slice(), &[235.0, 0.54, 0.49]);
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let epsilon = 0.001;

        let hsl_red = Hsl::new(0.0, 1.0, 0.5);
        let rgb_red_f32: Rgb<f32> = hsl_red.into();
        assert!((rgb_red_f32.r - 1.0).abs() < epsilon);
        assert!((rgb_red_f32.g - 0.0).abs() < epsilon);
        assert!((rgb_red_f32.b - 0.0).abs() < epsilon);

        let hsl_blue = Hsl::new(235.0, 0.54, 0.49);
        let rgb_blue_f32: Rgb<f32> = hsl_blue.into();
        assert!((rgb_blue_f32.r - 0.2254).abs() < epsilon);
        assert!((rgb_blue_f32.g - 0.2695).abs() < epsilon);
        assert!((rgb_blue_f32.b - 0.7546).abs() < epsilon);

        let rgb_blue_u8: Rgb<u8> = hsl_blue.into();
        assert_eq!(rgb_blue_u8.r, 57);
        assert_eq!(rgb_blue_u8.g, 69);
        assert_eq!(rgb_blue_u8.b, 192);
    }

    #[test]
    fn test_hsl_to_rgba_conversion() {
        let epsilon = 0.001;

        let hsl_red = Hsl::new(0.0, 1.0, 0.5);
        let rgb_red_f32: Rgba<f32> = hsl_red.into();
        assert!((rgb_red_f32.r - 1.0).abs() < epsilon);
        assert!((rgb_red_f32.g - 0.0).abs() < epsilon);
        assert!((rgb_red_f32.b - 0.0).abs() < epsilon);
        assert_eq!(rgb_red_f32.a, 1.0);

        let hsl_blue = Hsl::new(235.0, 0.54, 0.49);
        let rgb_blue_f32: Rgba<f32> = hsl_blue.into();
        assert!((rgb_blue_f32.r - 0.2254).abs() < epsilon);
        assert!((rgb_blue_f32.g - 0.2695).abs() < epsilon);
        assert!((rgb_blue_f32.b - 0.7546).abs() < epsilon);
        assert_eq!(rgb_blue_f32.a, 1.0);

        let rgb_blue_u8: Rgba<u8> = hsl_blue.into();
        assert_eq!(rgb_blue_u8.r, 57);
        assert_eq!(rgb_blue_u8.g, 69);
        assert_eq!(rgb_blue_u8.b, 192);
        assert_eq!(rgb_blue_u8.a, 255);
    }

    #[test]
    fn test_hsl_to_hsv_conversion() {
        let epsilon = 0.001;

        let hsl = Hsl::new(210.0, 1.0, 0.6);
        let hsv: Hsv = hsl.into();
        assert!((hsv.h - 210.0).abs() < epsilon);
        assert!((hsv.s - 0.8).abs() < epsilon);
        assert!((hsv.v - 1.0).abs() < epsilon);
    }

    #[test]
    fn test_hsl_cmyk_cross_conversion() {
        let epsilon = 0.002;

        let hsl_purple = Hsl::new(270.0, 0.5, 0.4);
        let cmyk: Cmyk = hsl_purple.into();
        assert!((cmyk.c - 0.333).abs() < epsilon);
        assert!((cmyk.m - 0.667).abs() < epsilon);
        assert!((cmyk.y - 0.0).abs() < epsilon);
        assert!((cmyk.k - 0.4).abs() < epsilon);
    }
}
