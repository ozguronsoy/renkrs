/// A numeric type that can be used as a color channel.
pub trait ColorChannel: Copy {
    /// Creates a channel from a normalized value in `0.0..=1.0`
    /// (`-1.0..=1.0` for signed integers).
    fn from_f64(value: f64) -> Self;

    /// Converts this channel to another channel type, rescaling as needed.
    fn convert<U: ColorChannel>(self) -> U;

    /// Creates a channel from an 8-bit value in `0..=255`.
    #[inline]
    fn from_u8(value: u8) -> Self {
        Self::from_f64(value as f64 / u8::MAX as f64)
    }

    /// Creates a channel from a 16-bit value in `0..=65535`.
    #[inline]
    fn from_u16(value: u16) -> Self {
        Self::from_f64(value as f64 / u16::MAX as f64)
    }

    /// Creates a channel from a 32-bit value in `0..=4294967295`.
    #[inline]
    fn from_u32(value: u32) -> Self {
        Self::from_f64(value as f64 / u32::MAX as f64)
    }

    /// Creates a channel from a signed normalized 8-bit value.
    #[inline]
    fn from_i8(value: i8) -> Self {
        Self::from_f64((value as f64 / i8::MAX as f64).max(-1.0))
    }

    /// Creates a channel from a signed normalized 16-bit value.
    #[inline]
    fn from_i16(value: i16) -> Self {
        Self::from_f64((value as f64 / i16::MAX as f64).max(-1.0))
    }

    /// Creates a channel from a signed normalized 32-bit value.
    #[inline]
    fn from_i32(value: i32) -> Self {
        Self::from_f64((value as f64 / i32::MAX as f64).max(-1.0))
    }

    /// Creates a channel from a normalized value in `0.0..=1.0`.
    #[inline]
    fn from_f32(value: f32) -> Self {
        Self::from_f64(value as f64)
    }
}

/// Implements `ColorChannel` for an integer type. `0` maps to `0.0` and `MAX` maps to `1.0`.
/// For signed types, negative values map to `-1.0..0.0` (SNORM) and are clamped to `0`
/// when converted to an unsigned type.
macro_rules! impl_integer_channel {
    ($t:ty, $from:ident, $doc:literal) => {
        #[doc = $doc]
        impl ColorChannel for $t {
            #[inline]
            fn from_f64(value: f64) -> Self {
                // Clamping keeps -1.0 at -MAX for signed types; `as` then saturates
                // negatives to 0 for unsigned types and turns NaN into 0.
                (value.clamp(-1.0, 1.0) * <$t>::MAX as f64).round() as $t
            }

            #[inline]
            fn convert<U: ColorChannel>(self) -> U {
                U::$from(self)
            }

            #[inline]
            fn $from(value: $t) -> Self {
                value
            }
        }
    };
}

/// Implements `ColorChannel` for a floating-point type, normalized to `0.0..=1.0`.
macro_rules! impl_float_channel {
    ($t:ty, $from:ident, $doc:literal) => {
        #[doc = $doc]
        impl ColorChannel for $t {
            #[inline]
            fn from_f64(value: f64) -> Self {
                value as $t
            }

            #[inline]
            fn convert<U: ColorChannel>(self) -> U {
                U::$from(self)
            }

            #[inline]
            fn $from(value: $t) -> Self {
                value
            }
        }
    };
}

impl_integer_channel!(
    u8,
    from_u8,
    "Implements `ColorChannel` for 8-bit unsigned integers."
);
impl_integer_channel!(
    u16,
    from_u16,
    "Implements `ColorChannel` for 16-bit unsigned integers."
);
impl_integer_channel!(
    u32,
    from_u32,
    "Implements `ColorChannel` for 32-bit unsigned integers."
);
impl_integer_channel!(
    i8,
    from_i8,
    "Implements `ColorChannel` for 8-bit signed (SNORM) integers."
);
impl_integer_channel!(
    i16,
    from_i16,
    "Implements `ColorChannel` for 16-bit signed (SNORM) integers."
);
impl_integer_channel!(
    i32,
    from_i32,
    "Implements `ColorChannel` for 32-bit signed (SNORM) integers."
);
impl_float_channel!(
    f32,
    from_f32,
    "Implements `ColorChannel` for 32-bit floating-point numbers."
);

/// Implements `ColorChannel` for 64-bit floating-point numbers.
impl ColorChannel for f64 {
    #[inline]
    fn from_f64(value: f64) -> Self {
        value
    }

    #[inline]
    fn convert<U: ColorChannel>(self) -> U {
        U::from_f64(self)
    }
}

/// Invokes `$m!(From, To)` for every ordered pair of distinct built-in channel types.
///
/// Used to implement conversions like `From<Rgb<u8>> for Rgb<f32>`, which can't be written
/// generically because `From<Rgb<T>> for Rgb<U>` would overlap with `impl<T> From<T> for T`.
macro_rules! for_each_channel_pair {
    ($m:ident) => {
        $crate::color_channel::for_each_channel_pair!(@pairs $m; u8, u16, u32, i8, i16, i32, f32, f64);
    };
    (@pairs $m:ident; $head:ty $(, $tail:ty)*) => {
        $(
            $m!($head, $tail);
            $m!($tail, $head);
        )*
        $crate::color_channel::for_each_channel_pair!(@pairs $m; $($tail),*);
    };
    (@pairs $m:ident;) => {};
}

pub(crate) use for_each_channel_pair;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_conversion() {
        assert_eq!(200u8.convert::<u8>(), 200);
        assert_eq!(51_234u16.convert::<u16>(), 51_234);
        assert_eq!(4_000_000_000u32.convert::<u32>(), 4_000_000_000);
        assert_eq!((-100i8).convert::<i8>(), -100);
        assert_eq!((-12_345i16).convert::<i16>(), -12_345);
        assert_eq!(i32::MIN.convert::<i32>(), i32::MIN);
        assert_eq!(0.123f32.convert::<f32>(), 0.123);
        assert_eq!(0.123f64.convert::<f64>(), 0.123);
    }

    #[test]
    fn test_unsigned_conversion() {
        assert_eq!(255u8.convert::<u16>(), u16::MAX);
        assert_eq!(0x12u8.convert::<u16>(), 0x1212);
        assert_eq!(0x1212u16.convert::<u8>(), 0x12);
        assert_eq!(51u8.convert::<f32>(), 0.2);
        assert_eq!(0.2f64.convert::<u8>(), 51);
        assert_eq!(u16::MAX.convert::<f64>(), 1.0);
        assert_eq!(0x12u8.convert::<u32>(), 0x1212_1212);
        assert_eq!(0x1234u16.convert::<u32>(), 0x1234_1234);
        assert_eq!(0x1212_1212u32.convert::<u8>(), 0x12);
        assert_eq!(u32::MAX.convert::<i32>(), i32::MAX);
        assert_eq!(i32::MAX.convert::<u32>(), u32::MAX);
    }

    #[test]
    fn test_signed_conversion() {
        assert_eq!(i8::MAX.convert::<f32>(), 1.0);
        assert_eq!(i8::MIN.convert::<f32>(), -1.0);
        assert_eq!((-127i8).convert::<f32>(), -1.0);
        assert_eq!(0i8.convert::<f32>(), 0.0);
        assert_eq!(1.0f32.convert::<i16>(), i16::MAX);
        assert_eq!((-1.0f32).convert::<i16>(), -i16::MAX);
        assert_eq!(255u8.convert::<i8>(), i8::MAX);
        assert_eq!(i16::MAX.convert::<i32>(), i32::MAX);
        assert_eq!(i32::MAX.convert::<u8>(), u8::MAX);
    }

    #[test]
    fn test_signed_to_unsigned_clamps_negatives() {
        assert_eq!((-50i8).convert::<u8>(), 0);
        assert_eq!(i16::MIN.convert::<u16>(), 0);
        assert_eq!(i32::MIN.convert::<u8>(), 0);
    }

    #[test]
    fn test_out_of_range_floats_saturate() {
        assert_eq!(1.5f32.convert::<u8>(), u8::MAX);
        assert_eq!((-0.5f64).convert::<u16>(), 0);
        assert_eq!(2.0f64.convert::<i8>(), i8::MAX);
        assert_eq!((-2.0f32).convert::<i8>(), -i8::MAX);
        assert_eq!(f32::NAN.convert::<u8>(), 0);
    }

    #[test]
    fn test_i32_round_trip_is_lossless() {
        for v in [i32::MAX, i32::MAX - 1, 1, -1, -123_456_789, i32::MIN + 1] {
            assert_eq!(v.convert::<f64>().convert::<i32>(), v);
        }
    }

    #[test]
    fn test_u32_round_trip_is_lossless() {
        for v in [u32::MAX, u32::MAX - 1, 0, 1, 123_456_789, 2_147_483_648] {
            assert_eq!(v.convert::<f64>().convert::<u32>(), v);
        }
    }
}
