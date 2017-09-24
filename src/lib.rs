// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library that provides ASCII-only string and character types, equivalent to the `char`, `str`
//! and `String` types in the standard library.
//!
#![cfg_attr(feature = "std",
           doc = "[The documentation for the `core` mode is here](https://tomprogrammer.github.io/rust-ascii/core/ascii/index.html).")]
#![cfg_attr(not(feature = "std"), doc = "This is the documentation for `core` mode.")]
//! Please refer to the readme file to learn about the different feature modes of this crate.
//!
//! # Requirements
//!
//! The `ascii` library requires rustc 1.9.0 or greater, due to the [stabilization of
//! `AsciiExt`](https://github.com/rust-lang/rust/pull/32804). Using only `core` instead of `std` in
//! your project lowers this requirement to rustc 1.6.0 or greater.
//!
//! # History
//!
//! This package included the Ascii types that were removed from the Rust standard library by the
//! 2014-12 [reform of the `std::ascii` module](https://github.com/rust-lang/rfcs/pull/486). The
//! API changed significantly since then.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "std")]
extern crate libc;
extern crate memchr;
#[cfg(feature = "quickcheck")]
extern crate quickcheck;

mod free_functions;
mod ascii_char;
mod ascii_str;
#[cfg(feature = "std")]
mod ascii_string;

#[cfg(feature = "std")]
pub mod ffi;

pub use free_functions::{caret_decode, caret_encode};
pub use ascii_char::{AsciiChar, ToAsciiChar, ToAsciiCharError};
pub use ascii_str::{AsAsciiStr, AsAsciiStrError, AsMutAsciiStr, AsciiStr, Chars, CharsMut, Lines};
#[cfg(feature = "std")]
pub use ascii_string::{AsciiString, FromAsciiError, IntoAsciiString};
