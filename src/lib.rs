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

#![cfg_attr(feature = "unstable", feature(ascii,std_misc))]

use std::{fmt, mem, ops};
use std::borrow::{Borrow, ToOwned};
use std::ops::{Deref, DerefMut, Add, Index, IndexMut};
use std::cmp::{Ord, Ordering};
use std::str::FromStr;
use std::ascii::AsciiExt;
use std::iter::FromIterator;
#[cfg(feature = "unstable")]
use std::ascii::OwnedAsciiExt;

/// Datatype to hold one ascii character. It wraps a `u8`, with the highest bit always zero.
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Copy)]
pub struct Ascii { chr: u8 }

impl Ascii {
    /// Constructs an Ascii character from a `char`.
    ///
    /// # Failure
    ///
    /// Returns `Err(())` if the character can't be ascii encoded.
    ///
    /// # Example
    /// ```
    /// # use ascii::Ascii;
    /// let a = Ascii::from('g').unwrap();
    /// assert_eq!(a.as_char(), 'g');
    /// ```
    #[inline]
    pub fn from(ch: char) -> Result<Ascii, ()> {
        if ch as u32 <= 0x7F {
            return Ok( Ascii { chr: ch as u8 });
        }
        Err(())
    }
    /// Converts an ascii character into a `u8`.
    #[inline]
    pub fn as_byte(&self) -> u8 {
        self.chr
    }

    /// Converts an ascii character into a `char`.
    #[inline]
    pub fn as_char(&self) -> char {
        self.chr as char
    }

    // the following methods are like ctype, and the implementation is inspired by musl

    /// Check if the character is a letter (a-z, A-Z)
    #[inline]
    pub fn is_alphabetic(&self) -> bool {
        (self.chr >= 0x41 && self.chr <= 0x5A) || (self.chr >= 0x61 && self.chr <= 0x7A)
    }

    /// Check if the character is a number (0-9)
    #[inline]
    pub fn is_digit(&self) -> bool {
        self.chr >= 0x30 && self.chr <= 0x39
    }

    /// Check if the character is a letter or number
    #[inline]
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Check if the character is a space or horizontal tab
    #[inline]
    pub fn is_blank(&self) -> bool {
        self.chr == b' ' || self.chr == b'\t'
    }

    /// Check if the character is a control character
    #[inline]
    pub fn is_control(&self) -> bool {
        self.chr < 0x20 || self.chr == 0x7F
    }

    /// Checks if the character is printable (except space)
    #[inline]
    pub fn is_graph(&self) -> bool {
        (self.chr - 0x21) < 0x5E
    }

    /// Checks if the character is printable (including space)
    #[inline]
    pub fn is_print(&self) -> bool {
        (self.chr - 0x20) < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    #[inline]
    pub fn is_lowercase(&self) -> bool {
        (self.chr - b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    #[inline]
    pub fn is_uppercase(&self) -> bool {
        (self.chr - b'A') < 26
    }

    /// Checks if the character is punctuation
    #[inline]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    #[inline]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || ((self.chr | 32u8) - b'a') < 6
    }
}

impl fmt::Display for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
    }
}

impl fmt::Debug for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
     }
}

/// A growable string stored as an ascii encoded buffer.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiString {
    vec: Vec<Ascii>,
}

impl AsciiString {
    /// Creates a new ascii string buffer initialized with the empty ascii string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::new();
    /// ```
    #[inline]
    pub fn new() -> AsciiString {
        AsciiString { vec: Vec::new() }
    }

    /// Creates a new ascii string buffer with the given capacity. The string will be able to hold
    /// exactly `capacity` bytes without reallocating. If `capacity` is 0, the ascii string will not
    /// allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> AsciiString {
        AsciiString {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Pushes the given ascii string onto this ascii string buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::{AsciiString, AsciiStr};
    /// use std::str::FromStr;
    /// let mut s = AsciiString::from_str("foo").unwrap();
    /// s.push_str(AsciiStr::from_str("bar").unwrap());
    /// assert_eq!(s, AsciiStr::from_str("foobar").unwrap());
    /// ```
    #[inline]
    pub fn push_str(&mut self, string: &AsciiStr) {
        self.vec.extend(string.as_slice().iter().cloned())
    }

    /// Returns the number of bytes that this ascii string buffer can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let s = String::with_capacity(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Reserves capacity for at least `additional` more bytes to be inserted in the given
    /// `AsciiString`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::new();
    /// s.reserve(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more bytes to be inserted in the
    /// given `AsciiString`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer `reserve` if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::new();
    /// s.reserve_exact(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional)
    }

    /// Shrinks the capacity of this ascii string buffer to match it's length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// use std::str::FromStr;
    /// let mut s = AsciiString::from_str("foo").unwrap();
    /// s.reserve(100);
    /// assert!(s.capacity() >= 100);
    /// s.shrink_to_fit();
    /// assert_eq!(s.capacity(), 3);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }

    /// Adds the given ascii character to the end of the ascii string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::{ Ascii, AsciiString};
    /// let mut s = AsciiString::from_bytes("abc").unwrap();
    /// s.push(Ascii::from('1').unwrap());
    /// s.push(Ascii::from('2').unwrap());
    /// s.push(Ascii::from('3').unwrap());
    /// assert_eq!(s, "abc123");
    /// ```
    #[inline]
    pub fn push(&mut self, ch: Ascii) {
        self.vec.push(ch)
    }

    /// Shortens a ascii string to the specified length.
    ///
    /// # Panics
    ///
    /// Panics if `new_len` > current length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_bytes("hello").unwrap();
    /// s.truncate(2);
    /// assert_eq!(s, "he");
    /// ```
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.vec.truncate(new_len)
    }

    /// Removes the last character from the ascii string buffer and returns it. Returns `None` if
    /// this string buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_bytes("foo").unwrap();
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('o'));
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('o'));
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('f'));
    /// assert_eq!(s.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<Ascii> {
        self.vec.pop()
    }

    /// Removed the ascii character from the string buffer at bytes position `idx` and returns it.
    ///
    /// # Warning
    ///
    /// This is an O(n) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// If `idx` is out of bounds this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_bytes("foo").unwrap();
    /// assert_eq!(s.remove(0).as_char(), 'f');
    /// assert_eq!(s.remove(1).as_char(), 'o');
    /// assert_eq!(s.remove(0).as_char(), 'o');
    /// ```
    #[inline]
    pub fn remove(&mut self, idx: usize) -> Ascii {
        self.vec.remove(idx)
    }

    /// Inserts a character into the ascii string buffer at byte position `idx`.
    ///
    /// # Warning
    ///
    /// This is an O(n) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// If `idx` is out of bounds this function will panic.
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: Ascii) {
        self.vec.insert(idx, ch)
    }

    /// Returns the number of bytes in this ascii string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let s = AsciiString::from_bytes("foo").unwrap();
    /// assert_eq!(s.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if the ascii string contains no bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::{Ascii, AsciiString};
    /// let mut s = AsciiString::new();
    /// assert!(s.is_empty());
    /// s.push(Ascii::from('a').unwrap());
    /// assert!(!s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates the ascii string, returning it to 0 length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_bytes("foo").unwrap();
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Converts anything that can represent a byte buffer into an `AsciiString`.
    ///
    /// # Failure
    /// Returns the byte buffer if it can not be ascii encoded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// let foo = AsciiString::from_bytes("foo").unwrap();
    /// let err = AsciiString::from_bytes("Ŋ");
    /// assert_eq!(foo.as_str(), "foo");
    /// assert_eq!(err, Err("Ŋ"));
    /// ```
    pub fn from_bytes<B>(bytes: B) -> Result<AsciiString, B> where B: Into<Vec<u8>> + AsRef<[u8]> {
        if bytes.as_ref().is_ascii() {
            unsafe { Ok( AsciiString::from_vec(bytes.into()) ) }
        } else {
            Err(bytes)
        }
    }

    unsafe fn from_vec(src: Vec<u8>) -> AsciiString {
        let vec = Vec::from_raw_parts(src.as_ptr() as *mut Ascii,
                                      src.len(),
                                      src.capacity());

        // We forget `src` to avoid freeing it at the end of the scope.
        // Otherwise, the returned `AsciiString` would point to freed memory.
        mem::forget(src);
        AsciiString { vec: vec }
    }
}

impl FromStr for AsciiString {
    type Err = ();

    fn from_str(s: &str) -> Result<AsciiString, ()> {
        if s.is_ascii() {
            unsafe { Ok(AsciiString::from_vec(s.as_bytes().to_vec())) }
        } else {
            Err(())
        }
    }
}

impl Deref for AsciiString {
    type Target = AsciiStr;

    #[inline]
    fn deref<'a>(&'a self) -> &'a AsciiStr {
        unsafe { mem::transmute(&self.vec[..]) }
    }
}

impl DerefMut for AsciiString {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut AsciiStr {
        unsafe { mem::transmute(&mut self.vec[..]) }
    }
}

impl Borrow<AsciiStr> for AsciiString {
    fn borrow(&self) -> &AsciiStr {
        &*self
    }
}

impl Into<Vec<u8>> for AsciiString {
    fn into(self) -> Vec<u8> {
        unsafe {
            let v = Vec::from_raw_parts(self.vec.as_ptr() as *mut u8,
                                        self.vec.len(),
                                        self.vec.capacity());

            // We forget `self` to avoid freeing it at the end of the scope.
            // Otherwise, the returned `Vec` would point to freed memory.
            mem::forget(self);
            v
        }
    }
}

impl Into<String> for AsciiString {
    fn into(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into()) }
    }
}

impl AsRef<AsciiStr> for AsciiString {
    fn as_ref(&self) -> &AsciiStr {
        &*self
    }
}

impl fmt::Display for AsciiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl fmt::Debug for AsciiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl Default for AsciiString {
    #[inline]
    fn default() -> AsciiString {
        AsciiString::new()
    }
}

impl FromIterator<Ascii> for AsciiString {
    fn from_iter<I: IntoIterator<Item=Ascii>>(iter: I) -> AsciiString {
        let mut buf = AsciiString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a AsciiStr> for AsciiString {
    fn from_iter<I: IntoIterator<Item=&'a AsciiStr>>(iter: I) -> AsciiString {
        let mut buf = AsciiString::new();
        buf.extend(iter);
        buf
    }
}

impl Extend<Ascii> for AsciiString {
    fn extend<I: IntoIterator<Item=Ascii>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        for ch in iterator {
            self.push(ch)
        }
    }
}

impl<'a> Extend<&'a Ascii> for AsciiString {
    fn extend<I: IntoIterator<Item=&'a Ascii>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned())
    }
}

impl<'a> Extend<&'a AsciiStr> for AsciiString {
    fn extend<I: IntoIterator<Item=&'a AsciiStr>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        for s in iterator {
            self.push_str(s)
        }
    }
}

impl<'a> Add<&'a AsciiStr> for AsciiString {
    type Output = AsciiString;

    #[inline]
    fn add(mut self, other: &AsciiStr) -> AsciiString {
        self.push_str(other);
        self
    }
}

impl<T> ops::Index<T> for AsciiString where AsciiStr: ops::Index<T> {
    type Output = <AsciiStr as ops::Index<T>>::Output;

    #[inline]
    fn index(&self, index: T) -> &<AsciiStr as ops::Index<T>>::Output {
        &(**self)[index]
    }
}

impl<T> ops::IndexMut<T> for AsciiString where AsciiStr: ops::IndexMut<T> {
    #[inline]
    fn index_mut(&mut self, index: T) -> &mut <AsciiStr as ops::Index<T>>::Output {
        &mut (**self)[index]
    }
}

/// A borrowed ascii string, like a slice into an `AsciiString`.
#[derive(Hash)]
pub struct AsciiStr {
    slice: [Ascii],
}

impl AsciiStr {
    /// Coerces into an `AsciiStr` slice.
    pub fn new<S: AsRef<AsciiStr> + ?Sized>(s: &S) -> &AsciiStr {
        s.as_ref()
    }

    /// Converts `&self` to a `&str` slice.
    pub fn as_str(&self) -> &str {
        unsafe { mem::transmute(&self.slice) }
    }

    /// Copies the content of this `AsciiStr` into an owned `AsciiString`.
    pub fn to_ascii_string(&self) -> AsciiString {
        AsciiString { vec: self.slice.to_vec() }
    }

    /// Converts `&self` into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::transmute(&self.slice) }
    }

    /// Converts anything that can represent a byte slice into an `AsciiStr`.
    ///
    /// # Failure
    /// Returns `None` if the byte slice can not be ascii encoded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let foo = AsciiStr::from_bytes("foo").unwrap();
    /// let err = AsciiStr::from_bytes("Ŋ");
    /// assert_eq!(foo.as_str(), "foo");
    /// assert_eq!(err, None);
    /// ```
    pub fn from_bytes<'a, B: ?Sized>(bytes: &'a B) -> Option<&'a AsciiStr> where B: AsRef<[u8]> {
        if bytes.as_ref().is_ascii() {
            unsafe { Some( mem::transmute(bytes.as_ref()) ) }
        } else {
            None
        }
    }

    /// Converts a borrowed string to a borrows ascii string.
    ///
    /// # Failure
    /// Returns `None` if the byte slice can not be ascii encoded.
    pub fn from_str<'a>(s: &'a str) -> Option<&'a AsciiStr> {
        AsciiStr::from_bytes(s.as_bytes())
    }

    /// Returns the number of bytes in this ascii string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let s = AsciiStr::from_bytes("foo").unwrap();
    /// assert_eq!(s.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns true if the ascii string contains no bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::{AsciiStr};
    /// let mut empty = AsciiStr::from_bytes("").unwrap();
    /// let mut full = AsciiStr::from_bytes("foo").unwrap();
    /// assert!(empty.is_empty());
    /// assert!(!full.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_slice(&self) -> &[Ascii] {
        &self.slice
    }
}

impl PartialEq for AsciiStr {
    fn eq(&self, other: &AsciiStr) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl Eq for AsciiStr {}

impl PartialOrd for AsciiStr {
    #[inline]
    fn partial_cmp(&self, other: &AsciiStr) -> Option<Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }

    #[inline]
    fn lt(&self, other: &AsciiStr) -> bool {
        self.as_bytes().lt(other.as_bytes())
    }

    #[inline]
    fn le(&self, other: &AsciiStr) -> bool {
        self.as_bytes().le(other.as_bytes())
    }

    #[inline]
    fn gt(&self, other: &AsciiStr) -> bool {
        self.as_bytes().gt(other.as_bytes())
    }

    #[inline]
    fn ge(&self, other: &AsciiStr) -> bool {
        self.as_bytes().ge(other.as_bytes())
    }
}

/*
impl PartialOrd<AsciiString> for AsciiStr {
    #[inline]
    fn partial_cmp(&self, other: &AsciiString) -> Option<Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}
*/

impl Ord for AsciiStr {
    #[inline]
    fn cmp(&self, other: &AsciiStr) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl ToOwned for AsciiStr {
    type Owned = AsciiString;

    fn to_owned(&self) -> AsciiString {
        self.to_ascii_string()
    }
}

impl AsRef<[u8]> for AsciiStr {
    fn as_ref(&self) -> &[u8] {
        unsafe { mem::transmute(&self.slice) }
    }
}

impl AsRef<str> for AsciiStr {
    fn as_ref(&self) -> &str {
        unsafe { mem::transmute(&self.slice) }
    }
}

impl fmt::Display for AsciiStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Debug for AsciiStr {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       fmt::Debug::fmt(self.as_str(), f)
   }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs:ty) => {
        impl<'a> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&**self, &**other)
            }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool {
                PartialEq::ne(&**self, &**other)
            }
        }
    }
}

impl_eq! { AsciiString, String }
impl_eq! { &'a AsciiStr, String }
impl_eq! { String, AsciiString }
impl_eq! { String, &'a AsciiStr }
impl_eq! { &'a AsciiStr, AsciiString }
impl_eq! { AsciiString, &'a AsciiStr }
impl_eq! { &'a str, AsciiString }
impl_eq! { AsciiString, &'a str }

impl PartialEq<str> for AsciiString {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl PartialEq<AsciiString> for str {
    fn eq(&self, other: &AsciiString) -> bool {
        **other == *self
    }
}

impl PartialEq<str> for AsciiStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<AsciiStr> for str {
    fn eq(&self, other: &AsciiStr) -> bool {
        other.as_str() == self
    }
}

macro_rules! impl_index {
    ($lhs:ty, $idx:ty, $rhs:ty) => {
        impl Index<$idx> for $lhs {
            type Output = $rhs;

            #[inline]
            fn index(&self, index: $idx) -> &$rhs {
                unsafe { mem::transmute(&self.slice[index]) }
            }
        }

        impl IndexMut<$idx> for $lhs {
            #[inline]
            fn index_mut(&mut self, index: $idx) -> &mut $rhs {
                unsafe { mem::transmute(&mut self.slice[index]) }
            }
        }
    }
}

impl_index! { AsciiStr, usize, Ascii }
impl_index! { AsciiStr, ops::Range<usize>, AsciiStr }
impl_index! { AsciiStr, ops::RangeTo<usize>, AsciiStr }
impl_index! { AsciiStr, ops::RangeFrom<usize>, AsciiStr }
impl_index! { AsciiStr, ops::RangeFull, AsciiStr }

#[cfg(feature = "unstable")]
impl AsciiExt for Ascii {
    type Owned = Ascii;

    #[inline]
    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> Ascii {
        Ascii{chr: self.chr.to_ascii_uppercase()}
    }

    fn to_ascii_lowercase(&self) -> Ascii {
        Ascii{chr: self.chr.to_ascii_lowercase()}
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.chr.eq_ignore_ascii_case(&other.chr)
    }

    #[inline]
    fn make_ascii_uppercase(&mut self) {
        self.chr.make_ascii_uppercase()
    }

    #[inline]
    fn make_ascii_lowercase(&mut self) {
        self.chr.make_ascii_lowercase()
    }
}

#[cfg(feature = "unstable")]
impl AsciiExt for AsciiStr {
    type Owned = AsciiString;

    #[inline]
    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> AsciiString {
        let mut ascii_string = self.to_ascii_string();
        ascii_string.make_ascii_uppercase();
        ascii_string
    }

    fn to_ascii_lowercase(&self) -> AsciiString {
        let mut ascii_string = self.to_ascii_string();
        ascii_string.make_ascii_uppercase();
        ascii_string
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.len() == other.len() &&
        self.slice.iter().zip(other.slice.iter()).all(|(a, b)| a.eq_ignore_ascii_case(b))
    }

    fn make_ascii_uppercase(&mut self) {
        for ascii in &mut self.slice {
            ascii.make_ascii_uppercase();
        }
    }

    fn make_ascii_lowercase(&mut self) {
        for ascii in &mut self.slice {
            ascii.make_ascii_lowercase();
        }
    }
}

#[cfg(feature = "unstable")]
impl OwnedAsciiExt for AsciiString {
    #[inline]
    fn into_ascii_uppercase(mut self) -> AsciiString {
        self.make_ascii_uppercase();
        self
    }

    #[inline]
    fn into_ascii_lowercase(mut self) -> AsciiString {
        self.make_ascii_lowercase();
        self
    }
}

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

impl<'a> AsciiCast<'a> for [u8] {
    type Target = &'a AsciiStr;

    #[inline]
    unsafe fn to_ascii_nocheck(&'a self) -> &'a AsciiStr {
        mem::transmute(self)
    }
}

impl<'a> AsciiCast<'a> for str {
    type Target = &'a AsciiStr;

    #[inline]
    unsafe fn to_ascii_nocheck(&'a self) -> &'a AsciiStr {
        mem::transmute(self)
    }
}

impl<'a> AsciiCast<'a> for u8 {
    type Target = Ascii;

    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self }
    }
}

impl<'a> AsciiCast<'a> for char {
    type Target = Ascii;

    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self as u8 }
    }
}

/// Trait for copyless casting to an ascii vector.
pub trait OwnedAsciiCast<T: ?Sized> : Sized + Borrow<T>
where T: AsciiExt<Owned=Self> {
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
        let v = Vec::from_raw_parts(self.as_ptr() as *mut Ascii,
                                    self.len(),
                                    self.capacity());

        // We forget `self` to avoid freeing it at the end of the scope
        // Otherwise, the returned `Vec` would point to freed memory
        mem::forget(self);
        AsciiString { vec: v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        assert_eq!(65u8.to_ascii().unwrap().as_byte(), 65u8);
        assert_eq!(65u8.to_ascii().unwrap().as_char(), 'A');
        assert_eq!('A'.to_ascii().unwrap().as_char(), 'A');
        assert_eq!('A'.to_ascii().unwrap().as_byte(), 65u8);

        assert!('0'.to_ascii().unwrap().is_digit());
        assert!('9'.to_ascii().unwrap().is_digit());
        assert!(!'/'.to_ascii().unwrap().is_digit());
        assert!(!':'.to_ascii().unwrap().is_digit());

        assert!(0x1f_u8.to_ascii().unwrap().is_control());
        assert!(!' '.to_ascii().unwrap().is_control());
        assert!(0x7f_u8.to_ascii().unwrap().is_control());
    }

    #[test]
    fn ascii_vec() {
        let test = &[40_u8, 32, 59];
        let b = AsciiStr::from_bytes(test).unwrap();
        assert_eq!(test.to_ascii().unwrap(), b);
        assert_eq!("( ;".to_ascii().unwrap(), b);
        let v = vec![40_u8, 32, 59];
        assert_eq!(v.to_ascii().unwrap(), b);
        assert_eq!("( ;".to_string().to_ascii().unwrap(), b);
    }

    #[test]
    fn ascii_str_as_str() {
        let b = &[40_u8, 32, 59];
        let v = AsciiStr::from_bytes(b).unwrap();
        assert_eq!(v.as_str(), "( ;");
        assert_eq!(AsRef::<str>::as_ref(v), "( ;");
    }

    #[test]
    fn ascii_str_as_bytes() {
        let b = &[40_u8, 32, 59];
        let v = AsciiStr::from_bytes(b).unwrap();
        assert_eq!(v.as_bytes(), b"( ;");
        assert_eq!(AsRef::<[u8]>::as_ref(v), b"( ;");
    }

    #[test]
    fn ascii_string_into_string() {
        let v = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<String>::into(v), "( ;".to_string());
    }

    #[test]
    fn ascii_string_into_bytes() {
        let v = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<Vec<u8>>::into(v), vec![40_u8, 32, 59])
    }

    #[test]
    fn opt() {
        assert_eq!(65_u8.to_ascii(), Ok(Ascii { chr: 65_u8 }));
        assert_eq!(255_u8.to_ascii(), Err(()));

        assert_eq!('A'.to_ascii(), Ok(Ascii { chr: 65_u8 }));
        assert_eq!('λ'.to_ascii(), Err(()));

        assert_eq!("zoä华".to_ascii(), Err(()));

        let test1 = &[127_u8, 128, 255];
        assert_eq!(test1.to_ascii(), Err(()));

        let v = [40_u8, 32, 59];
        let v1 = AsciiStr::from_bytes(&v).unwrap();
        assert_eq!(v.to_ascii(), Ok(v1));
        let v = [127_u8, 128, 255];
        assert_eq!(v.to_ascii(), Err(()));

        let v = "( ;";
        assert_eq!(v.to_ascii(), Ok(v1));
        assert_eq!("zoä华".to_ascii(), Err(()));

        let v1 = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(vec![40_u8, 32, 59].into_ascii(), Ok(v1));
        assert_eq!(vec![127_u8, 128, 255].into_ascii(), Err(vec![127_u8, 128, 255]));

        let v1 = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!("( ;".to_string().into_ascii(), Ok(v1));
        assert_eq!("zoä华".to_string().into_ascii(), Err("zoä华".to_string()));
    }

    #[test]
    fn fmt_display_ascii() {
        let s = Ascii{ chr: b't' };
        assert_eq!(format!("{}", s), "t".to_string());
    }

    #[test]
    fn fmt_display_ascii_str() {
        let s = "abc".to_ascii().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
    }

    #[test]
    fn fmt_display_ascii_string() {
        let s = "abc".to_string().into_ascii().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
    }

    #[test]
    fn fmt_debug_ascii() {
        let c = Ascii { chr: b't' };
        assert_eq!(format!("{:?}", c), "'t'".to_string());
    }

    #[test]
    fn fmt_debug_ascii_str() {
        let s = "abc".to_ascii().unwrap();
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }

    #[test]
    fn fmt_debug_ascii_string() {
        let s = "abc".to_string().into_ascii().unwrap();
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }

    #[test]
    fn compare_ascii_string_ascii_str() {
        let v = b"abc";
        let ascii_string = AsciiString::from_bytes(&v[..]).unwrap();
        let ascii_str = AsciiStr::from_bytes(v).unwrap();
        assert!(ascii_string == ascii_str);
        assert!(ascii_str == ascii_string);
    }

    #[test]
    fn compare_ascii_string_string() {
        let v = b"abc";
        let string = String::from_utf8(v.to_vec()).unwrap();
        let ascii_string = AsciiString::from_bytes(&v[..]).unwrap();
        assert!(string == ascii_string);
        assert!(ascii_string == string);
    }

    #[test]
    fn compare_ascii_str_string() {
        let v = b"abc";
        let string = String::from_utf8(v.to_vec()).unwrap();
        let ascii_str = AsciiStr::from_bytes(&v[..]).unwrap();
        assert!(string == ascii_str);
        assert!(ascii_str == string);
    }

    #[test]
    fn compare_ascii_string_str() {
        let v = b"abc";
        let sstr = ::std::str::from_utf8(v).unwrap();
        let ascii_string = AsciiString::from_bytes(&v[..]).unwrap();
        assert!(sstr == ascii_string);
        assert!(ascii_string == sstr);
    }

    #[test]
    fn compare_ascii_str_str() {
        let v = b"abc";
        let sstr = ::std::str::from_utf8(v).unwrap();
        let ascii_str = AsciiStr::from_bytes(v).unwrap();
        assert!(sstr == ascii_str);
        assert!(ascii_str == sstr);
    }

    #[test]
    fn compare_ascii_str_slice() {
        let b = b"abc".to_ascii().unwrap();
        let c = b"ab".to_ascii().unwrap();
        assert_eq!(&b[..2], &c[..]);
        assert_eq!(c[1].as_char(), 'b');
    }

    #[test]
    fn compare_ascii_string_slice() {
        let b = AsciiString::from_bytes("abc").unwrap();
        let c = AsciiString::from_bytes("ab").unwrap();
        assert_eq!(&b[..2], &c[..]);
        assert_eq!(c[1].as_char(), 'b');
    }
}
