/// shorthands
pub macro is {
  ( eof $rhs:expr ) => ( crate::ascii::is_end_of_file($rhs) ),
  ( newline $rhs:expr ) => ( crate::ascii::is_newline($rhs) ),
  ( quote $rhs:expr ) => ( crate::ascii::is_quote($rhs) ),
  ( single_quote $rhs:expr ) => ( crate::ascii::is_single_quote($rhs) ),
  ( double_quote $rhs:expr ) => ( crate::ascii::is_double_quote($rhs) ),
  ( number $rhs:expr ) => ( crate::ascii::is_number($rhs) ),
  ( zero $rhs:expr ) => ( crate::ascii::is_zero($rhs) ),
  ( ident $rhs:expr ) => ( crate::ascii::is_ident($rhs) ),
  ( underscore $rhs:expr ) => ( crate::ascii::is_underscore($rhs) ),
  () => ( std::io::print!("\nshorthand needed")),
}
