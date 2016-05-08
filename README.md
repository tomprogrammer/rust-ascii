This packages includes the `Ascii` type and related functionality
that was removed from the Rust standard library by the 2014-12
[reform of the `std::ascii` module](https://github.com/rust-lang/rfcs/pull/486).

`Ascii` is a wrapper for `u8` that forces the value to be within the ASCII range (0x00 to 0x7F).
`AsciiString` and `AsciiStr` are naturally strings of text entirely within the ASCII range.

[Documentation](https://tomprogrammer.github.io/rust-ascii/ascii/index.html)
