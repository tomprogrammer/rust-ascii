This packages includes the Ascii types that were removed from the Rust
standard library by the 2014-12 [reform of the `std::ascii` module]
(https://github.com/rust-lang/rfcs/pull/486).

`AsciiChar` is a wrapper for `u8` that forces the value to be within the ASCII range (0x00 to 0x7F).
`AsciiString` and `AsciiStr` are naturally strings of text entirely within the ASCII range.

Most of `AsciiChar` and `AsciiStr` can be used without std by enabling the feature no_std.

[Documentation](https://tomprogrammer.github.io/rust-ascii/ascii/index.html)
