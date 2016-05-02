use std::{fmt, mem};
use std::ops::{Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
use std::ascii::AsciiExt;

use AsciiCast;
use ascii::Ascii;
use ascii_string::AsciiString;

/// A borrowed ascii string, like a slice into an `AsciiString`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Converts `&self` into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::transmute(&self.slice) }
    }

    /// Returns the entire string as slice of `Ascii` characters.
    pub fn as_slice(&self) -> &[Ascii] {
        &self.slice
    }

    /// Returns the entire string as mutable slice of `Ascii` characters.
    pub fn as_mut_slice(&mut self) -> &mut [Ascii] {
        &mut self.slice
    }

    /// Returns a raw pointer to the `AsciiStr`'s buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this function returns, or else it
    /// will end up pointing to garbage. Modifying the `AsciiStr` may cause it's buffer to be
    /// reallocated, which would also make any pointers to it invalid.
    pub fn as_ptr(&self) -> *const Ascii {
        self.as_slice().as_ptr()
    }

    /// Returns an unsafe mutable pointer to the `AsciiStr`'s buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this function returns, or else it
    /// will end up pointing to garbage. Modifying the `AsciiStr` may cause it's buffer to be
    /// reallocated, which would also make any pointers to it invalid.
    pub fn as_mut_ptr(&mut self) -> *mut Ascii {
        self.as_mut_slice().as_mut_ptr()
    }

    /// Copies the content of this `AsciiStr` into an owned `AsciiString`.
    pub fn to_ascii_string(&self) -> AsciiString {
        AsciiString::from(self.slice.to_vec())
    }

    /// Converts anything that can represent a byte slice into an `AsciiStr`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let foo = AsciiStr::from_bytes("foo");
    /// let err = AsciiStr::from_bytes("Ŋ");
    /// assert_eq!(foo.unwrap().as_str(), "foo");
    /// assert_eq!(err, Err(()));
    /// ```
    pub fn from_bytes<'a, B: ?Sized>(bytes: &'a B) -> Result<&'a AsciiStr, ()>
        where B: AsRef<[u8]>
    {
        unsafe {
            if bytes.as_ref().is_ascii() {
                Ok( mem::transmute(bytes.as_ref()) )
            } else {
                Err(())
            }
        }
    }

    /// Converts a borrowed string to a borrows ascii string.
    pub fn from_str<'a>(s: &'a str) -> Result<&'a AsciiStr, ()> {
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
    /// # use ascii::AsciiStr;
    /// let mut empty = AsciiStr::from_bytes("").unwrap();
    /// let mut full = AsciiStr::from_bytes("foo").unwrap();
    /// assert!(empty.is_empty());
    /// assert!(!full.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an ascii string slice with leading and trailing whitespace removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_str("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("white \tspace", example.trim());
    /// ```
    pub fn trim(&self) -> &Self {
        unsafe { mem::transmute(self.as_str().trim()) }
    }

    /// Returns a string slice with leading whitespace removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_str("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("white \tspace  \t", example.trim_left());
    /// ```
    pub fn trim_left(&self) -> &Self {
        unsafe { mem::transmute(self.as_str().trim_left()) }
    }

    /// Returns a string slice with trainling whitespace removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_str("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("  \twhite \tspace", example.trim_right());
    /// ```
    pub fn trim_right(&self) -> &Self {
        unsafe { mem::transmute(self.as_str().trim_right()) }
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

/*
impl PartialOrd<AsciiString> for AsciiStr {
    #[inline]
    fn partial_cmp(&self, other: &AsciiString) -> Option<Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
}
*/

impl Default for &'static AsciiStr {
    fn default() -> &'static AsciiStr {
        unsafe { mem::transmute("") }
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
        self.as_bytes()
    }
}
impl AsRef<str> for AsciiStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<[Ascii]> for AsciiStr {
    fn as_ref(&self) -> &[Ascii] {
        &self.slice
    }
}
impl AsMut<[Ascii]> for AsciiStr {
    fn as_mut(&mut self) -> &mut[Ascii] {
        &mut self.slice
    }
}

impl<'a> From<&'a[Ascii]> for &'a AsciiStr {
    fn from(slice: &[Ascii]) -> &AsciiStr {
        unsafe{ mem::transmute(slice) }
    }
}
impl<'a> From<&'a mut [Ascii]> for &'a mut AsciiStr {
    fn from(slice: &mut[Ascii]) -> &mut AsciiStr {
        unsafe{ mem::transmute(slice) }
    }
}
impl From<Box<[Ascii]>> for Box<AsciiStr> {
    fn from(owned: Box<[Ascii]>) -> Box<AsciiStr> {
        unsafe{ mem::transmute(owned) }
    }
}

macro_rules! impl_into {
    ($wider: ty) => {
        impl<'a> From<&'a AsciiStr> for &'a$wider {
            fn from(slice: &AsciiStr) -> &$wider {
                unsafe{ mem::transmute(slice) }
            }
        }
        impl From<Box<AsciiStr>> for Box<$wider> {
            fn from(owned: Box<AsciiStr>) -> Box<$wider> {
                unsafe{ mem::transmute(owned) }
            }
        }
    }
}
impl_into! {[Ascii]}
impl_into! {[u8]}
impl_into! {str}

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
impl_index! { AsciiStr, Range<usize>, AsciiStr }
impl_index! { AsciiStr, RangeTo<usize>, AsciiStr }
impl_index! { AsciiStr, RangeFrom<usize>, AsciiStr }
impl_index! { AsciiStr, RangeFull, AsciiStr }

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

#[cfg(test)]
mod tests {
    use AsciiCast;
    use super::AsciiStr;

    #[test]
    fn default() {
        let default: &'static AsciiStr = Default::default();
        assert!(default.is_empty());
    }

    #[test]
    fn as_str() {
        let b = &[40_u8, 32, 59];
        let v = AsciiStr::from_bytes(b).unwrap();
        assert_eq!(v.as_str(), "( ;");
        assert_eq!(AsRef::<str>::as_ref(v), "( ;");
    }

    #[test]
    fn as_bytes() {
        let b = &[40_u8, 32, 59];
        let v = AsciiStr::from_bytes(b).unwrap();
        assert_eq!(v.as_bytes(), b"( ;");
        assert_eq!(AsRef::<[u8]>::as_ref(v), b"( ;");
    }

    #[test]
    fn fmt_display_ascii_str() {
        let s = "abc".to_ascii().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
    }

    #[test]
    fn fmt_debug_ascii_str() {
        let s = "abc".to_ascii().unwrap();
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }
}
