use std::{fmt, mem};
use std::ascii::AsciiExt;
use std::borrow::Borrow;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Add, Index, IndexMut};
use std::iter::FromIterator;

use ascii::Ascii;
use ascii_str::AsciiStr;

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
    pub fn new() -> Self {
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
    pub fn with_capacity(capacity: usize) -> Self {
        AsciiString {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Converts a vector of bytes to an `AsciiString` without checking that the vector contains
    /// valid ascii characters.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ascii characters. If this constraint is violated, it may cause memory unsafety issues with
    /// future of the `AsciiString`, as the rest of this library assumes that `AsciiString`s are
    /// ascii encoded.
    pub unsafe fn from_bytes_unchecked<B>(bytes: B) -> Self
        where B: Into<Vec<u8>>
    {
        let bytes: Vec<u8> = bytes.into();
        let vec = Vec::from_raw_parts(bytes.as_ptr() as *mut Ascii,
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
    pub fn from_bytes<B>(bytes: B) -> Result<AsciiString, B>
        where B: Into<Vec<u8>> + AsRef<[u8]>
    {
        unsafe {
            if bytes.as_ref().is_ascii() {
                Ok( AsciiString::from_bytes_unchecked(bytes) )
            } else {
                Err(bytes)
            }
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

impl FromStr for AsciiString {
    type Err = ();

    fn from_str(s: &str) -> Result<AsciiString, ()> {
        AsciiStr::from_str(s).map(AsciiStr::to_ascii_string)
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

#[cfg(test)]
mod tests {
    use OwnedAsciiCast;
    use super::AsciiString;

    #[test]
    fn into_string() {
        let v = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<String>::into(v), "( ;".to_string());
    }

    #[test]
    fn into_bytes() {
        let v = AsciiString::from_bytes(&[40_u8, 32, 59][..]).unwrap();
        assert_eq!(Into::<Vec<u8>>::into(v), vec![40_u8, 32, 59])
    }

    #[test]
    fn fmt_display_ascii_string() {
        let s = "abc".to_string().into_ascii().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
    }

    #[test]
    fn fmt_debug_ascii_string() {
        let s = "abc".to_string().into_ascii().unwrap();
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }
}
