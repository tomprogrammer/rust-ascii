extern crate ascii;

use ascii::{AsciiStr, AsciiString, AsciiCast, OwnedAsciiCast};

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
fn opt() {
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
