# ascii

A library that provides ASCII-only string and character types, equivalent to the `char`, `str` and
`String` types in the standard library.

Types and conversion traits are described in the
[Documentation](https://tomprogrammer.github.io/rust-ascii/ascii/index.html)

# Using ascii without libstd

Most of `AsciiChar` and `AsciiStr` can be used without `std` by enabling the feature `no_std`. The
owned string type `AsciiString` and the conversion trait `IntoAsciiString` as well as all methods
referring to these types aren't available without `std`.

# Requirements

The `ascii` library requires rustc 1.9.0 or greater, due to the
[stabilization of `AsciiExt`](https://github.com/rust-lang/rust/pull/32804). Using the `no_std`
feature lowers this requirement to rustc 1.6.0 or greater.

# History

This packages included the Ascii types that were removed from the Rust standard library by the
2014-12 [reform of the `std::ascii` module] (https://github.com/rust-lang/rfcs/pull/486). The API
changed significantly since then.
