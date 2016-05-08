// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ignore-lexer-test FIXME #15679

//! Operations on ASCII strings and characters

mod ascii;
mod ascii_string;
mod ascii_str;

use std::borrow::Borrow;
use std::ascii::AsciiExt;

pub use ascii::{Ascii, IntoAscii, IntoAsciiError};
pub use ascii_string::{AsciiString, IntoAsciiString};
pub use ascii_str::AsciiStr;

/// Trait for converting into an ascii type.
pub trait AsciiCast<'a>: AsciiExt {
    type Target;

    /// Convert to an ascii type, return Err(()) on non-ASCII input.
    #[inline]
    fn to_ascii(&'a self) -> Result<Self::Target, ()> {
        if self.is_ascii() {
            Ok(unsafe { self.to_ascii_nocheck() })
        } else {
            Err(())
        }
    }

    /// Convert to an ascii type, not doing any range asserts
    unsafe fn to_ascii_nocheck(&'a self) -> Self::Target;
}

/// Trait for copyless casting to an ascii vector.
pub trait OwnedAsciiCast<T: ?Sized> : Sized
    where Self: Borrow<T>,
          T: AsciiExt<Owned=Self>
{
    /// Take ownership and cast to an ascii vector. On non-ASCII input return ownership of data
    /// that was attempted to cast to ascii in `Err(Self)`.
    #[inline]
    fn into_ascii(self) -> Result<AsciiString, Self> {
        if self.borrow().is_ascii() {
            Ok(unsafe { self.into_ascii_nocheck() })
        } else {
            Err(self)
        }
    }

    /// Take ownership and cast to an ascii vector.
    /// Does not perform validation checks.
    unsafe fn into_ascii_nocheck(self) -> AsciiString;
}

impl OwnedAsciiCast<str> for String {
    #[inline]
    unsafe fn into_ascii_nocheck(self) -> AsciiString {
        self.into_bytes().into_ascii_nocheck()
    }
}

impl OwnedAsciiCast<[u8]> for Vec<u8> {
    #[inline]
    unsafe fn into_ascii_nocheck(self) -> AsciiString {
        AsciiString::from_bytes_unchecked(self)
    }
}
