fn main() {}

#[cfg(test)]
mod unit_test {
  use mwa::testing::{unit, test, must};

  unit!(
    test!(be_equal, must!(eq: 0, 0));
  );
}
