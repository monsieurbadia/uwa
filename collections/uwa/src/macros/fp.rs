pub macro and_then {
  ( $f:expr, $( $x:expr ),* ) => ( $f( $( $x ),* )),
  () => (),
}
