/// A marker trait used to restrict color channels to valid types.
pub trait ColorChannel {}

/// Implements `ColorChannel` for 8-bit unsigned integers.
impl ColorChannel for u8 {}
/// Implements `ColorChannel` for 32-bit floating-point numbers.
impl ColorChannel for f32 {}
