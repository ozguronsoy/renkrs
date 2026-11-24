use crate::color_channel::for_each_channel_pair;
use crate::{Cmyk, Color, ColorChannel, Hsl, Hsv, ParseColorError, Rgba};

/// Stores color information as red, green, and blue.
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

impl<T: ColorChannel> Default for Rgb<T> {
    fn default() -> Self {
        let zero = T::from_u8(0);
        Self::new(zero, zero, zero)
    }
}

impl<T: ColorChannel> Color<T> for Rgb<T> {
    fn as_slice(&self) -> &[T] {
        // SAFETY: `Rgb` is `#[repr(C)]` and consists of exactly three fields of type `T`,
        // so it has the same layout as `[T; 3]`.
        unsafe { std::slice::from_raw_parts((self as *const Self).cast::<T>(), 3) }
    }
}

macro_rules! impl_rgb_channel_conversion {
    ($from:ty, $to:ty) => {
        impl From<Rgb<$from>> for Rgb<$to> {
            fn from(rgb: Rgb<$from>) -> Self {
                Self::new(rgb.r.convert(), rgb.g.convert(), rgb.b.convert())
            }
        }
    };
}

for_each_channel_pair!(impl_rgb_channel_conversion);

impl<T: ColorChannel, U: ColorChannel> From<Rgba<T>> for Rgb<U> {
    fn from(rgba: Rgba<T>) -> Self {
        Self::new(rgba.r.convert(), rgba.g.convert(), rgba.b.convert())
    }
}

impl<T: ColorChannel> From<Hsl> for Rgb<T> {
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

        Self::new(
            T::from_f32(r_prime + m),
            T::from_f32(g_prime + m),
            T::from_f32(b_prime + m),
        )
    }
}

impl<T: ColorChannel> From<Hsv> for Rgb<T> {
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

        Self::new(
            T::from_f32(r_prime + m),
            T::from_f32(g_prime + m),
            T::from_f32(b_prime + m),
        )
    }
}

impl<T: ColorChannel> From<Cmyk> for Rgb<T> {
    fn from(cmyk: Cmyk) -> Self {
        Self::new(
            T::from_f32((1.0 - cmyk.c) * (1.0 - cmyk.k)),
            T::from_f32((1.0 - cmyk.m) * (1.0 - cmyk.k)),
            T::from_f32((1.0 - cmyk.y) * (1.0 - cmyk.k)),
        )
    }
}

impl<T: ColorChannel> std::str::FromStr for Rgb<T> {
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
            Ok(Self::new(T::from_u8(r), T::from_u8(g), T::from_u8(b)))
        } else if hex.len() == 8 {
            let rgba: Rgba<T> = hex.parse()?;
            Ok(rgba.into())
        } else {
            Err(ParseColorError::InvalidLength)
        }
    }
}

impl<T: ColorChannel> std::fmt::LowerHex for Rgb<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: Rgb<u8> = Rgb::new(self.r.convert(), self.g.convert(), self.b.convert());
        // If the user uses the alternate flag `{:#x}`, prefix with '#'
        if f.alternate() {
            write!(f, "#")?;
        }
        write!(f, "{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }
}

impl<T: ColorChannel> std::fmt::UpperHex for Rgb<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb: Rgb<u8> = Rgb::new(self.r.convert(), self.g.convert(), self.b.convert());
        // If the user uses the alternate flag `{:#X}`, prefix with '#'
        if f.alternate() {
            write!(f, "#")?;
        }
        write!(f, "{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_new() {
        let cu8 = Rgb::<u8>::new(18, 86, 205);
        assert_eq!(cu8.r, 18);
        assert_eq!(cu8.g, 86);
        assert_eq!(cu8.b, 205);

        let cf32 = Rgb::<f32>::new(0.10, 0.25, 0.40);
        assert_eq!(cf32.r, 0.10);
        assert_eq!(cf32.g, 0.25);
        assert_eq!(cf32.b, 0.40);
    }

    #[test]
    fn test_as_slice() {
        let cu8 = Rgb::<u8>::new(18, 86, 205);
        assert_eq!(cu8.as_slice(), &[18, 86, 205]);

        let cf32 = Rgb::<f32>::new(0.10, 0.25, 0.40);
        assert_eq!(cf32.as_slice(), &[0.10, 0.25, 0.40]);
    }

    #[test]
    fn test_rgb_channel_type_conversion() {
        let cu8 = Rgb::<u8>::new(0x12, 0x80, 0xFF);

        let cu16: Rgb<u16> = cu8.into();
        assert_eq!(cu16, Rgb::new(0x1212, 0x8080, 0xFFFF));
        assert_eq!(Rgb::<u8>::from(cu16), cu8);

        let ci8: Rgb<i8> = cu8.into();
        assert_eq!(ci8, Rgb::new(9, 64, 127));

        let ci32: Rgb<i32> = Rgb::<f64>::new(1.0, 0.0, -1.0).into();
        assert_eq!(ci32, Rgb::new(i32::MAX, 0, -i32::MAX));
        assert_eq!(Rgb::<u8>::from(ci32), Rgb::new(255, 0, 0));

        let cf64: Rgb<f64> = Rgb::<i16>::new(i16::MAX, 0, i16::MAX).into();
        assert_eq!(cf64, Rgb::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn test_rgb_extended_channel_types() {
        assert_eq!(Rgb::<u16>::default(), Rgb::new(0, 0, 0));
        assert_eq!(Rgb::<f64>::default(), Rgb::new(0.0, 0.0, 0.0));

        let ci16: Rgb<i16> = "#FF0000".parse().unwrap();
        assert_eq!(ci16, Rgb::new(i16::MAX, 0, 0));
        assert_eq!(format!("{:#x}", ci16), "#ff0000");

        let cu16: Rgb<u16> = Hsl::new(0.0, 1.0, 0.5).into();
        assert_eq!(cu16, Rgb::new(u16::MAX, 0, 0));
        let hsl: Hsl = cu16.into();
        assert_eq!(hsl, Hsl::new(0.0, 1.0, 0.5));
    }

    #[test]
    fn test_rgb_default() {
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
