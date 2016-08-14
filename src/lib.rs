// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library that provides ASCII-only string and character types, equivalent to the `char`, `str` and
//! `String` types in the standard library.
//!
#![cfg_attr(not(feature = "no_std"), doc="[The documentation for `#[no_std]` mode is here](https://tomprogrammer.github.io/rust-ascii/no_std/ascii/index.html).")]
#![cfg_attr(feature = "no_std", doc="This is the documentation for `#[no_std]` mode.")]
//!
//! # Requirements
//!
//! The `ascii` library requires rustc 1.9.0 or greater, due to the [stabilization of
//! `AsciiExt`](https://github.com/rust-lang/rust/pull/32804). Using the `no_std` feature lowers
//! this requirement to rustc 1.6.0 or greater.
//!
//! # History
//!
//! This packages included the Ascii types that were removed from the Rust standard library by the
//! 2014-12 [reform of the `std::ascii` module](https://github.com/rust-lang/rfcs/pull/486). The
//! API changed significantly since then.

#![cfg_attr(feature = "no_std", no_std)]

mod free_functions;
mod ascii_char;
mod ascii_str;
#[cfg(not(feature = "no_std"))]
mod ascii_string;

pub use free_functions::{caret_encode, caret_decode};
pub use ascii_char::{AsciiChar, ToAsciiChar, ToAsciiCharError};
pub use ascii_str::{AsciiStr, AsAsciiStr, AsMutAsciiStr, AsAsciiStrError};
#[cfg(not(feature = "no_std"))]
pub use ascii_string::{AsciiString, IntoAsciiString, FromAsciiError};
