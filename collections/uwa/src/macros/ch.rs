use unitest::testing::{must, test, unit};

use utilcase::charcase::{
  endofcase, spacecase, quotecase, numbercase, identcase,
  underscorecase, lowercase, uppercase,
};

/// shorthands of the ascii module
pub macro is {
  ( eof $rhs:expr ) => ( endofcase::is_end_of_file($rhs) ),
  ( newline $rhs:expr ) => ( endofcase::is_end_of_line($rhs) ),
  ( whitespace $rhs:expr ) => ( spacecase::is_whitespace($rhs) ),
  ( quote $rhs:expr ) => ( quotecase::is_quote($rhs) ),
  ( quote_single $rhs:expr ) => ( quotecase::is_quote_single($rhs) ),
  ( quote_double $rhs:expr ) => ( quotecase::is_quote_double($rhs) ),
  ( number $rhs:expr ) => ( numbercase::is_number($rhs) ),
  ( number_zero $rhs:expr ) => ( numbercase::is_number_zero($rhs) ),
  ( number_continue $rhs:expr ) => ( numbercase::is_number_continue($rhs) ),
  ( number_hex $rhs:expr ) => ( numbercase::is_number_hex($rhs) ),
  ( ident $rhs:expr ) => ( identcase::is_ident($rhs) ),
  ( underscore $rhs:expr ) => ( underscorecase::is_underscore($rhs) ),
  ( lowercase $rhs:expr ) => ( lowercase::is_lowercase($rhs) ),
  ( uppercase $rhs:expr ) => ( uppercase::is_uppercase($rhs) ),
  () => (),
}

unit!(
  test!(returs_true_if_eof, must!(truthy: is!(eof '\0')));
  test!(returs_true_if_newline, must!(truthy: is!(newline '\n')));
  test!(returs_true_if_quote_sinlge, must!(truthy: is!(quote '"')));
  test!(returs_true_if_quote_double, must!(truthy: is!(quote '\'')));
  test!(returs_true_if_single_quote, must!(truthy: is!(quote_single '\'')));
  test!(returs_true_if_double_quote, must!(truthy: is!(quote_double '"')));
  test!(returs_true_if_number, must!(truthy: is!(number '3')));
  test!(returs_true_if_number_zero, must!(truthy: is!(number_zero '0')));
  test!(returs_true_if_number_continue, must!(truthy: is!(number_continue '8')));
  test!(returs_true_if_number_hex, must!(truthy: is!(number_hex 'f')));
  test!(returs_true_if_ident, must!(truthy: is!(ident 'a')));
  test!(returs_true_if_underscore, must!(truthy: is!(underscore '_')));
  test!(returs_true_if_lowercase, must!(truthy: is!(lowercase 'a')));
  test!(returs_true_if_uppercase, must!(truthy: is!(uppercase 'A')));
);
