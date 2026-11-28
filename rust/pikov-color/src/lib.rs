use fixed::FixedU8;
use fixed::types::extra::U8;

enum Color {
  Grayscale,
  Red,
  Green,
  Blue,
}

const INVERSE_OETF_LINEAR_THRESHOLD: FixedU8<U8> = match FixedU8::from_ascii(b"0.0031308") {
  Ok(val) => val,
  Err(_) => panic!("hardcoded threshold should be valid"),
};

/// Inverse opto-electronic transfer function (OETF)
/// maps linear light intensity to sRGB.
///
/// See: https://en.wikipedia.org/wiki/SRGB
pub const fn linear_to_srgb(intensity: FixedU8<U8>) -> FixedU8<U8> {
    intensity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = linear_to_srgb(INVERSE_OETF_LINEAR_THRESHOLD);
        assert_eq!(result, 4);
    }
}
