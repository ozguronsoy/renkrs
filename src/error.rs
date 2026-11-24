/// Possible errors that can occur when parsing hex strings.
#[derive(Debug, PartialEq)]
pub enum ParseColorError {
    /// The hex string was not exactly 6 or 8 characters.
    InvalidLength,
    /// The hex string contained non-hexadecimal characters.
    InvalidFormat,
}

impl std::error::Error for ParseColorError {}
impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseColorError::InvalidLength => {
                write!(f, "Hex color string must be 6 or 8 characters long.")
            }
            ParseColorError::InvalidFormat => {
                write!(f, "Hex color string contains invalid characters.")
            }
        }
    }
}
