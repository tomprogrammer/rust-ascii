use std::{fmt, mem};
use std::ops::{Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
use std::error::Error;
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
                Ok( Self::from_bytes_unchecked(bytes) )
            } else {
                Err(())
            }
        }
    }

    /// Converts anything that can be represented as a byte slice to an `AsciiStr` without checking for non-ASCII characters..
    ///
    /// # Examples
    /// ```
    /// # use ascii::AsciiStr;
    /// let foo = unsafe{ AsciiStr::from_bytes_unchecked("foo") };
    /// assert_eq!(foo.as_str(), "foo");
    /// ```
    pub unsafe fn from_bytes_unchecked<'a, B: ?Sized>(bytes: &'a B) -> &'a AsciiStr
        where B: AsRef<[u8]>
    {
        mem::transmute(bytes.as_ref())
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

impl Default for &'static AsciiStr {
    fn default() -> &'static AsciiStr {
        unsafe{ "".as_ascii_unchecked() }
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


/// Error returned by `AsAsciiStr`
#[derive(Clone,Copy)]
pub struct AsAsciiStrError {
    index: usize,
    /// If less than 128, it was a byte >= 128 and not from a str
    not_ascii: char,
}

impl AsAsciiStrError {
    /// Get the index of the first non-ASCII byte or character.
    pub fn index(self) -> usize {
        self.index
    }

    /// Get the non-ASCII byte that caused the conversion to fail.
    ///
    /// If it was a `str` that was being converted, the first byte in the utf8 encoding is returned.
    pub fn byte(self) -> u8 {
        if (self.not_ascii as u32) < 128 {
            self.not_ascii as u8 + 128
        } else {
            // FIXME: use char::encode_utf8() when stabilized.
            let mut s = String::with_capacity(4);
            s.push(self.not_ascii);
            s.bytes().next().unwrap()
        }
    }

    /// Get the character that caused the conversion from a `str` to fail.
    ///
    /// Returns `None` if the error was caused by a byte in a `[u8]`
    pub fn char(self) -> Option<char> {
        match self.not_ascii as u32 {
            0...127 => None, // byte in a [u8]
               _    => Some(self.not_ascii),
        }
    }
}

impl fmt::Debug for AsAsciiStrError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        if (self.not_ascii as u32) < 128 {
            write!(fmtr, "b'\\x{:x}' at index {}", self.not_ascii as u8 + 128, self.index)
        } else {
            write!(fmtr, "'{}' at index {}", self.not_ascii, self.index)
        }
    }
}

impl fmt::Display for AsAsciiStrError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        if (self.not_ascii as u32) < 128 {
            write!(fmtr, "the byte \\x{:x} at index {} is not ASCII", self.not_ascii as u8 + 128, self.index)
        } else {
            write!(fmtr, "the character {} at index {} is not ASCII", self.not_ascii, self.index)
        }
    }
}

impl Error for AsAsciiStrError {
    fn description(&self) -> &'static str {
        if (self.not_ascii as u32) < 128 {
            "one or more bytes are not ASCII"
        } else {
            "one or more characters are not ASCII"
        }
    }
}


/// Connvert mutable slices of bytes to `AsciiStr`.
pub trait AsAsciiStr : AsciiExt {
    /// Convert to an ASCII slice without checking for non-ASCII characters.
    unsafe fn as_ascii_unchecked(&self) -> &AsciiStr;
    /// Convert to an ASCII slice.
    fn as_ascii(&self) -> Result<&AsciiStr,AsAsciiStrError>;
}
/// Connvert mutable slices of bytes to `AsciiStr`.
pub trait AsMutAsciiStr : AsciiExt {
    /// Convert to a mutable ASCII slice without checking for non-ASCII characters.
    unsafe fn as_mut_ascii_unchecked(&mut self) -> &mut AsciiStr;
    /// Convert to a mutable ASCII slice.
    fn as_mut_ascii(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError>;
}

#[cfg(feature = "unstable")]
impl AsAsciiStr for AsciiStr {
    fn as_ascii(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        Ok(self)
    }
    unsafe fn as_ascii_unchecked(&self) -> &AsciiStr {
        self
    }
}
#[cfg(feature = "unstable")]
impl AsMutAsciiStr for AsciiStr {
    fn as_mut_ascii(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        Ok(self)
    }
    unsafe fn as_mut_ascii_unchecked(&mut self) -> &mut AsciiStr {
        self
    }
}

// Cannot implement for [Ascii] since AsciiExt isn't implementet for it.

impl AsAsciiStr for [u8] {
    fn as_ascii(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        match self.iter().enumerate().find(|&(_,b)| *b > 127 ) {
            Some((index, &byte)) => Err(AsAsciiStrError{
                                            index: index,
                                            not_ascii: (byte - 128) as char,
                                        }),
            None => unsafe{ Ok(self.as_ascii_unchecked()) },
        }
    }
    unsafe fn as_ascii_unchecked(&self) -> &AsciiStr {
        AsciiStr::from_bytes_unchecked(self)
    }
}
impl AsMutAsciiStr for [u8] {
    fn as_mut_ascii(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        match self.iter().enumerate().find(|&(_,b)| *b > 127 ) {
            Some((index, &byte)) => Err(AsAsciiStrError{
                                            index: index,
                                            not_ascii: (byte - 128) as char,
                                        }),
            None => unsafe{ Ok(self.as_mut_ascii_unchecked()) },
        }
    }
    unsafe fn as_mut_ascii_unchecked(&mut self) -> &mut AsciiStr {
        mem::transmute(self)
    }
}

impl AsAsciiStr for str {
    fn as_ascii(&self) -> Result<&AsciiStr,AsAsciiStrError> {
        self.as_bytes().as_ascii().map_err(|err| AsAsciiStrError{
            not_ascii: self[err.index..].chars().next().unwrap(),
            index: err.index,
        })
    }
    unsafe fn as_ascii_unchecked(&self) -> &AsciiStr {
        mem::transmute(self)
    }
}
impl AsMutAsciiStr for str {
    fn as_mut_ascii(&mut self) -> Result<&mut AsciiStr,AsAsciiStrError> {
        match self.bytes().position(|b| b > 127 ) {
            Some(index) => Err(AsAsciiStrError{
                                   index: index,
                                   not_ascii: self[index..].chars().next().unwrap(),
                               }),
            None => unsafe{ Ok(self.as_mut_ascii_unchecked()) },
        }
    }
    unsafe fn as_mut_ascii_unchecked(&mut self) -> &mut AsciiStr {
        mem::transmute(self)
    }
}


#[cfg(test)]
mod tests {
    use {AsciiCast,Ascii};
    use super::{AsciiStr,AsAsciiStr,AsMutAsciiStr,AsAsciiStrError};

    /// Make Result<_,AsAsciiError> comparable.
    pub fn tuplify<T>(r: Result<T,AsAsciiStrError>) -> Result<T,(usize,char)> {
        r.map_err(|e| (e.index, e.not_ascii) )
    }

    #[test]
    fn generic_as_ascii() {
        fn generic<C:AsAsciiStr+?Sized>(c: &C) -> Result<&AsciiStr,AsAsciiStrError> {
            c.as_ascii()
        }
        let arr = [Ascii::A];
        let ascii_str = arr.as_ref().into();
        assert_eq!(tuplify(generic("A")), Ok(ascii_str));
        assert_eq!(tuplify(generic(&b"A"[..])), Ok(ascii_str));
        //assert_eq!(generic(ascii_str), Ok(ascii_str));
    }

    #[test]
    fn as_ascii() {
        let mut s: String = "abčd".to_string();
        let mut b: Vec<u8> = s.clone().into();
        assert_eq!(tuplify(s.as_str().as_ascii()), Err((2,'č')));
        assert_eq!(tuplify(s.as_mut_str().as_mut_ascii()), Err((2,'č')));
        let c = (b[2]-128) as char;
        assert_eq!(tuplify(b.as_slice().as_ascii()), Err((2,c)));
        assert_eq!(tuplify(b.as_mut_slice().as_mut_ascii()), Err((2,c)));
        let mut a = [Ascii::a, Ascii::b];
        assert_eq!(tuplify((&s[..2]).as_ascii()), Ok((&a[..]).into()));
        assert_eq!(tuplify((&b[..2]).as_ascii()), Ok((&a[..]).into()));
        let a = Ok((&mut a[..]).into());
        assert_eq!(tuplify((&mut s[..2]).as_mut_ascii()), a);
        assert_eq!(tuplify((&mut b[..2]).as_mut_ascii()), a);
    }

    #[test]
    fn as_ascii_error() {
        let s = "abčd".as_ascii().unwrap_err();
        let b = "abčd".as_bytes().as_ascii().unwrap_err();
        assert_eq!(s.char(), Some('č'));
        assert_eq!(b.char(), None);
        assert_eq!(s.byte(), b.byte());
    }

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
