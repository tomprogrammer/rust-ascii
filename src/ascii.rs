use std::fmt;
use std::ascii::AsciiExt;

use AsciiCast;

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

    /// Constructs an Ascii character from a `u8`.
    ///
    /// # Failure
    ///
    /// Returns `Err(())` if the character can't be ascii encoded.
    ///
    /// # Example
    ///
    /// ```
    /// # use ascii::Ascii;
    /// let a = Ascii::from_byte(65).unwrap();
    /// assert_eq!(a.as_char(), 'A');
    /// ```
    #[inline]
    pub fn from_byte(ch: u8) -> Result<Ascii, ()> {
        if ch <= 0x7F {
            return Ok( Ascii { chr: ch });
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
        self.chr.wrapping_sub(0x21) < 0x5E
    }

    /// Checks if the character is printable (including space)
    #[inline]
    pub fn is_print(&self) -> bool {
        self.chr.wrapping_sub(0x20) < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    #[inline]
    pub fn is_lowercase(&self) -> bool {
        self.chr.wrapping_sub(b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    #[inline]
    pub fn is_uppercase(&self) -> bool {
        self.chr.wrapping_sub(b'A') < 26
    }

    /// Checks if the character is punctuation
    #[inline]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    #[inline]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || (self.chr | 32u8).wrapping_sub(b'a') < 6
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

#[cfg(test)]
mod tests {
    use AsciiCast;
    use super::Ascii;

    #[test]
    fn to_ascii() {
        assert_eq!(65_u8.to_ascii(), Ok(Ascii { chr: 65_u8 }));
        assert_eq!(255_u8.to_ascii(), Err(()));

        assert_eq!('A'.to_ascii(), Ok(Ascii { chr: 65_u8 }));
        assert_eq!('λ'.to_ascii(), Err(()));
    }

    #[test]
    fn as_byte() {
        assert_eq!(65u8.to_ascii().unwrap().as_byte(), 65u8);
        assert_eq!('A'.to_ascii().unwrap().as_byte(), 65u8);
    }

    #[test]
    fn as_char() {
        assert_eq!(65u8.to_ascii().unwrap().as_char(), 'A');
        assert_eq!('A'.to_ascii().unwrap().as_char(), 'A');
    }

    #[test]
    fn is_digit() {
        assert!('0'.to_ascii().unwrap().is_digit());
        assert!('9'.to_ascii().unwrap().is_digit());
        assert!(!'/'.to_ascii().unwrap().is_digit());
        assert!(!':'.to_ascii().unwrap().is_digit());
    }

    #[test]
    fn is_control() {
        assert!(0x1f_u8.to_ascii().unwrap().is_control());
        assert!(0x7f_u8.to_ascii().unwrap().is_control());
        assert!(!' '.to_ascii().unwrap().is_control());
    }

    #[test]
    fn fmt_display_ascii() {
        let s = Ascii { chr: b't' };
        assert_eq!(format!("{}", s), "t".to_string());
    }

    #[test]
    fn fmt_debug_ascii() {
        let c = Ascii { chr: b't' };
        assert_eq!(format!("{:?}", c), "'t'".to_string());
    }
}
