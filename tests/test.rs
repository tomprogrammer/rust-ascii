extern crate ascii;

use ascii::{Ascii, AsciiStr, AsciiString, AsAsciiStr, IntoAsciiString};

#[test]
fn ascii_vec() {
    let test = b"( ;";
    let a = AsciiStr::from_bytes(test).unwrap();
    assert_eq!(test.as_ascii_str(), Ok(a));
    assert_eq!("( ;".as_ascii_str(), Ok(a));
    let v = test.to_vec();
    assert_eq!(v.as_ascii_str(), Ok(a));
    assert_eq!("( ;".to_string().as_ascii_str(), Ok(a));
}

#[test]
fn to_ascii() {
    assert!("zoä华".as_ascii_str().is_err());
    assert!([127_u8, 128, 255].as_ascii_str().is_err());

    let arr = [Ascii::ParenOpen, Ascii::Space, Ascii::Semicolon];
    let a: &AsciiStr = (&arr[..]).into();
    assert_eq!(b"( ;".as_ascii_str(), Ok(a));
    assert_eq!("( ;".as_ascii_str(), Ok(a));

    assert_eq!("zoä华".to_string().into_ascii_string(), Err("zoä华".to_string()));
    assert_eq!(vec![127_u8, 128, 255].into_ascii_string(), Err(vec![127_u8, 128, 255]));

    let v = AsciiString::from(arr.to_vec());
    assert_eq!(b"( ;".to_vec().into_ascii_string(), Ok(v.clone()));
    assert_eq!("( ;".to_string().into_ascii_string(), Ok(v));
}

#[test]
fn compare_ascii_string_ascii_str() {
    let v = b"abc";
    let ascii_string = AsciiString::from_ascii(&v[..]).unwrap();
    let ascii_str = AsciiStr::from_bytes(v).unwrap();
    assert!(ascii_string == ascii_str);
    assert!(ascii_str == ascii_string);
}

#[test]
fn compare_ascii_string_string() {
    let v = b"abc";
    let string = String::from_utf8(v.to_vec()).unwrap();
    let ascii_string = AsciiString::from_ascii(&v[..]).unwrap();
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
    let ascii_string = AsciiString::from_ascii(&v[..]).unwrap();
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
    let b = b"abc".as_ascii_str().unwrap();
    let c = b"ab".as_ascii_str().unwrap();
    assert_eq!(&b[..2], &c[..]);
    assert_eq!(c[1].as_char(), 'b');
}

#[test]
fn compare_ascii_string_slice() {
    let b = AsciiString::from_ascii("abc").unwrap();
    let c = AsciiString::from_ascii("ab").unwrap();
    assert_eq!(&b[..2], &c[..]);
    assert_eq!(c[1].as_char(), 'b');
}
