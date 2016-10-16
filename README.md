# ascii

A library that provides ASCII-only string and character types, equivalent to the
`char`, `str` and `String` types in the standard library.

Types and conversion traits are described in the
[Documentation](https://tomprogrammer.github.io/rust-ascii/ascii/index.html).

You can include this crate in your cargo project by adding it to the
dependencies section in `Cargo.toml`:
```toml
[dependencies]
ascii = "0.8"
```

# Using ascii without libstd

Most of `AsciiChar` and `AsciiStr` can be used without `std` by disabling the
default features. The owned string type `AsciiString` and the conversion trait
`IntoAsciiString` as well as all methods referring to these types are
unavailable. Because libcore doesn't have `AsciiExt` and `Error`, most of their
methods are implemented directly:
* `Ascii{Char,Str}::eq_ignore_ascii_case()`
* `AsciiChar::to_ascii_{upper,lower}case()`
* `AsciiStr::make_ascii_{upper,lower}case()`
* `{ToAsciiChar,AsAsciiStr}Error::description()`

To use the `ascii` crate in `core`-only mode in your cargo project just add the
following dependency declaration in `Cargo.toml`:
```toml
[dependencies]
ascii = { version = "0.8", default-features = false }
```

# Requirements

The `ascii` library requires rustc 1.9.0 or greater, due to
the [stabilization of `AsciiExt`](https://github.com/rust-lang/rust/pull/32804).
Using only `core` instead of `std` in your project lowers this requirement to
rustc 1.6.0 or greater.

# History

This package included the Ascii types that were removed from the Rust standard
library by the 2014-12 [reform of the `std::ascii` module]
(https://github.com/rust-lang/rfcs/pull/486). The API changed significantly
since then.
