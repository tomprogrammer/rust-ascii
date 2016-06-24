use std::{fmt, mem};
use std::ascii::AsciiExt;
use std::borrow::Borrow;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Add, Index, IndexMut};
use std::iter::FromIterator;

use ascii_char::AsciiChar;
use ascii_str::{AsciiStr,AsAsciiStr,AsAsciiStrError};

/// A growable string stored as an ASCII encoded buffer.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiString {
    vec: Vec<AsciiChar>,
}

impl AsciiString {
    /// Creates a new, empty ASCII string buffer without allocating.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        AsciiString { vec: Vec::new() }
    }

    /// Creates a new ASCII string buffer with the given capacity.
    /// The string will be able to hold exactly `capacity` bytes without reallocating.
    /// If `capacity` is 0, the ASCII string will not allocate.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        AsciiString {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `AsciiString` from a length, capacity and pointer.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't checked:
    ///
    /// * The memory at `ptr` need to have been previously allocated by the same allocator this
    ///   library uses.
    /// * `length` needs to be less than or equal to `capacity`.
    /// * `capacity` needs to be the correct value.
    ///
    /// Violating these may cause problems like corrupting the allocator's internal datastructures.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ascii::AsciiString;
    /// use std::mem;
    ///
    /// unsafe {
    ///    let s = AsciiString::from_ascii("hello").unwrap();
    ///    let ptr = s.as_ptr();
    ///    let len = s.len();
    ///    let capacity = s.capacity();
    ///
    ///    mem::forget(s);
    ///
    ///    let s = AsciiString::from_raw_parts(ptr as *mut _, len, capacity);
    ///
    ///    assert_eq!(AsciiString::from_ascii("hello").unwrap(), s);
    /// }
    /// ```
    pub unsafe fn from_raw_parts(buf: *mut AsciiChar, length: usize, capacity: usize) -> Self {
        AsciiString {
            vec: Vec::from_raw_parts(buf, length, capacity),
        }
    }

    /// Converts a vector of bytes to an `AsciiString` without checking for non-ASCII characters.
    ///
    /// # Safety
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ASCII characters. If this constraint is violated, it may cause memory unsafety issues with
    /// future of the `AsciiString`, as the rest of this library assumes that `AsciiString`s are
    /// ASCII encoded.
    pub unsafe fn from_ascii_unchecked<B>(bytes: B) -> Self
        where B: Into<Vec<u8>>
    {
        let bytes: Vec<u8> = bytes.into();
        let vec = Vec::from_raw_parts(bytes.as_ptr() as *mut AsciiChar,
                                      bytes.len(),
                                      bytes.capacity());

        // We forget `src` to avoid freeing it at the end of the scope.
        // Otherwise, the returned `AsciiString` would point to freed memory.
        mem::forget(bytes);
        AsciiString { vec: vec }
    }

    /// Converts anything that can represent a byte buffer into an `AsciiString`.
    ///
    /// # Failure
    /// Returns the byte buffer if not all of the bytes are ASCII characters.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let foo = AsciiString::from_ascii("foo").unwrap();
    /// let err = AsciiString::from_ascii("Ŋ");
    /// assert_eq!(foo.as_str(), "foo");
    /// assert_eq!(err, Err("Ŋ"));
    /// ```
    pub fn from_ascii<B>(bytes: B) -> Result<AsciiString, B>
        where B: Into<Vec<u8>> + AsRef<[u8]>
    {
        unsafe {
            if bytes.as_ref().is_ascii() {
                Ok( AsciiString::from_ascii_unchecked(bytes) )
            } else {
                Err(bytes)
            }
        }
    }

    /// Pushes the given ASCII string onto this ASCII string buffer.
    ///
    /// # Examples
    /// ```
    /// # use ascii::{AsciiString, AsAsciiStr};
    /// use std::str::FromStr;
    /// let mut s = AsciiString::from_str("foo").unwrap();
    /// s.push_str("bar".as_ascii_str().unwrap());
    /// assert_eq!(s, "foobar".as_ascii_str().unwrap());
    /// ```
    #[inline]
    pub fn push_str(&mut self, string: &AsciiStr) {
        self.vec.extend(string.as_slice().iter().cloned())
    }

    /// Returns the number of bytes that this ASCII string buffer can hold without reallocating.
    ///
    /// # Examples
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
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
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
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
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

    /// Shrinks the capacity of this ASCII string buffer to match it's length.
    ///
    /// # Examples
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

    /// Adds the given ASCII character to the end of the ASCII string.
    ///
    /// # Examples
    /// ```
    /// # use ascii::{ AsciiChar, AsciiString};
    /// let mut s = AsciiString::from_ascii("abc").unwrap();
    /// s.push(AsciiChar::from('1').unwrap());
    /// s.push(AsciiChar::from('2').unwrap());
    /// s.push(AsciiChar::from('3').unwrap());
    /// assert_eq!(s, "abc123");
    /// ```
    #[inline]
    pub fn push(&mut self, ch: AsciiChar) {
        self.vec.push(ch)
    }

    /// Shortens a ASCII string to the specified length.
    ///
    /// # Panics
    /// Panics if `new_len` > current length.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_ascii("hello").unwrap();
    /// s.truncate(2);
    /// assert_eq!(s, "he");
    /// ```
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.vec.truncate(new_len)
    }

    /// Removes the last character from the ASCII string buffer and returns it.
    /// Returns `None` if this string buffer is empty.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_ascii("foo").unwrap();
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('o'));
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('o'));
    /// assert_eq!(s.pop().map(|c| c.as_char()), Some('f'));
    /// assert_eq!(s.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<AsciiChar> {
        self.vec.pop()
    }

    /// Removes the ASCII character at position `idx` from the buffer and returns it.
    ///
    /// # Warning
    /// This is an O(n) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    /// If `idx` is out of bounds this function will panic.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_ascii("foo").unwrap();
    /// assert_eq!(s.remove(0).as_char(), 'f');
    /// assert_eq!(s.remove(1).as_char(), 'o');
    /// assert_eq!(s.remove(0).as_char(), 'o');
    /// ```
    #[inline]
    pub fn remove(&mut self, idx: usize) -> AsciiChar {
        self.vec.remove(idx)
    }

    /// Inserts an ASCII character into the buffer at position `idx`.
    ///
    /// # Warning
    /// This is an O(n) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    /// If `idx` is out of bounds this function will panic.
    ///
    /// # Examples
    /// ```
    /// # use ascii::{AsciiString,AsciiChar};
    /// let mut s = AsciiString::from_ascii("foo").unwrap();
    /// s.insert(2, AsciiChar::b);
    /// assert_eq!(s, "fobo");
    /// ```
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: AsciiChar) {
        self.vec.insert(idx, ch)
    }

    /// Returns the number of bytes in this ASCII string.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let s = AsciiString::from_ascii("foo").unwrap();
    /// assert_eq!(s.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if the ASCII string contains zero bytes.
    ///
    /// # Examples
    /// ```
    /// # use ascii::{AsciiChar, AsciiString};
    /// let mut s = AsciiString::new();
    /// assert!(s.is_empty());
    /// s.push(AsciiChar::from('a').unwrap());
    /// assert!(!s.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates the ASCII string, setting length (but not capacity) to zero.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiString;
    /// let mut s = AsciiString::from_ascii("foo").unwrap();
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear()
    }
}

impl Deref for AsciiString {
    type Target = AsciiStr;

    #[inline]
    fn deref(&self) -> &AsciiStr {
        unsafe { mem::transmute(&self.vec[..]) }
    }
}

impl DerefMut for AsciiString {
    #[inline]
    fn deref_mut(&mut self) -> &mut AsciiStr {
        unsafe { mem::transmute(&mut self.vec[..]) }
    }
}

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
impl_eq! { String, AsciiString }
impl_eq! { &'a AsciiStr, String }
impl_eq! { String, &'a AsciiStr }
impl_eq! { &'a AsciiStr, AsciiString }
impl_eq! { AsciiString, &'a AsciiStr }
impl_eq! { &'a str, AsciiString }
impl_eq! { AsciiString, &'a str }

impl Borrow<AsciiStr> for AsciiString {
    fn borrow(&self) -> &AsciiStr {
        &*self
    }
}

impl From<Vec<AsciiChar>> for AsciiString {
    fn from(vec: Vec<AsciiChar>) -> Self {
        AsciiString { vec: vec }
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

impl AsMut<AsciiStr> for AsciiString {
    fn as_mut(&mut self) -> &mut AsciiStr {
        &mut *self
    }
}

impl FromStr for AsciiString {
    type Err = AsAsciiStrError;

    fn from_str(s: &str) -> Result<AsciiString, AsAsciiStrError> {
        s.as_ascii_str().map(AsciiStr::to_ascii_string)
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

impl FromIterator<AsciiChar> for AsciiString {
    fn from_iter<I: IntoIterator<Item=AsciiChar>>(iter: I) -> AsciiString {
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

impl Extend<AsciiChar> for AsciiString {
    fn extend<I: IntoIterator<Item=AsciiChar>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        for ch in iterator {
            self.push(ch)
        }
    }
}

impl<'a> Extend<&'a AsciiChar> for AsciiString {
    fn extend<I: IntoIterator<Item=&'a AsciiChar>>(&mut self, iter: I) {
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

impl<T> Index<T> for AsciiString where AsciiStr: Index<T> {
    type Output = <AsciiStr as Index<T>>::Output;

    #[inline]
    fn index(&self, index: T) -> &<AsciiStr as Index<T>>::Output {
        &(**self)[index]
    }
}

impl<T> IndexMut<T> for AsciiString where AsciiStr: IndexMut<T> {
    #[inline]
    fn index_mut(&mut self, index: T) -> &mut <AsciiStr as Index<T>>::Output {
        &mut (**self)[index]
    }
}


/// Convert vectors into `AsciiString`.
pub trait IntoAsciiString : Sized {
    /// Convert to `AsciiString` without checking for non-ASCII characters.
    unsafe fn into_ascii_string_unchecked(self) -> AsciiString;
    /// Convert to `AsciiString`.
    fn into_ascii_string(self) -> Result<AsciiString,Self>;
}

impl IntoAsciiString for AsciiString {
    unsafe fn into_ascii_string_unchecked(self) -> AsciiString {
        self
    }
    fn into_ascii_string(self) -> Result<AsciiString,Self> {
        Ok(self)
    }
}

impl IntoAsciiString for Vec<AsciiChar> {
    unsafe fn into_ascii_string_unchecked(self) -> AsciiString {
        AsciiString::from(self)
    }
    fn into_ascii_string(self) -> Result<AsciiString,Self> {
        Ok(AsciiString::from(self))
    }
}

impl IntoAsciiString for Vec<u8> {
    unsafe fn into_ascii_string_unchecked(self) -> AsciiString {
        AsciiString::from_ascii_unchecked(self)
    }
    fn into_ascii_string(self) -> Result<AsciiString,Self> {
        AsciiString::from_ascii(self)
    }
}

impl IntoAsciiString for String {
    unsafe fn into_ascii_string_unchecked(self) -> AsciiString {
        self.into_bytes().into_ascii_string_unchecked()
    }
    fn into_ascii_string(self) -> Result<AsciiString,Self> {
        AsciiString::from_ascii(self)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use AsciiChar;
    use super::{AsciiString, IntoAsciiString};

    #[test]
    fn into_string() {
        let v = AsciiString::from_ascii(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<String>::into(v), "( ;".to_string());
    }

    #[test]
    fn into_bytes() {
        let v = AsciiString::from_ascii(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<Vec<u8>>::into(v), vec![40_u8, 32, 59])
    }

    #[test]
    fn from_ascii_vec() {
        let vec = vec![AsciiChar::from('A').unwrap(), AsciiChar::from('B').unwrap()];
        assert_eq!(AsciiString::from(vec), AsciiString::from_str("AB").unwrap());
    }

    #[test]
    fn fmt_display_ascii_string() {
        let s = "abc".to_string().into_ascii_string().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
    }

    #[test]
    fn fmt_debug_ascii_string() {
        let s = "abc".to_string().into_ascii_string().unwrap();
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }
}
