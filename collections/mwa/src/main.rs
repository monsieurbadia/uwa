fn main() {}

#[cfg(test)]
mod unit_test {
  use mwa::testing::{unit, test, must};

  unit!(
    test!(be_equal, must!(eq: 0, 0));
    test!(not_equal, must!(ne: 1, 0));
    test!(be_truthy, must!(truthy: true));
    test!(be_falsy, must!(falsy: false));
    test!(catch, must!(die: { panic!() } ));
  );
}
