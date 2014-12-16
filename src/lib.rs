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

#![feature(macro_rules, globs, default_type_params)]
#![unstable = "unsure about placement and naming"]
#![allow(deprecated)]

//use std::kinds::Sized;
use std::fmt;
use std::mem;
use std::borrow::BorrowFrom;
use std::ascii::AsciiExt;


/// Datatype to hold one ascii character. It wraps a `u8`, with the highest bit always zero.
#[deriving(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Copy)]
pub struct Ascii { chr: u8 }

impl Ascii {
    /// Converts an ascii character into a `u8`.
    #[inline]
    #[unstable = "recently renamed"]
    pub fn as_byte(&self) -> u8 {
        self.chr
    }

    /// Converts an ascii character into a `char`.
    #[inline]
    #[unstable = "recently renamed"]
    pub fn as_char(&self) -> char {
        self.chr as char
    }

    /// Convert to lowercase.
    #[inline]
    #[stable]
    pub fn to_lowercase(&self) -> Ascii {
        Ascii{chr: self.chr.to_ascii_lowercase()}
    }

    /// Convert to uppercase.
    #[inline]
    #[stable]
    pub fn to_uppercase(&self) -> Ascii {
        Ascii{chr: self.chr.to_ascii_uppercase()}
    }

    // the following methods are like ctype, and the implementation is inspired by musl

    /// Check if the character is a letter (a-z, A-Z)
    #[inline]
    #[stable]
    pub fn is_alphabetic(&self) -> bool {
        (self.chr >= 0x41 && self.chr <= 0x5A) || (self.chr >= 0x61 && self.chr <= 0x7A)
    }

    /// Check if the character is a number (0-9)
    #[inline]
    #[unstable = "may be renamed"]
    pub fn is_digit(&self) -> bool {
        self.chr >= 0x30 && self.chr <= 0x39
    }

    /// Check if the character is a letter or number
    #[inline]
    #[stable]
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Check if the character is a space or horizontal tab
    #[inline]
    #[experimental = "likely to be removed"]
    pub fn is_blank(&self) -> bool {
        self.chr == b' ' || self.chr == b'\t'
    }

    /// Check if the character is a control character
    #[inline]
    #[stable]
    pub fn is_control(&self) -> bool {
        self.chr < 0x20 || self.chr == 0x7F
    }

    /// Checks if the character is printable (except space)
    #[inline]
    #[experimental = "unsure about naming, or whether this is needed"]
    pub fn is_graph(&self) -> bool {
        (self.chr - 0x21) < 0x5E
    }

    /// Checks if the character is printable (including space)
    #[inline]
    #[unstable = "unsure about naming"]
    pub fn is_print(&self) -> bool {
        (self.chr - 0x20) < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    #[inline]
    #[stable]
    pub fn is_lowercase(&self) -> bool {
        (self.chr - b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    #[inline]
    #[stable]
    pub fn is_uppercase(&self) -> bool {
        (self.chr - b'A') < 26
    }

    /// Checks if the character is punctuation
    #[inline]
    #[stable]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    #[inline]
    #[stable]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || ((self.chr | 32u8) - b'a') < 6
    }
}

impl<'a> fmt::Show for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.chr as char).fmt(f)
    }
}

/// Trait for converting into an ascii type.
#[experimental = "may be replaced by generic conversion traits"]
pub trait AsciiCast<T, U = Self> for Sized?: AsciiExt<U> {
    /// Convert to an ascii type, panic on non-ASCII input.
    #[inline]
    fn to_ascii(&self) -> T {
        assert!(self.is_ascii());
        unsafe {self.to_ascii_nocheck()}
    }

    /// Convert to an ascii type, return None on non-ASCII input.
    #[inline]
    fn to_ascii_opt(&self) -> Option<T> {
        if self.is_ascii() {
            Some(unsafe { self.to_ascii_nocheck() })
        } else {
            None
        }
    }

    /// Convert to an ascii type, not doing any range asserts
    unsafe fn to_ascii_nocheck(&self) -> T;
}

#[experimental = "may be replaced by generic conversion traits"]
impl<'a> AsciiCast<&'a[Ascii], Vec<u8>> for [u8] {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> &'a[Ascii] {
        mem::transmute(self)
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl<'a> AsciiCast<&'a [Ascii], String> for str {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> &'a [Ascii] {
        mem::transmute(self)
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiCast<Ascii> for u8 {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self }
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiCast<Ascii> for char {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self as u8 }
    }
}

/// Trait for copyless casting to an ascii vector.
#[experimental = "may be replaced by generic conversion traits"]
pub trait OwnedAsciiCast<Sized? T, U = Self>
where T: BorrowFrom<Self> + AsciiExt<U> {
    /// Take ownership and cast to an ascii vector.
    /// # Panics
    ///
    /// Panic on non-ASCII input.
    #[inline]
    fn into_ascii(self) -> Vec<Ascii> {
        assert!({
            let borrowed: &T = BorrowFrom::borrow_from(&self);
            borrowed.is_ascii()
        });
        unsafe {self.into_ascii_nocheck()}
    }

    /// Take ownership and cast to an ascii vector. Return None on non-ASCII input.
    #[inline]
    fn into_ascii_opt(self) -> Option<Vec<Ascii>> {
        if {
            let borrowed: &T = BorrowFrom::borrow_from(&self);
            borrowed.is_ascii()
        } {
            Some(unsafe { self.into_ascii_nocheck() })
        } else {
            None
        }
    }

    /// Take ownership and cast to an ascii vector.
    /// Does not perform validation checks.
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii>;
}

#[experimental = "may be replaced by generic conversion traits"]
impl OwnedAsciiCast<str> for String {
    #[inline]
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii> {
        self.into_bytes().into_ascii_nocheck()
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl OwnedAsciiCast<[u8]> for Vec<u8> {
    #[inline]
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii> {
        let v = Vec::from_raw_parts(self.as_ptr() as *mut Ascii,
                                    self.len(),
                                    self.capacity());

        // We forget `self` to avoid freeing it at the end of the scope
        // Otherwise, the returned `Vec` would point to freed memory
        mem::forget(self);
        v
    }
}

/// Trait for converting an ascii type to a string. Needed to convert
/// `&[Ascii]` to `&str`.
#[experimental = "may be replaced by generic conversion traits"]
pub trait AsciiStr for Sized? {
    /// Convert to a string.
    fn as_str<'a>(&'a self) -> &'a str;

    /// Convert to bytes.
    fn as_bytes<'a>(&'a self) -> &'a [u8];
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiStr for [Ascii] {
    #[inline]
    fn as_str<'a>(&'a self) -> &'a str {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe { mem::transmute(self) }
    }
}

impl IntoString for Vec<Ascii> {
    #[inline]
    fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_bytes()) }
    }
}

/// Trait to convert to an owned byte vector by consuming self
#[experimental = "may be replaced by generic conversion traits"]
pub trait IntoBytes {
    /// Converts to an owned byte vector by consuming self
    fn into_bytes(self) -> Vec<u8>;
}

#[experimental = "may be replaced by generic conversion traits"]
impl IntoBytes for Vec<Ascii> {
    fn into_bytes(self) -> Vec<u8> {
        unsafe {
            let v = Vec::from_raw_parts(self.as_ptr() as *mut u8,
                                        self.len(),
                                        self.capacity());

            // We forget `self` to avoid freeing it at the end of the scope
            // Otherwise, the returned `Vec` would point to freed memory
            mem::forget(self);
            v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! v2ascii (
        ( [$($e:expr),*]) => (&[$(Ascii{chr:$e}),*]);
        (&[$($e:expr),*]) => (&[$(Ascii{chr:$e}),*]);
    )

    macro_rules! vec2ascii (
        ($($e:expr),*) => ([$(Ascii{chr:$e}),*].to_vec());
    )

    #[test]
    fn test_ascii() {
        assert_eq!(65u8.to_ascii().as_byte(), 65u8);
        assert_eq!(65u8.to_ascii().as_char(), 'A');
        assert_eq!('A'.to_ascii().as_char(), 'A');
        assert_eq!('A'.to_ascii().as_byte(), 65u8);

        assert!('0'.to_ascii().is_digit());
        assert!('9'.to_ascii().is_digit());
        assert!(!'/'.to_ascii().is_digit());
        assert!(!':'.to_ascii().is_digit());

        assert!((0x1fu8).to_ascii().is_control());
        assert!(!' '.to_ascii().is_control());
        assert!((0x7fu8).to_ascii().is_control());
    }

    #[test]
    fn test_ascii_vec() {
        let test = &[40u8, 32u8, 59u8];
        let b: &[_] = v2ascii!([40, 32, 59]);
        assert_eq!(test.to_ascii(), b);
        assert_eq!("( ;".to_ascii(), b);
        let v = vec![40u8, 32u8, 59u8];
        assert_eq!(v.as_slice().to_ascii(), b);
        assert_eq!("( ;".to_string().as_slice().to_ascii(), b);
    }

    #[test]
    fn test_owned_ascii_vec() {
        assert_eq!(("( ;".to_string()).into_ascii(), vec2ascii![40, 32, 59]);
        assert_eq!((vec![40u8, 32u8, 59u8]).into_ascii(), vec2ascii![40, 32, 59]);
    }

    #[test]
    fn test_ascii_as_str() {
        let v = v2ascii!([40, 32, 59]);
        assert_eq!(v.as_str(), "( ;");
    }

    #[test]
    fn test_ascii_as_bytes() {
        let v = v2ascii!([40, 32, 59]);
        assert_eq!(v.as_bytes(), b"( ;");
    }

    #[test]
    fn test_ascii_into_string() {
        assert_eq!(vec2ascii![40, 32, 59].into_string(), "( ;".to_string());
        assert_eq!(vec2ascii!(40, 32, 59).into_string(), "( ;".to_string());
    }

    #[test]
    fn test_ascii_to_bytes() {
        assert_eq!(vec2ascii![40, 32, 59].into_bytes(), vec![40u8, 32u8, 59u8]);
    }

    #[test] #[should_fail]
    fn test_ascii_vec_panic_u8_slice()  { (&[127u8, 128u8, 255u8]).to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_vec_panic_str_slice() { "zoä华".to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_panic_u8_slice() { 255u8.to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_panic_char_slice() { 'λ'.to_ascii(); }

    #[test]
    fn test_opt() {
        assert_eq!(65u8.to_ascii_opt(), Some(Ascii { chr: 65u8 }));
        assert_eq!(255u8.to_ascii_opt(), None);

        assert_eq!('A'.to_ascii_opt(), Some(Ascii { chr: 65u8 }));
        assert_eq!('λ'.to_ascii_opt(), None);

        assert_eq!("zoä华".to_ascii_opt(), None);

        let test1 = &[127u8, 128u8, 255u8];
        assert_eq!((test1).to_ascii_opt(), None);

        let v = [40u8, 32u8, 59u8];
        let v2: &[_] = v2ascii!(&[40, 32, 59]);
        assert_eq!(v.to_ascii_opt(), Some(v2));
        let v = [127u8, 128u8, 255u8];
        assert_eq!(v.to_ascii_opt(), None);

        let v = "( ;";
        assert_eq!(v.to_ascii_opt(), Some(v2));
        assert_eq!("zoä华".to_ascii_opt(), None);

        assert_eq!((vec![40u8, 32u8, 59u8]).into_ascii_opt(), Some(vec2ascii![40, 32, 59]));
        assert_eq!((vec![127u8, 128u8, 255u8]).into_ascii_opt(), None);

        assert_eq!(("( ;".to_string()).into_ascii_opt(), Some(vec2ascii![40, 32, 59]));
        assert_eq!(("zoä华".to_string()).into_ascii_opt(), None);
    }

    #[test]
    fn test_to_string() {
        let s = Ascii{ chr: b't' }.to_string();
        assert_eq!(s, "t".to_string());
    }

    #[test]
    fn test_show() {
        let c = Ascii { chr: b't' };
        assert_eq!(format!("{}", c), "t".to_string());
    }
}
