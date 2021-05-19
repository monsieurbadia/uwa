/// this represents the string structure
pub macro string {
  () => ( String::new() ),
  ( $value:expr ) => ( String::from($value) ),
  ( $( $x:expr , )* ) => ({
    let buf = String::new();

    $( write!(buf, "{}", $x) );*

    buf
  }),
  ( $s:expr , $( $x:expr , )* ) => ({
    let buf = String::new();

    write!(buf, $s, $( $x )*);

    buf
  }),
  ( $s:expr , $( $x:expr , )* ) => ( format!($s, $( $x:expr , )*) ),
}
