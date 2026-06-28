# renkrs

Provides structs for RGB, RGBA, HSL, HSV, and CMYK color models with zero-cost conversions between them.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
renkrs = "0.1.0"
```

or run the following command:
```bash
cargo add renkrs
```

## Usage

`renkrs` implements `From` conversions between each type, allowing one to seamlessly convert between color spaces using `into`. `renkrs` also implements `FromStr`, `LowerHex`, and `UpperHex` for RGB and RGBA colors.

```rust
use renkrs::{RGB, RGBA, HSL, HSV, CMYK};

let color_u8: RGBA<u8> = "#FF5733".parse().unwrap();
let color_f32: RGB<f32> = color_u8.into();
let hsl: HSL = color_u8.into();
let hsv: HSV = color_f32.into();
let cmyk: CMYK = hsv.into();
let hex_lower: String = format!("{:#x}", color_u8);
let hex_upper: String = format!("{:#X}", color_f32);
```

## License
Licensed under the BSD-3-Clause License.
