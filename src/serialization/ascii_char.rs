use std::fmt;

use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use ascii_char::AsciiChar;

impl Serialize for AsciiChar {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_char(self.as_char())
    }
}

struct AsciiCharVisitor;

impl<'de> Visitor<'de> for AsciiCharVisitor {
    type Value = AsciiChar;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an ascii character")
    }

    #[inline]
    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        AsciiChar::from(v).map_err(|_| Error::invalid_value(Unexpected::Char(v), &self))
    }

    #[inline]
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut iter = v.chars();
        match (iter.next(), iter.next()) {
            (Some(c), None) => self.visit_char(c),
            _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for AsciiChar {
    fn deserialize<D>(deserializer: D) -> Result<AsciiChar, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_char(AsciiCharVisitor)
    }
}

#[cfg(all(test, feature = "serde_test"))]
mod tests {
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};

    use super::*;

    const ASCII_CHAR: char = 'e';
    const ASCII_STR: &str = "e";
    const UNICODE_CHAR: char = 'é';

    #[test]
    fn serialize() {
        let ascii_char = AsciiChar::from(ASCII_CHAR).unwrap();
        assert_tokens(&ascii_char, &[Token::Char(ASCII_CHAR)]);
    }

    #[test]
    fn deserialize() {
        let ascii_char = AsciiChar::from(ASCII_CHAR).unwrap();
        assert_de_tokens(&ascii_char, &[Token::String(ASCII_STR)]);
        assert_de_tokens(&ascii_char, &[Token::Str(ASCII_STR)]);
        assert_de_tokens(&ascii_char, &[Token::BorrowedStr(ASCII_STR)]);
        assert_de_tokens_error::<AsciiChar>(
            &[Token::Char(UNICODE_CHAR)],
            "invalid value: character `é`, expected an ascii character",
        );
    }
}
