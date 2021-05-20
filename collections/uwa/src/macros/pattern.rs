use unitest::testing::{must, test, unit};

/// returns true if matched
pub macro matches {
  ( $e:expr , $( $p:tt )+ ) => ( match $e { $($p)+ => true, _ => false } ),
}

/// unwraps an option
pub macro unwrap_or {
  ( return : $e:expr , $r:expr ) => (
    match $e { Some(e) => e, None => return $r, }
  ),
  ( die : $e:expr ) => (
    match $e { Some(e) => e, None => return panic!() }
  ),
}

/// this returns a result
fn ok() -> Result<(), String> {
  Ok(())
}

/// this returns an error
fn err() -> Result<(), String> {
  Err(format!(""))
}

/// this unwraps an option or returns 
pub macro result_or {
  ( die: $e:expr ) => (
    match $e {
      Ok(v) => v,
      Err(e) => panic!("{} - {}", stringify!($e), e),
    }
  ),
  () => {}
}

unit!(
  test!(
    should_be_ok,
    must!(eq: result_or!(die: ok()), ())
  );
  test!(
    should_be_err,
    must!(die: result_or!(die: err()))
  );
);

unit!(
  test!(
    returns_matched_value,
    must!(
      truthy:
      matches!(Some("-12"), Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'..= b'9')
      )
    )
  );
);
