# U.W.A

> *utils wit attitudes*

## About

a collection of simple macros that I use in my compilers.

## Collections

**`use` `uwa::ch`**

**is**

```rust
match c {
  c if is!(eof c),
  c if is!(newline c),
  c if is!(quote c),
  c if is!(quote c),
  c if is!(single_quote c),
  c if is!(double_quote c),
  c if is!(number c),
  c if is!(zero c),
  c if is!(ident c),
  c if is!(underscore c),
  c => print!("\nunknown char"),
}
```

**`use` `uwa::file`**

**read ~file**

```rust
match read!(file "Cargo.toml") {
  Ok(f) => print!("\n{}", f),
  Err(e) => print!("\n", e),
}
```

**read ~line**

```rust
loop {
  match read!(line "Cargo.toml") {
    Ok(line) => print!("\n{}", line),
    Err(e) => print!("\n", e),
  }
}
```

**`use` `uwa::result`**

**result_or ~die**

```rust
fn parse(t: Token) {
  result_or!(die Ok(t))
}
```
