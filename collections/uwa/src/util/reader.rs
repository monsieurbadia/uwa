use std::fs;
use std::io;
use std::path::Path;

use crate::macros::string::strbuf;

/// reads a file
#[inline]
pub fn readfile(path: &Path) -> Result<String, String> {
  match fs::read_to_string(path) {
    Ok(file) => Ok(file),
    Err(e) => Err(format!("{}", e)),
  }
}

/// reads line
#[inline]
pub fn readline<'a>(icon: &str) -> Result<String, String> {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut buf = strbuf![];

  print!("\n{} ", icon);

  io::Write::flush(&mut stdout).expect("flush failed!");
  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  buf.truncate(buf.trim_end().len());

  Ok(buf)
}
