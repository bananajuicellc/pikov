enum AppState {
  MainMenu,  // add struct with selections (new, load)
  LoadMenu,
  NewMenu,  // slot number, name?
  CalibrationScreen,  // forward/back/color/channel/checker color 1/checker color 2.
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
