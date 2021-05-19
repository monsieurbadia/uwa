#[inline]
/// returns `true` if this char is a end of file
pub fn is_end_of_file(ascii: char) -> bool {
  ascii == '\u{0}'
}

#[inline]
/// returns `true` if this char is a newline
pub fn is_newline(ascii: char) -> bool {
  ascii == '\u{000A}'
}

#[inline]
/// returns `true` if this char is a number
pub fn is_number(ascii: char) -> bool {
  ascii.is_digit(10)
}

#[inline]
/// returns `true` if this char is a number zero
pub fn is_zero(ascii: char) -> bool {
  ascii == '0'
}

#[inline]
/// returns `true` if this char is an identifier
pub fn is_ident(ascii: char) -> bool {
  ascii.is_alphabetic() || is_underscore(ascii)
}

#[inline]
/// returns `true` if this char is a quote
pub fn is_quote(ascii: char) -> bool {
  is_double_quote(ascii) || is_single_quote(ascii)
}

#[inline]
/// returns `true` if this char is a double quote
pub fn is_double_quote(ascii: char) -> bool {
  ascii == '\u{0022}'
}

#[inline]
/// returns `true` if this char is a single quote
pub fn is_single_quote(ascii: char) -> bool {
  ascii == '\u{0027}'
}

#[inline]
/// returns `true` if this char is an underscore
pub fn is_underscore(ascii: char) -> bool {
  ascii == '\u{005F}'
}

#[inline]
/// returns `true` if this char is a whitespace
pub fn is_whitespace(ascii: char) -> bool {
  match ascii {
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // next line from latin1
    | '\u{200E}' // left-to-right mark
    | '\u{200F}' // right-to-left mark
    | '\u{2028}' // line seprarator
    | '\u{2029}' // paragraph seprarator
    => true,
    _ => false,
  }
}
