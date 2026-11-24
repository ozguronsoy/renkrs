use crate::ColorChannel;

/// Common interface for all color types.
pub trait Color<T: ColorChannel> {
    /// Returns the color's channels as a slice, in field declaration order.
    fn as_slice(&self) -> &[T];
}
