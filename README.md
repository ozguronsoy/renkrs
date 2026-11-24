# renkrs

Provides structs for RGB, RGBA, HSL, HSV, and CMYK color models with zero-cost conversions between them.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
renkrs = "0.1.1"
```

or run the following command:
```bash
cargo add renkrs
```

## Usage

`renkrs` implements `From` conversions between each type, allowing one to seamlessly convert between color spaces using `into`. `renkrs` also implements `FromStr`, `LowerHex`, and `UpperHex` for `Rgb` and `Rgba`.

```rust
use renkrs::*;

let color_u8: Rgba<u8> = "#FF5733".parse().unwrap();
let color_f32: Rgb<f32> = color_u8.into();
let hsl: Hsl = color_u8.into();
let hsv: Hsv = color_f32.into();
let cmyk: Cmyk = hsv.into();
let hex_lower: String = format!("{:#x}", color_u8);
let hex_upper: String = format!("{:#X}", color_f32);
let channels: &[f32] = cmyk.as_slice();
```

## License
Licensed under the BSD-3-Clause License.
