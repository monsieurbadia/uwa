use unitest::testing::{must, test, unit};

/// this represents the string structure
pub macro strbuf {
  () => ( String::new() ),
  ( $value:expr ) => ( String::from($value) ),
  ( $( $x:expr ),* ) => ({
    use std::fmt::Write;

    let mut s = String::new();

    $( let _ = std::write!(s, "{}", $x ); )*

    s
  }),
  ( $s:expr , $( $x:expr ),* ) => ({
    use std::fmt::Write;

    let mut s = String::new();
    let _ = std::write!(s, $s, $( $x ),*);

    s
  }),
}

unit!(
  test!(returns_empty_string_1, must!(eq: strbuf![""], String::new()));
  test!(returns_empty_string_2, must!(eq: strbuf![""], ""));
  test!(returns_empty_string_3, must!(eq: strbuf![""], format!("")));
  test!(returns_yo_1, must!(eq: strbuf!["yo!"], String::from("yo!")));
  test!(returns_yo_2, must!(eq: strbuf!["yo!"], "yo!"));
  test!(returns_yo_3, must!(eq: strbuf!["yo!"], format!("yo!")));
  test!(returns_abc_1, must!(eq: strbuf!["a", "b", "c"], String::from("abc")));
  test!(returns_abc_2, must!(eq: strbuf!["a", "b", "c"], "abc"));
  test!(returns_abc_3, must!(eq: strbuf!["a", "b", "c"], format!("abc")));
);
