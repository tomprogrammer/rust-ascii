This packages includes the `Ascii` type and related functionality
that were removed from the Rust standard library on the 2014-12
[reform of the `std::ascii` module](https://github.com/rust-lang/rfcs/pull/486).

`Ascii` is a wrapper for `u8` force the value to be within the ASCII range (0x00 to 0x7F).
`Vec<Ascii>` and `[Ascii]` are naturally strings of text entirely within the ASCII range.
