use core::{fmt, mem};
use core::ops::{Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::ascii::AsciiExt;

use ascii_char::AsciiChar;
#[cfg(feature = "std")]
use ascii_string::AsciiString;

/// AsciiStr represents a byte or string slice that only contains ASCII characters.
///
/// It wraps an `[AsciiChar]` and implements many of `str`s methods and traits.
///
/// It can be created by a checked conversion from a `str` or `[u8]`,
/// or borrowed from an `AsciiString`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiStr {
    slice: [AsciiChar],
}

impl AsciiStr {
    /// Coerces into an `AsciiStr` slice.
    pub fn new<S: AsRef<AsciiStr> + ?Sized>(s: &S) -> &AsciiStr {
        s.as_ref()
    }

    /// Converts `&self` to a `&str` slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { mem::transmute(&self.slice) }
    }

    /// Converts `&self` into a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::transmute(&self.slice) }
    }

    /// Returns the entire string as slice of `AsciiChar`s.
    #[inline]
    pub fn as_slice(&self) -> &[AsciiChar] {
        &self.slice
    }

    /// Returns the entire string as mutable slice of `AsciiChar`s.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [AsciiChar] {
        &mut self.slice
    }

    /// Returns a raw pointer to the `AsciiStr`'s buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this function returns, or else it
    /// will end up pointing to garbage. Modifying the `AsciiStr` may cause it's buffer to be
    /// reallocated, which would also make any pointers to it invalid.
    #[inline]
    pub fn as_ptr(&self) -> *const AsciiChar {
        self.as_slice().as_ptr()
    }

    /// Returns an unsafe mutable pointer to the `AsciiStr`'s buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this function returns, or else it
    /// will end up pointing to garbage. Modifying the `AsciiStr` may cause it's buffer to be
    /// reallocated, which would also make any pointers to it invalid.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut AsciiChar {
        self.as_mut_slice().as_mut_ptr()
    }

    /// Copies the content of this `AsciiStr` into an owned `AsciiString`.
    #[cfg(feature = "std")]
    pub fn to_ascii_string(&self) -> AsciiString {
        AsciiString::from(self.slice.to_vec())
    }

    /// Converts anything that can represent a byte slice into an `AsciiStr`.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let foo = AsciiStr::from_ascii("foo");
    /// let err = AsciiStr::from_ascii("Ŋ");
    /// assert_eq!(foo.unwrap().as_str(), "foo");
    /// assert_eq!(err.unwrap_err().valid_up_to(), 0);
    /// ```
    #[inline]
    pub fn from_ascii<B: ?Sized>(bytes: &B) -> Result<&AsciiStr, AsAsciiStrError>
        where B: AsRef<[u8]>
    {
        bytes.as_ref().as_ascii_str()
    }

    /// Converts anything that can be represented as a byte slice to an `AsciiStr` without checking
    /// for non-ASCII characters..
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let foo = unsafe{ AsciiStr::from_ascii_unchecked("foo") };
    /// assert_eq!(foo.as_str(), "foo");
    /// ```
    #[inline]
    pub unsafe fn from_ascii_unchecked<B: ?Sized>(bytes: &B) -> &AsciiStr
        where B: AsRef<[u8]>
    {
        bytes.as_ref().as_ascii_str_unchecked()
    }

    /// Returns the number of characters / bytes in this ASCII sequence.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let s = AsciiStr::from_ascii("foo").unwrap();
    /// assert_eq!(s.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns true if the ASCII slice contains zero bytes.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let mut empty = AsciiStr::from_ascii("").unwrap();
    /// let mut full = AsciiStr::from_ascii("foo").unwrap();
    /// assert!(empty.is_empty());
    /// assert!(!full.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an ASCII string slice with leading and trailing whitespace removed.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_ascii("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("white \tspace", example.trim());
    /// ```
    pub fn trim(&self) -> &Self {
        self.trim_right().trim_left()
    }

    /// Returns an ASCII string slice with leading whitespace removed.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_ascii("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("white \tspace  \t", example.trim_left());
    /// ```
    pub fn trim_left(&self) -> &Self {
        &self[self.slice.iter().take_while(|a| a.is_whitespace() ).count()..]
    }

    /// Returns an ASCII string slice with trailing whitespace removed.
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let example = AsciiStr::from_ascii("  \twhite \tspace  \t").unwrap();
    /// assert_eq!("  \twhite \tspace", example.trim_right());
    /// ```
    pub fn trim_right(&self) -> &Self {
        let trimmed = self.slice.into_iter()
                          .rev().take_while(|a| a.is_whitespace() ).count();
        &self[..self.len()-trimmed]
    }

    /// Compares two strings case-insensitively.
    ///
    /// A replacement for `AsciiExt::eq_ignore_ascii_case()`.
    #[cfg(not(feature = "std"))]
    pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.len() == other.len() &&
        self.slice.iter().zip(other.slice.iter()).all(|(a, b)| a.eq_ignore_ascii_case(b) )
    }

    /// Replaces lowercase letters with their uppercase equivalent.
    ///
    /// A replacement for `AsciiExt::make_ascii_uppercase()`.
    #[cfg(not(feature = "std"))]
    pub fn make_ascii_uppercase(&mut self) {
        for a in &mut self.slice {
            *a = a.to_ascii_uppercase();
        }
    }

    /// Replaces uppercase letters with their lowercase equivalent.
    ///
    /// A replacement for `AsciiExt::make_ascii_lowercase()`.
    #[cfg(not(feature = "std"))]
    pub fn make_ascii_lowercase(&mut self) {
        for a in &mut self.slice {
            *a = a.to_ascii_lowercase();
        }
    }
}

impl PartialEq<str> for AsciiStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<AsciiStr> for str {
    #[inline]
    fn eq(&self, other: &AsciiStr) -> bool {
        other.as_str() == self
    }
}

#[cfg(feature = "std")]
impl ToOwned for AsciiStr {
    type Owned = AsciiString;

    #[inline]
    fn to_owned(&self) -> AsciiString {
        self.to_ascii_string()
    }
}

impl AsRef<[u8]> for AsciiStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsRef<str> for AsciiStr {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<[AsciiChar]> for AsciiStr {
    #[inline]
    fn as_ref(&self) -> &[AsciiChar] {
        &self.slice
    }
}
impl AsMut<[AsciiChar]> for AsciiStr {
    #[inline]
    fn as_mut(&mut self) -> &mut[AsciiChar] {
        &mut self.slice
    }
}

impl Default for &'static AsciiStr {
    #[inline]
    fn default() -> &'static AsciiStr {
        unsafe{ "".as_ascii_str_unchecked() }
    }
}
impl<'a> From<&'a[AsciiChar]> for &'a AsciiStr {
    #[inline]
    fn from(slice: &[AsciiChar]) -> &AsciiStr {
        unsafe{ mem::transmute(slice) }
    }
}
impl<'a> From<&'a mut [AsciiChar]> for &'a mut AsciiStr {
    #[inline]
    fn from(slice: &mut[AsciiChar]) -> &mut AsciiStr {
        unsafe{ mem::transmute(slice) }
    }
}
#[cfg(feature = "std")]
impl From<Box<[AsciiChar]>> for Box<AsciiStr> {
    #[inline]
    fn from(owned: Box<[AsciiChar]>) -> Box<AsciiStr> {
        unsafe{ mem::transmute(owned) }
    }
}

macro_rules! impl_into {
    ($wider: ty) => {
        impl<'a> From<&'a AsciiStr> for &'a$wider {
            #[inline]
            fn from(slice: &AsciiStr) -> &$wider {
                unsafe{ mem::transmute(slice) }
            }
        }
        impl<'a> From<&'a mut AsciiStr> for &'a mut $wider {
            #[inline]
            fn from(slice: &mut AsciiStr) -> &mut $wider {
                unsafe{ mem::transmute(slice) }
            }
        }
        #[cfg(feature = "std")]
        impl From<Box<AsciiStr>> for Box<$wider> {
            #[inline]
            fn from(owned: Box<AsciiStr>) -> Box<$wider> {
                unsafe{ mem::transmute(owned) }
            }
        }
    }
}
impl_into! {[AsciiChar]}
impl_into! {[u8]}
impl_into! {str}

impl fmt::Display for AsciiStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Debug for AsciiStr {
    #[inline]
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

impl_index! { AsciiStr, usize, AsciiChar }
impl_index! { AsciiStr, Range<usize>, AsciiStr }
impl_index! { AsciiStr, RangeTo<usize>, AsciiStr }
impl_index! { AsciiStr, RangeFrom<usize>, AsciiStr }
impl_index! { AsciiStr, RangeFull, AsciiStr }

#[cfg(feature = "std")]
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
        self.slice.iter().zip(other.slice.iter()).all(|(a, b)| a.eq_ignore_ascii_case(b) )
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


/// Error that is returned when a sequence of `u8` are not all ASCII.
///
/// Is used by `As[Mut]AsciiStr` and the `from_ascii` method on `AsciiStr` and `AsciiString`.
#[derive(Clone,Copy, PartialEq,Eq, Debug)]
pub struct AsAsciiStrError (usize);

const ERRORMSG_STR: &'static str = "one or more bytes are not ASCII";

impl AsAsciiStrError {
    /// Returns the index of the first non-ASCII byte.
    ///
    /// It is the maximum index such that `from_ascii(input[..index])` would return `Ok(_)`.
    #[inline]
    pub fn valid_up_to(self) -> usize {
        self.0
    }
    #[cfg(not(feature = "std"))]
    /// Returns a description for this error, like `std::error::Error::description`.
    #[inline]
    pub fn description(&self) -> &'static str {
        ERRORMSG_STR
    }
}
impl fmt::Display for AsAsciiStrError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "the byte at index {} is not ASCII", self.0)
    }
}
#[cfg(feature = "std")]
impl Error for AsAsciiStrError {
    #[inline]
    fn description(&self) -> &'static str {
        ERRORMSG_STR
    }
}


/// Convert slices of bytes to `AsciiStr`.
pub trait AsAsciiStr {
    /// Convert to an ASCII slice without checking for non-ASCII characters.
    unsafe fn as_ascii_str_unchecked(&self) -> &AsciiStr;
    /// Convert to an ASCII slice.
    fn as_ascii_str(&self) -> Result<&AsciiStr,AsAsciiStrError>;
}

/// Convert mutable slices of bytes to `AsciiStr`.
pub trait AsMutAsciiStr {
    /// Convert to a mutable ASCII slice without checking for non-ASCII characters.
    unsafe fn as_mut_ascii_str_unchecked(&mut self) -> &mut AsciiStr;
    /// Convert to a mutable ASCII slice.
    fn as_mut_ascii_str(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError>;
}

impl AsAsciiStr for AsciiStr {
    #[inline]
    fn as_ascii_str(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        Ok(self)
    }
    #[inline]
    unsafe fn as_ascii_str_unchecked(&self) -> &AsciiStr {
        self
    }
}
impl AsMutAsciiStr for AsciiStr {
    #[inline]
    fn as_mut_ascii_str(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        Ok(self)
    }
    #[inline]
    unsafe fn as_mut_ascii_str_unchecked(&mut self) -> &mut AsciiStr {
        self
    }
}

impl AsAsciiStr for [AsciiChar] {
    #[inline]
    fn as_ascii_str(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        Ok(self.into())
    }
    #[inline]
    unsafe fn as_ascii_str_unchecked(&self) -> &AsciiStr {
        self.into()
    }
}
impl AsMutAsciiStr for [AsciiChar] {
    #[inline]
    fn as_mut_ascii_str(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        Ok(self.into())
    }
    #[inline]
    unsafe fn as_mut_ascii_str_unchecked(&mut self) -> &mut AsciiStr {
        self.into()
    }
}

impl AsAsciiStr for [u8] {
    fn as_ascii_str(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        match self.iter().position(|&b| b > 127 ) {
            Some(index) => Err(AsAsciiStrError(index)),
            None => unsafe{ Ok(self.as_ascii_str_unchecked()) },
        }
    }
    #[inline]
    unsafe fn as_ascii_str_unchecked(&self) -> &AsciiStr {
        mem::transmute(self)
    }
}
impl AsMutAsciiStr for [u8] {
    fn as_mut_ascii_str(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        match self.iter().position(|&b| b > 127 ) {
            Some(index) => Err(AsAsciiStrError(index)),
            None => unsafe{ Ok(self.as_mut_ascii_str_unchecked()) },
        }
    }
    #[inline]
    unsafe fn as_mut_ascii_str_unchecked(&mut self) -> &mut AsciiStr {
        mem::transmute(self)
    }
}

impl AsAsciiStr for str {
    fn as_ascii_str(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        self.as_bytes().as_ascii_str()
    }
    #[inline]
    unsafe fn as_ascii_str_unchecked(&self) -> &AsciiStr {
        self.as_bytes().as_ascii_str_unchecked()
    }
}
impl AsMutAsciiStr for str {
    fn as_mut_ascii_str(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        match self.bytes().position(|b| b > 127 ) {
            Some(index) => Err(AsAsciiStrError(index)),
            None => unsafe{ Ok(self.as_mut_ascii_str_unchecked()) },
        }
    }
    #[inline]
    unsafe fn as_mut_ascii_str_unchecked(&mut self) -> &mut AsciiStr {
        mem::transmute(self)
    }
}


#[cfg(test)]
mod tests {
    use AsciiChar;
    use super::{AsciiStr, AsAsciiStr, AsMutAsciiStr, AsAsciiStrError};
    #[cfg(feature = "std")]
    use std::ascii::AsciiExt;

    #[test]
    fn generic_as_ascii_str() {
        fn generic<C:AsAsciiStr+?Sized>(c: &C) -> Result<&AsciiStr,AsAsciiStrError> {
            c.as_ascii_str()
        }
        let arr = [AsciiChar::A];
        let ascii_str: &AsciiStr = arr.as_ref().into();
        assert_eq!(generic("A"), Ok(ascii_str));
        assert_eq!(generic(&b"A"[..]), Ok(ascii_str));
        assert_eq!(generic(ascii_str), Ok(ascii_str));
    }

    #[test]
    #[cfg(feature = "std")]
    fn as_ascii_str() {
        macro_rules! err {{$i:expr} => {Err(AsAsciiStrError($i))}}
        let mut s: String = "abčd".to_string();
        let mut b: Vec<u8> = s.clone().into();
        assert_eq!(s.as_str().as_ascii_str(), err!(2));
        assert_eq!(s.as_mut_str().as_mut_ascii_str(), err!(2));
        assert_eq!(b.as_slice().as_ascii_str(), err!(2));
        assert_eq!(b.as_mut_slice().as_mut_ascii_str(), err!(2));
        let mut a = [AsciiChar::a, AsciiChar::b];
        assert_eq!((&s[..2]).as_ascii_str(), Ok((&a[..]).into()));
        assert_eq!((&b[..2]).as_ascii_str(), Ok((&a[..]).into()));
        let a = Ok((&mut a[..]).into());
        assert_eq!((&mut s[..2]).as_mut_ascii_str(), a);
        assert_eq!((&mut b[..2]).as_mut_ascii_str(), a);
    }

    #[test]
    fn default() {
        let default: &'static AsciiStr = Default::default();
        assert!(default.is_empty());
    }

    #[test]
    fn as_str() {
        let b = b"( ;";
        let v = AsciiStr::from_ascii(b).unwrap();
        assert_eq!(v.as_str(), "( ;");
        assert_eq!(AsRef::<str>::as_ref(v), "( ;");
    }

    #[test]
    fn as_bytes() {
        let b = b"( ;";
        let v = AsciiStr::from_ascii(b).unwrap();
        assert_eq!(v.as_bytes(), b"( ;");
        assert_eq!(AsRef::<[u8]>::as_ref(v), b"( ;");
    }

    #[test]
    fn ascii_case() {
        let mut bytes = ([b'a',b'@',b'A'], [b'A',b'@',b'a']);
        let mut a = bytes.0.as_mut_ascii_str().unwrap();
        let mut b = bytes.1.as_mut_ascii_str().unwrap();
        assert!(a.eq_ignore_ascii_case(b));
        assert!(b.eq_ignore_ascii_case(a));
        a.make_ascii_lowercase();
        b.make_ascii_uppercase();
        assert_eq!(a, "a@a");
        assert_eq!(b, "A@A");
    }

    #[test]
    #[cfg(feature = "std")]
    fn fmt_ascii_str() {
        let s = "abc".as_ascii_str().unwrap();
        assert_eq!(format!("{}", s), "abc".to_string());
        assert_eq!(format!("{:?}", s), "\"abc\"".to_string());
    }
}
