extern crate core;

use self::core::mem::transmute;
use self::core::fmt;
#[cfg(not(feature = "no_std"))]
use std::error::Error;
#[cfg(not(feature = "no_std"))]
use std::ascii::AsciiExt;

#[allow(non_camel_case_types)]
/// An ASCII character. It wraps a `u8`, with the highest bit always zero.
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Copy)]
#[repr(u8)]
pub enum AsciiChar {
    /// `'\0'`
    Null            =   0,
    /// [Start Of Heading](http://en.wikipedia.org/wiki/Start_of_Heading)
    SOH             =   1,
    /// [Start Of teXt](http://en.wikipedia.org/wiki/Start_of_Text)
    SOX             =   2,
    /// [End of TeXt](http://en.wikipedia.org/wiki/End-of-Text_character)
    ETX             =   3,
    /// [End Of Transmission](http://en.wikipedia.org/wiki/End-of-Transmission_character)
    EOT             =   4,
    /// [Enquiry](http://en.wikipedia.org/wiki/Enquiry_character)
    ENQ             =   5,
    /// [Acknowledgement](http://en.wikipedia.org/wiki/Acknowledge_character)
    ACK             =   6,
    /// [bell / alarm / audible](http://en.wikipedia.org/wiki/Bell_character)
    ///
    /// `'\a'` is not recognized by Rust.
    Bell            =   7,
    /// [Backspace](http://en.wikipedia.org/wiki/Backspace)
    ///
    /// `'\b'` is not recognized by Rust.
    BackSpace       =   8,
    /// `'\t'`
    Tab             =   9,
    /// `'\n'`
    LineFeed        =  10,
    /// [Vertical tab](http://en.wikipedia.org/wiki/Vertical_Tab)
    ///
    /// `'\v'` is not recognized by Rust.
    VT              =  11,
    /// [Form Feed](http://en.wikipedia.org/wiki/Form_Feed)
    ///
    /// `'\f'` is not recognized by Rust.
    FF              =  12,
    /// `'\r'`
    CarriageReturn  =  13,
    /// [Shift In](http://en.wikipedia.org/wiki/Shift_Out_and_Shift_In_characters)
    SI              =  14,
    /// [Shift Out](http://en.wikipedia.org/wiki/Shift_Out_and_Shift_In_characters)
    SO              =  15,
    /// [Data Link Escape](http://en.wikipedia.org/wiki/Data_Link_Escape)
    DLE             =  16,
    /// [Device control 1, often XON](http://en.wikipedia.org/wiki/Device_Control_1)
    DC1             =  17,
    /// Device control 2
    DC2             =  18,
    /// Device control 3, Often XOFF
    DC3             =  19,
    /// Device control 4
    DC4             =  20,
    /// [Negative AcKnowledgement](http://en.wikipedia.org/wiki/Negative-acknowledge_character)
    NAK             =  21,
    /// [Synchronous idle](http://en.wikipedia.org/wiki/Synchronous_Idle)
    SYN             =  22,
    /// [End of Transmission Block](http://en.wikipedia.org/wiki/End-of-Transmission-Block_character)
    ETB             =  23,
    /// [Cancel](http://en.wikipedia.org/wiki/Cancel_character)
    CAN             =  24,
    /// [End of Medium](http://en.wikipedia.org/wiki/End_of_Medium)
    EM              =  25,
    /// [Substitute](http://en.wikipedia.org/wiki/Substitute_character)
    SUB             =  26,
    /// [Escape](http://en.wikipedia.org/wiki/Escape_character)
    ///
    /// `'\e'` is not recognized by Rust.
    ESC             =  27,
    /// [File Separator](http://en.wikipedia.org/wiki/File_separator)
    FS              =  28,
    /// [Group Separator](http://en.wikipedia.org/wiki/Group_separator)
    GS              =  29,
    /// [Record Separator](http://en.wikipedia.org/wiki/Record_separator)
    RS              =  30,
    /// [Unit Separator](http://en.wikipedia.org/wiki/Unit_separator)
    US              =  31,
    /// `' '`
    Space           =  32,
    /// `'!'`
    Exclamation     =  33,
    /// `'"'`
    Quotation       =  34,
    /// `'#'`
    Hash            =  35,
    /// `'$'`
    Dollar          =  36,
    /// `'%'`
    Percent         =  37,
    /// `'&'`
    Ampersand       =  38,
    /// `'\''`
    Apostrophe      =  39,
    /// `'('`
    ParenOpen       =  40,
    /// `')'`
    ParenClose      =  41,
    /// `'*'`
    Asterisk        =  42,
    /// `'+'`
    Plus            =  43,
    /// `','`
    Comma           =  44,
    /// `'-'`
    Minus           =  45,
    /// `'.'`
    Dot             =  46,
    /// `'/'`
    Slash           =  47,
    /// `'0'`
    _0              =  48,
    /// `'1'`
    _1              =  49,
    /// `'2'`
    _2              =  50,
    /// `'3'`
    _3              =  51,
    /// `'4'`
    _4              =  52,
    /// `'5'`
    _5              =  53,
    /// `'6'`
    _6              =  54,
    /// `'7'`
    _7              =  55,
    /// `'8'`
    _8              =  56,
    /// `'9'`
    _9              =  57,
    /// `':'`
    Colon           =  58,
    /// `';'`
    Semicolon       =  59,
    /// `'<'`
    LessThan        =  60,
    /// `'='`
    Equal           =  61,
    /// `'>'`
    GreaterThan     =  62,
    /// `'?'`
    Question        =  63,
    /// `'@'`
    At              =  64,
    /// `'A'`
    A               =  65,
    /// `'B'`
    B               =  66,
    /// `'C'`
    C               =  67,
    /// `'D'`
    D               =  68,
    /// `'E'`
    E               =  69,
    /// `'F'`
    F               =  70,
    /// `'G'`
    G               =  71,
    /// `'H'`
    H               =  72,
    /// `'I'`
    I               =  73,
    /// `'J'`
    J               =  74,
    /// `'K'`
    K               =  75,
    /// `'L'`
    L               =  76,
    /// `'M'`
    M               =  77,
    /// `'N'`
    N               =  78,
    /// `'O'`
    O               =  79,
    /// `'P'`
    P               =  80,
    /// `'Q'`
    Q               =  81,
    /// `'R'`
    R               =  82,
    /// `'S'`
    S               =  83,
    /// `'T'`
    T               =  84,
    /// `'U'`
    U               =  85,
    /// `'V'`
    V               =  86,
    /// `'W'`
    W               =  87,
    /// `'X'`
    X               =  88,
    /// `'Y'`
    Y               =  89,
    /// `'Z'`
    Z               =  90,
    /// `'['`
    BracketOpen     =  91,
    /// `'\'`
    BackSlash       =  92,
    /// `']'`
    BracketClose    =  93,
    /// `'_'`
    Caret           =  94,
    /// `'_'`
    UnderScore      =  95,
    /// `'`'`
    Grave           =  96,
    /// `'a'`
    a               =  97,
    /// `'b'`
    b               =  98,
    /// `'c'`
    c               =  99,
    /// `'d'`
    d               = 100,
    /// `'e'`
    e               = 101,
    /// `'f'`
    f               = 102,
    /// `'g'`
    g               = 103,
    /// `'h'`
    h               = 104,
    /// `'i'`
    i               = 105,
    /// `'j'`
    j               = 106,
    /// `'k'`
    k               = 107,
    /// `'l'`
    l               = 108,
    /// `'m'`
    m               = 109,
    /// `'n'`
    n               = 110,
    /// `'o'`
    o               = 111,
    /// `'p'`
    p               = 112,
    /// `'q'`
    q               = 113,
    /// `'r'`
    r               = 114,
    /// `'s'`
    s               = 115,
    /// `'t'`
    t               = 116,
    /// `'u'`
    u               = 117,
    /// `'v'`
    v               = 118,
    /// `'w'`
    w               = 119,
    /// `'x'`
    x               = 120,
    /// `'y'`
    y               = 121,
    /// `'z'`
    z               = 122,
    /// `'{'`
    CurlyBraceOpen  = 123,
    /// `'|'`
    VerticalBar     = 124,
    /// `'}'`
    CurlyBraceClose = 125,
    /// `'~'`
    Tilde           = 126,
    /// [Delete](http://en.wikipedia.org/wiki/Delete_character)
    DEL             = 127,
}

impl AsciiChar {
    /// Constructs an ASCII character from a `u8`, `char` or other character type.
    ///
    /// # Failure
    /// Returns `Err(())` if the character can't be ASCII encoded.
    ///
    /// # Example
    /// ```
    /// # use ascii::AsciiChar;
    /// let a = AsciiChar::from('g').unwrap();
    /// assert_eq!(a.as_char(), 'g');
    /// ```
    #[inline]
    pub fn from<C:ToAsciiChar>(ch: C) -> Result<Self, ToAsciiCharError> {
        ch.to_ascii_char()
    }

    /// Constructs an ASCII character from a `char` or `u8` without any checks.
    pub unsafe fn from_unchecked<C:ToAsciiChar>(ch: C) -> Self {
        ch.to_ascii_char_unchecked()
    }

    /// Converts an ASCII character into a `u8`.
    #[inline]
    pub fn as_byte(&self) -> u8 {
        *self as u8
    }

    /// Converts an ASCII character into a `char`.
    #[inline]
    pub fn as_char(&self) -> char {
        self.as_byte() as char
    }

    // the following methods are like ctype, and the implementation is inspired by musl

    /// Check if the character is a letter (a-z, A-Z)
    #[inline]
    pub fn is_alphabetic(&self) -> bool {
        let c = self.as_byte() | 0b010_0000;// Turns uppercase into lowercase.
        c >= b'a' && c <= b'z'
    }

    /// Check if the character is a number (0-9)
    #[inline]
    pub fn is_digit(&self) -> bool {
        self >= &AsciiChar::_0 && self <= &AsciiChar::_9
    }

    /// Check if the character is a letter or number
    #[inline]
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Check if the character is a space or horizontal tab
    #[inline]
    pub fn is_blank(&self) -> bool {
        *self == AsciiChar::Space || *self == AsciiChar::Tab
    }

    /// Check if the character is a ' ', '\t', '\n' or '\r'
    #[inline]
    pub fn is_whitespace(&self) -> bool {
        self.is_blank() || *self == AsciiChar::LineFeed
                        || *self == AsciiChar::CarriageReturn
    }

    /// Check if the character is a control character
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('\0'.to_ascii_char().unwrap().is_control(), true);
    /// assert_eq!('n'.to_ascii_char().unwrap().is_control(), false);
    /// assert_eq!(' '.to_ascii_char().unwrap().is_control(), false);
    /// assert_eq!('\n'.to_ascii_char().unwrap().is_control(), true);
    /// ```
    #[inline]
    pub fn is_control(&self) -> bool {
        *self < AsciiChar::Space || *self == AsciiChar::DEL
    }

    /// Checks if the character is printable (except space)
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('n'.to_ascii_char().unwrap().is_graph(), true);
    /// assert_eq!(' '.to_ascii_char().unwrap().is_graph(), false);
    /// assert_eq!('\n'.to_ascii_char().unwrap().is_graph(), false);
    /// ```
    #[inline]
    pub fn is_graph(&self) -> bool {
        self.as_byte().wrapping_sub(b' '+1) < 0x5E
    }

    /// Checks if the character is printable (including space)
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('n'.to_ascii_char().unwrap().is_print(), true);
    /// assert_eq!(' '.to_ascii_char().unwrap().is_print(), true);
    /// assert_eq!('\n'.to_ascii_char().unwrap().is_print(), false);
    /// ```
    #[inline]
    pub fn is_print(&self) -> bool {
        self.as_byte().wrapping_sub(b' ') < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('a'.to_ascii_char().unwrap().is_lowercase(), true);
    /// assert_eq!('A'.to_ascii_char().unwrap().is_lowercase(), false);
    /// assert_eq!('@'.to_ascii_char().unwrap().is_lowercase(), false);
    /// ```
    #[inline]
    pub fn is_lowercase(&self) -> bool {
        self.as_byte().wrapping_sub(b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('A'.to_ascii_char().unwrap().is_uppercase(), true);
    /// assert_eq!('a'.to_ascii_char().unwrap().is_uppercase(), false);
    /// assert_eq!('@'.to_ascii_char().unwrap().is_uppercase(), false);
    /// ```
    #[inline]
    pub fn is_uppercase(&self) -> bool {
        self.as_byte().wrapping_sub(b'A') < 26
    }

    /// Checks if the character is punctuation
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('n'.to_ascii_char().unwrap().is_punctuation(), false);
    /// assert_eq!(' '.to_ascii_char().unwrap().is_punctuation(), false);
    /// assert_eq!('_'.to_ascii_char().unwrap().is_punctuation(), true);
    /// assert_eq!('~'.to_ascii_char().unwrap().is_punctuation(), true);
    /// ```
    #[inline]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    ///
    /// # Examples
    /// ```
    /// use ascii::ToAsciiChar;
    /// assert_eq!('5'.to_ascii_char().unwrap().is_hex(), true);
    /// assert_eq!('a'.to_ascii_char().unwrap().is_hex(), true);
    /// assert_eq!('F'.to_ascii_char().unwrap().is_hex(), true);
    /// assert_eq!('G'.to_ascii_char().unwrap().is_hex(), false);
    /// assert_eq!(' '.to_ascii_char().unwrap().is_hex(), false);
    /// ```
    #[inline]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || (self.as_byte() | 0x20u8).wrapping_sub(b'a') < 6
    }
}

impl fmt::Display for AsciiChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
    }
}

impl fmt::Debug for AsciiChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
     }
}

#[cfg(not(feature = "no_std"))]
impl AsciiExt for AsciiChar {
    type Owned = AsciiChar;

    #[inline]
    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> AsciiChar {
        unsafe{ self.as_byte().to_ascii_uppercase().to_ascii_char_unchecked() }
    }

    fn to_ascii_lowercase(&self) -> AsciiChar {
        unsafe{ self.as_byte().to_ascii_lowercase().to_ascii_char_unchecked() }
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.as_byte().eq_ignore_ascii_case(&other.as_byte())
    }

    #[inline]
    fn make_ascii_uppercase(&mut self) {
        *self = self.to_ascii_uppercase();
    }

    #[inline]
    fn make_ascii_lowercase(&mut self) {
        *self = self.to_ascii_lowercase();
    }
}


/// Error returned by `ToAsciiChar`.
#[derive(PartialEq)]
pub struct ToAsciiCharError(());

const ERRORMSG_CHAR: &'static str = "not an ASCII character";

impl fmt::Debug for ToAsciiCharError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", ERRORMSG_CHAR)
    }
}

impl fmt::Display for ToAsciiCharError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", ERRORMSG_CHAR)
    }
}

#[cfg(not(feature = "no_std"))]
impl Error for ToAsciiCharError {
    fn description(&self) -> &'static str {
        ERRORMSG_CHAR
    }
}

/// Convert `char`, `u8` and other character types to `AsciiChar`.
pub trait ToAsciiChar {
    /// Convert to `AsciiChar` without checking that it is an ASCII character.
    unsafe fn to_ascii_char_unchecked(self) -> AsciiChar;
    /// Convert to `AsciiChar`.
    fn to_ascii_char(self) -> Result<AsciiChar, ToAsciiCharError>;
}

impl ToAsciiChar for AsciiChar {
    fn to_ascii_char(self) -> Result<AsciiChar, ToAsciiCharError> {
        Ok(self)
    }
    unsafe fn to_ascii_char_unchecked(self) -> AsciiChar {
        self
    }
}

impl ToAsciiChar for u8 {
    fn to_ascii_char(self) -> Result<AsciiChar, ToAsciiCharError> {
        unsafe{ if self <= 0x7F {
            return Ok(self.to_ascii_char_unchecked());
        }}
        Err(ToAsciiCharError(()))
    }
    unsafe fn to_ascii_char_unchecked(self) -> AsciiChar {
        transmute(self)
    }
}

impl ToAsciiChar for char {
    fn to_ascii_char(self) -> Result<AsciiChar, ToAsciiCharError> {
        unsafe{ if self as u32 <= 0x7F {
            return Ok(self.to_ascii_char_unchecked());
        }}
        Err(ToAsciiCharError(()))
    }
    unsafe fn to_ascii_char_unchecked(self) -> AsciiChar {
        (self as u8).to_ascii_char_unchecked()
    }
}


#[cfg(test)]
mod tests {
    use super::{AsciiChar, ToAsciiChar, ToAsciiCharError};
    use AsciiChar::*;
    #[cfg(not(feature = "no_std"))]
    use std::ascii::AsciiExt;

    #[test]
    fn to_ascii_char() {
        fn generic<C:ToAsciiChar>(ch: C) -> Result<AsciiChar, ToAsciiCharError> {
            ch.to_ascii_char()
        }
        assert_eq!(generic(A), Ok(A));
        assert_eq!(generic(b'A'), Ok(A));
        assert_eq!(generic('A'), Ok(A));
        assert!(generic(200).is_err());
        assert!(generic('λ').is_err());
    }

    #[test]
    fn as_byte_and_char() {
        assert_eq!(A.as_byte(), b'A');
        assert_eq!(A.as_char(),  'A');
    }

    #[test]
    fn is_digit() {
        assert_eq!('0'.to_ascii_char().unwrap().is_digit(), true);
        assert_eq!('9'.to_ascii_char().unwrap().is_digit(), true);
        assert_eq!('/'.to_ascii_char().unwrap().is_digit(), false);
        assert_eq!(':'.to_ascii_char().unwrap().is_digit(), false);
    }

    #[test]
    fn is_control() {
        assert_eq!(US.is_control(), true);
        assert_eq!(DEL.is_control(), true);
        assert_eq!(Space.is_control(), false);
    }

    #[test]
    #[cfg(not(feature = "no_std"))]
    fn ascii_case() {
        assert_eq!(At.to_ascii_lowercase(), At);
        assert_eq!(At.to_ascii_uppercase(), At);
        assert_eq!(A.to_ascii_lowercase(), a);
        assert_eq!(A.to_ascii_uppercase(), A);
        assert_eq!(a.to_ascii_lowercase(), a);
        assert_eq!(a.to_ascii_uppercase(), A);

        assert!(LineFeed.eq_ignore_ascii_case(&LineFeed));
        assert!(!LineFeed.eq_ignore_ascii_case(&CarriageReturn));
        assert!(z.eq_ignore_ascii_case(&Z));
        assert!(Z.eq_ignore_ascii_case(&z));
        assert!(!Z.eq_ignore_ascii_case(&DEL));
    }

    #[test]
    #[cfg(not(feature = "no_std"))]
    fn fmt_ascii() {
        assert_eq!(format!("{}", t), "t".to_string());
        assert_eq!(format!("{:?}", t), "'t'".to_string());
    }
}
