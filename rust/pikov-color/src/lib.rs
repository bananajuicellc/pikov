use fixed::FixedU8;
use fixed::types::extra::U7;

enum Color {
  Grayscale,
  Red,
  Green,
  Blue,
}

struct Calibration {
  target: FixedU8<U7>,
  color: Color,
  // 5-bit channel for GBA.
  selection: u8,
}

enum AppState {
  MainMenu,  // add struct with selections (new, load)
  LoadMenu,
  NewMenu,  // slot number, name?
  CalibrationScreen,  // forward/back/color/channel/checker color 1/checker color 2.
}

const INVERSE_OETF_LINEAR_THRESHOLD: FixedU8<U7> = match FixedU8::from_ascii(b"0.0031308") {
  Ok(val) => val,
  Err(_) => panic!("hardcoded threshold should be valid"),
};

/// Inverse opto-electronic transfer function (OETF)
/// maps linear light intensity to sRGB.
///
/// See: https://en.wikipedia.org/wiki/SRGB
pub const fn linear_to_srgb(intensity: FixedU8<U7>) -> FixedU8<U7> {
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
