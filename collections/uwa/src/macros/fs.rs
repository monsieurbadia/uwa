use unitest::testing::{must, test, unit};

/// this represents the Path structure
pub macro path {
  ( rel: $e:expr ) => ( std::path::Path::new($e) ),
  ( abs: $e:expr ) => (
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join($e)
  ),
  () => (),
}

/// shorthands of the reader module
pub macro read {
  ( file $p:expr ) => ({
    let path = path!(abs: "Cargo.toml");
    let f = crate::util::reader::readfile(&path);

    match f {
      Ok(f) => f,
      Err(e) => panic!("{}", e),
    }
  }),
  ( line ) => ({
    let f = crate::util::reader::readline("");

    match f {
      Ok(f) => f,
      Err(e) => panic!("{}", e),
    }
  }),
  ( line , $prompt:expr ) => ({
    let path = path!(abs: "Cargo.toml");
    let f = crate::util::reader::readline(&path);

    match f {
      Ok(f) => f,
      Err(e) => panic!("{}", e),
    }
  }),
  () => (),
}

unit!(
  test!(
    returns_absolute_path,
    must!(
      eq:
      path!(abs: "../Cargo.toml"),
      std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../Cargo.toml")
    )
  );
  test!(
    returns_relative_path,
    must!(eq: path!(rel: "../Cargo.toml"), std::path::Path::new("../Cargo.toml"))
  );
);

unit!(
  test!(
    read_a_file,
    must!(eq: read!(file "Cargo.toml"), include_str!("../../Cargo.toml"))
  );
);
