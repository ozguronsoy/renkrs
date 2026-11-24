use crate::color_channel::for_each_channel_pair;
use crate::{Cmyk, Color, ColorChannel, Hsl, Hsv, ParseColorError, Rgb};

/// Stores color information as red, green, and blue with an alpha channel.
///
/// # Type Parameters
///
/// * `T` - The numeric type of the color channels. This is restricted by the [`ColorChannel`] trait.
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

impl<T: ColorChannel> Default for Rgba<T> {
    fn default() -> Self {
        let zero = T::from_u8(0);
        Self::new(zero, zero, zero, T::from_u8(255))
    }
}

impl<T: ColorChannel> Color<T> for Rgba<T> {
    fn as_slice(&self) -> &[T] {
        // SAFETY: `Rgba` is `#[repr(C)]` and consists of exactly four fields of type `T`,
        // so it has the same layout as `[T; 4]`.
        unsafe { std::slice::from_raw_parts((self as *const Self).cast::<T>(), 4) }
    }
}

macro_rules! impl_rgba_channel_conversion {
    ($from:ty, $to:ty) => {
        impl From<Rgba<$from>> for Rgba<$to> {
            fn from(rgba: Rgba<$from>) -> Self {
                Self::new(
                    rgba.r.convert(),
                    rgba.g.convert(),
                    rgba.b.convert(),
                    rgba.a.convert(),
                )
            }
        }
    };
}

for_each_channel_pair!(impl_rgba_channel_conversion);

impl<T: ColorChannel, U: ColorChannel> From<Rgb<T>> for Rgba<U> {
    fn from(rgb: Rgb<T>) -> Self {
        Self::new(
            rgb.r.convert(),
            rgb.g.convert(),
            rgb.b.convert(),
            U::from_u8(255),
        )
    }
}

impl<T: ColorChannel> From<Hsl> for Rgba<T> {
    fn from(hsl: Hsl) -> Self {
        Rgb::<T>::from(hsl).into()
    }
}

impl<T: ColorChannel> From<Hsv> for Rgba<T> {
    fn from(hsv: Hsv) -> Self {
        Rgb::<T>::from(hsv).into()
    }
}

impl<T: ColorChannel> From<Cmyk> for Rgba<T> {
    fn from(cmyk: Cmyk) -> Self {
        Rgb::<T>::from(cmyk).into()
    }
}

impl<T: ColorChannel> std::str::FromStr for Rgba<T> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() == 6 {
            let rgb: Rgb<T> = hex.parse()?;
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
            Ok(Self::new(
                T::from_u8(r),
                T::from_u8(g),
                T::from_u8(b),
                T::from_u8(a),
            ))
        } else {
            Err(ParseColorError::InvalidLength)
        }
    }
}

impl<T: ColorChannel> std::fmt::LowerHex for Rgba<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: Rgba<u8> = Rgba::new(
            self.r.convert(),
            self.g.convert(),
            self.b.convert(),
            self.a.convert(),
        );
        // If the user uses the alternate flag `{:#x}`, prefix with '#'
        if f.alternate() {
            write!(f, "#")?;
        }
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}",
            rgba.r, rgba.g, rgba.b, rgba.a
        )
    }
}

impl<T: ColorChannel> std::fmt::UpperHex for Rgba<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgba: Rgba<u8> = Rgba::new(
            self.r.convert(),
            self.g.convert(),
            self.b.convert(),
            self.a.convert(),
        );
        // If the user uses the alternate flag `{:#X}`, prefix with '#'
        if f.alternate() {
            write!(f, "#")?;
        }
        write!(
            f,
            "{:02X}{:02X}{:02X}{:02X}",
            rgba.r, rgba.g, rgba.b, rgba.a
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_new() {
        let cu8 = Rgba::<u8>::new(18, 86, 205, 163);
        assert_eq!(cu8.r, 18);
        assert_eq!(cu8.g, 86);
        assert_eq!(cu8.b, 205);
        assert_eq!(cu8.a, 163);

        let cf32 = Rgba::<f32>::new(0.10, 0.25, 0.40, 0.86);
        assert_eq!(cf32.r, 0.10);
        assert_eq!(cf32.g, 0.25);
        assert_eq!(cf32.b, 0.40);
        assert_eq!(cf32.a, 0.86);
    }

    #[test]
    fn test_as_slice() {
        let cu8 = Rgba::<u8>::new(18, 86, 205, 163);
        assert_eq!(cu8.as_slice(), &[18, 86, 205, 163]);

        let cf32 = Rgba::<f32>::new(0.10, 0.25, 0.40, 0.86);
        assert_eq!(cf32.as_slice(), &[0.10, 0.25, 0.40, 0.86]);
    }

    #[test]
    fn test_rgba_channel_type_conversion() {
        let cu8 = Rgba::<u8>::new(0x12, 0x80, 0xFF, 0x00);

        let cu16: Rgba<u16> = cu8.into();
        assert_eq!(cu16, Rgba::new(0x1212, 0x8080, 0xFFFF, 0));
        assert_eq!(Rgba::<u8>::from(cu16), cu8);

        let ci16: Rgba<i16> = Rgba::<i8>::new(-127, 0, 127, 127).into();
        assert_eq!(ci16, Rgba::new(-i16::MAX, 0, i16::MAX, i16::MAX));
        assert_eq!(Rgba::<u8>::from(ci16), Rgba::new(0, 0, 255, 255));

        let cf64: Rgba<f64> = Rgba::<u16>::new(u16::MAX, 0, 0, u16::MAX).into();
        assert_eq!(cf64, Rgba::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rgba_extended_channel_types() {
        assert_eq!(Rgba::<u16>::default(), Rgba::new(0, 0, 0, u16::MAX));
        assert_eq!(Rgba::<i8>::default(), Rgba::new(0, 0, 0, i8::MAX));
        assert_eq!(Rgba::<f64>::default(), Rgba::new(0.0, 0.0, 0.0, 1.0));

        let cu16: Rgba<u16> = "#00FF0080".parse().unwrap();
        assert_eq!(cu16, Rgba::new(0, u16::MAX, 0, 0x8080));
        assert_eq!(format!("{:#X}", cu16), "#00FF0080");

        let ci32: Rgba<i32> = Rgb::<u8>::new(255, 0, 0).into();
        assert_eq!(ci32, Rgba::new(i32::MAX, 0, 0, i32::MAX));
    }

    #[test]
    fn test_rgba_default() {
        let cu8 = Rgba::<u8>::default();
        let cf32 = Rgba::<f32>::default();
        assert_eq!(cu8.r, 0);
        assert_eq!(cu8.g, 0);
        assert_eq!(cu8.b, 0);
        assert_eq!(cu8.a, 255);
        assert_eq!(cf32.r, 0.0);
        assert_eq!(cf32.g, 0.0);
        assert_eq!(cf32.b, 0.0);
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
        assert_eq!(cu8_2.r, 77);
        assert_eq!(cu8_2.g, 153);
        assert_eq!(cu8_2.b, 229);
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
