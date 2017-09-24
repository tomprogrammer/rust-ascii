use core::{fmt, mem, ops, slice, str};
use core::cmp::Ordering;
use core::fmt::Write;
use std::ascii;
use std::error::Error;

use {libc, memchr, AsciiStr};
use super::AsciiCString;

/// An error returned from [`AsciiCStr::from_bytes_with_nul`] to indicate that a nul byte was found
/// too early in the slice provided or one wasn't found at all.
///
/// [`AsciiCStr::from_bytes_with_nul`]: struct.AsciiCStr.html#method.from_bytes_with_nul
///
/// # Examples
///
/// ```
/// use ascii::ffi::{AsciiCStr, FromBytesWithNulError};
///
/// let _: FromBytesWithNulError = AsciiCStr::from_bytes_with_nul(b"f\0oo").unwrap_err();
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FromBytesWithNulError {
    kind: FromBytesWithNulErrorKind,
    pos: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FromBytesWithNulErrorKind {
    InteriorNul,
    NotNulTerminated,
    NotAscii,
}

impl FromBytesWithNulError {
    /// Returns the index of the first interior nul byte or non-ASCII byte. If the terminating nul
    /// byte is missing, the length of the string is returned.
    #[inline]
    pub fn valid_up_to(&self) -> usize {
        self.pos
    }

    pub fn kind(&self) -> FromBytesWithNulErrorKind {
        self.kind
    }

    pub(super) fn interior_nul(pos: usize) -> Self {
        FromBytesWithNulError {
            pos: pos,
            kind: FromBytesWithNulErrorKind::InteriorNul,
        }
    }
    pub(super) fn not_nul_terminated(len: usize) -> Self {
        FromBytesWithNulError {
            pos: len,
            kind: FromBytesWithNulErrorKind::NotNulTerminated,
        }
    }
    pub(super) fn not_ascii(index: usize) -> Self {
        FromBytesWithNulError {
            pos: index,
            kind: FromBytesWithNulErrorKind::NotAscii,
        }
    }
}

impl Error for FromBytesWithNulError {
    fn description(&self) -> &str {
        match self.kind {
            FromBytesWithNulErrorKind::InteriorNul => "data provided contains an interior nul byte",
            FromBytesWithNulErrorKind::NotNulTerminated => "data provided is not nul terminated",
            FromBytesWithNulErrorKind::NotAscii => "data provided contains a non-ascii character",
        }
    }
}

impl fmt::Display for FromBytesWithNulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())?;
        write!(f, " at byte pos {}", self.pos)?;
        Ok(())
    }
}

/// Representation of a borrowed ascii C string.
///
/// This dynamically sized type is only safely constructed via a borrowed version of an instance of
/// `AsciiCString`. This type can be constructed from a raw ascii C string as well and represents an
/// ascii C string borrowed from another location.
///
/// Note that this structure is **not** `repr(C)` and is not recommended to be placed in the
/// signatures of FFI functions. Instead safe wrappers of FFI functions may leverage the unsafe
/// [`from_ptr`] constructor to provide a safe interface to other consumers.
///
/// [`from_ptr`]: #method.from_ptr
///
/// # Examples
///
/// Inspecting a foreign C string:
///
/// ```no_run
/// # extern crate ascii;
/// # extern crate libc;
/// # fn main() {
/// use ascii::ffi::AsciiCStr;
/// use libc::c_char;
///
/// extern { fn my_string() -> *const c_char; }
///
/// unsafe {
///     let slice = AsciiCStr::from_ptr(my_string());
///     println!("string length: {}", slice.to_bytes().len());
/// }
/// # }
/// ```
///
/// Passing a Rust-originating C string:
///
/// ```no_run
/// # extern crate ascii;
/// # extern crate libc;
/// use ascii::ffi::{AsciiCString, AsciiCStr};
/// use libc::c_char;
///
/// fn work(data: &AsciiCStr) {
///     extern { fn work_with(data: *const c_char); }
///
///     unsafe { work_with(data.as_ptr()) }
/// }
///
/// # fn main() {
/// let s = AsciiCString::new("data data data data").unwrap();
/// work(&s);
/// # }
/// ```
///
/// Converting a foreign C string into a Rust `String`:
///
/// ```no_run
/// # extern crate ascii;
/// # extern crate libc;
/// use ascii::ffi::AsciiCStr;
/// use libc::c_char;
///
/// extern { fn my_string() -> *const c_char; }
///
/// fn my_string_safe() -> String {
///     unsafe {
///         AsciiCStr::from_ptr(my_string()).to_str().to_owned()
///     }
/// }
///
/// # fn main() {
/// println!("string: {}", my_string_safe());
/// # }
/// ```
#[derive(Hash)]
pub struct AsciiCStr {
    // FIXME: this should not be represented with a DST slice but rather with
    //        just a raw `c_char` along with some form of marker to make
    //        this an unsized type. Essentially `sizeof(&CStr)` should be the
    //        same as `sizeof(&c_char)` but `CStr` should be an unsized type.
    inner: [libc::c_char],
}

impl AsciiCStr {
    /// Casts a raw C string to a safe ascii C string wrapper.
    ///
    /// This function will cast the provided `ptr` to the `AsciiCStr` wrapper which
    /// allows inspection and interoperation of non-owned C strings. This method
    /// is unsafe for a number of reasons:
    ///
    /// * There is no guarantee to the validity of `ptr`.
    /// * The returned lifetime is not guaranteed to be the actual lifetime of `ptr`.
    /// * There is no guarantee that the memory pointed to by `ptr` contains a valid nul terminator
    ///   byte at the end of the string.
    /// * There is no guarantee that the memory pointed to by `ptr` contains only ascii characters.
    ///
    /// > **Note**: This operation is intended to be a 0-cost cast but it is
    /// > currently implemented with an up-front calculation of the length of
    /// > the string. This is not guaranteed to always be the case.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ascii;
    /// # extern crate libc;
    /// # fn main() {
    /// use ascii::ffi::AsciiCStr;
    /// use libc::c_char;
    ///
    /// extern {
    ///     fn my_string() -> *const c_char;
    /// }
    ///
    /// unsafe {
    ///     let slice = AsciiCStr::from_ptr(my_string());
    ///     println!("string returned: {}", slice.to_str());
    /// }
    /// # }
    /// ```
    pub unsafe fn from_ptr<'a>(ptr: *const libc::c_char) -> &'a Self {
        let len = libc::strlen(ptr);
        let ptr = ptr as *const u8;
        AsciiCStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(ptr, len as usize + 1))
    }

    /// Creates a ascii C string wrapper from a byte slice.
    ///
    /// This function will cast the provided `bytes` to an `AsciiCStr` wrapper after
    /// ensuring that it is null terminated, does not contain any interior
    /// nul bytes and only contains ascii characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let cstr = AsciiCStr::from_bytes_with_nul(b"hello\0");
    /// assert!(cstr.is_ok());
    /// ```
    ///
    /// Creating an `AsciiCStr` without a trailing nul byte is an error:
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"hello");
    /// assert!(c_str.is_err());
    /// ```
    ///
    /// Creating an `AsciiCStr` with an interior nul byte is an error:
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"he\0llo\0");
    /// assert!(c_str.is_err());
    /// ```
    pub fn from_bytes_with_nul(bytes: &[u8]) -> Result<&Self, FromBytesWithNulError> {
        let nul_pos = memchr::memchr(0, bytes);
        if let Some(nul_pos) = nul_pos {
            if nul_pos + 1 != bytes.len() {
                return Err(FromBytesWithNulError::interior_nul(nul_pos));
            }
            match bytes.iter().position(|&b| b > 127) {
                Some(index) => Err(FromBytesWithNulError::not_ascii(index)),
                None => unsafe { Ok(Self::from_bytes_with_nul_unchecked(bytes)) },
            }
        } else {
            Err(FromBytesWithNulError::not_nul_terminated(bytes.len()))
        }
    }

    /// Unsafely creates an ascii C string wrapper from a byte slice.
    ///
    /// This function will cast the provided `bytes` to an `AsciiCStr` wrapper without performing
    /// any sanity checks. The provided slice must be null terminated, not contain any interior nul
    /// bytes and only contain ascii characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::{AsciiCStr, AsciiCString};
    ///
    /// unsafe {
    ///     let cstring = AsciiCString::new("hello").unwrap();
    ///     let cstr = AsciiCStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
    ///     assert_eq!(cstr, &*cstring);
    /// }
    /// ```
    #[inline]
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes as *const [u8] as *const [libc::c_char] as *const AsciiCStr;
        &*ptr
    }

    /// Returns the inner pointer to this ascii C string.
    ///
    /// The returned pointer will be valid for as long as `self` is and points
    /// to a contiguous region of memory terminated with a 0 byte to represent
    /// the end of the ascii string.
    ///
    /// **WARNING**
    ///
    /// It is your responsibility to make sure that the underlying memory is not
    /// freed too early. For example, the following code will cause undefined
    /// behaviour when `ptr` is used inside the `unsafe` block:
    ///
    /// ```no_run
    /// use ascii::ffi::{AsciiCString};
    ///
    /// let ptr = AsciiCString::new("Hello").unwrap().as_ptr();
    /// unsafe {
    ///     // `ptr` is dangling
    ///     *ptr;
    /// }
    /// ```
    ///
    /// This happens because the pointer returned by `as_ptr` does not carry any
    /// lifetime information and the string is deallocated immediately after
    /// the `CString::new("Hello").unwrap().as_ptr()` expression is evaluated.
    /// To fix the problem, bind the string to a local variable:
    ///
    /// ```no_run
    /// use ascii::ffi::{AsciiCString};
    ///
    /// let hello = AsciiCString::new("Hello").unwrap();
    /// let ptr = hello.as_ptr();
    /// unsafe {
    ///     // `ptr` is valid because `hello` is in scope
    ///     *ptr;
    /// }
    /// ```
    #[inline]
    pub fn as_ptr(&self) -> *const libc::c_char {
        self.inner.as_ptr()
    }

    /// Converts this ascii C string to a byte slice.
    ///
    /// This function will calculate the length of this ascii string (which normally requires a
    /// linear amount of work to be done) and then return the resulting slice of `u8` elements.
    ///
    /// The returned slice will **not** contain the trailing nul that this ascii C string has.
    ///
    /// > **Note**: This method is currently implemented as a 0-cost cast, but
    /// > it is planned to alter its definition in the future to perform the
    /// > length calculation whenever this method is called.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap();
    /// assert_eq!(c_str.to_bytes(), b"foo");
    /// ```
    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }

    /// Converts this ascii C string to a byte slice containing the trailing 0 byte.
    ///
    /// This function is the equivalent of [`to_bytes`] except that it will retain
    /// the trailing nul instead of chopping it off.
    ///
    /// > **Note**: This method is currently implemented as a 0-cost cast, but
    /// > it is planned to alter its definition in the future to perform the
    /// > length calculation whenever this method is called.
    ///
    /// [`to_bytes`]: #method.to_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap();
    /// assert_eq!(c_str.to_bytes_with_nul(), b"foo\0");
    /// ```
    #[inline]
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        let ptr = &self.inner as *const [libc::c_char] as *const [u8];
        unsafe { &*ptr }
    }

    /// Yields a `&str` slice.
    ///
    /// This function will calculate the length of this ascii string and then return the `&str` if
    /// it's valid.
    ///
    /// > **Note**: This method is currently implemented to check for validity
    /// > after a 0-cost cast, but it is planned to alter its definition in the
    /// > future to perform the length calculation in addition to the UTF-8
    /// > check whenever this method is called.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap();
    /// assert_eq!(c_str.to_str(), "foo");
    /// ```
    pub fn to_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.to_bytes()) }
    }

    /// Yields an `&AsciiStr` slice.
    ///
    /// This function will calculate the length of this ascii string and then return the `&AsciiStr`
    /// if it's valid.
    ///
    /// > **Note**: This method is currently implemented to check for validity
    /// > after a 0-cost cast, but it is planned to alter its definition in the
    /// > future to perform the length calculation in addition to the UTF-8
    /// > check whenever this method is called.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::AsciiStr;
    /// use ascii::ffi::AsciiCStr;
    ///
    /// let c_str = AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap();
    /// assert_eq!(c_str.to_ascii_str(), AsciiStr::from_ascii(b"foo").unwrap());
    /// ```
    pub fn to_ascii_str(&self) -> &AsciiStr {
        unsafe { AsciiStr::from_ascii_unchecked(self.to_bytes()) }
    }

    /// Converts a `Box<AsciiCStr>` into an [`AsciiCString`] without copying or allocating.
    ///
    /// [`AsciiCString`]: struct.AsciiCString.html
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new(b"foo".to_vec()).unwrap();
    /// let boxed = c_string.into_boxed_c_str();
    /// assert_eq!(boxed.into_ascii_c_string(), AsciiCString::new("foo").unwrap());
    /// ```
    pub fn into_ascii_c_string(self: Box<AsciiCStr>) -> AsciiCString {
        AsciiCString::from(self)
    }
}

impl PartialEq for AsciiCStr {
    fn eq(&self, other: &AsciiCStr) -> bool {
        self.to_bytes().eq(other.to_bytes())
    }
}

impl Eq for AsciiCStr {}

impl PartialOrd for AsciiCStr {
    fn partial_cmp(&self, other: &AsciiCStr) -> Option<Ordering> {
        self.to_bytes().partial_cmp(&other.to_bytes())
    }
}

impl Ord for AsciiCStr {
    fn cmp(&self, other: &AsciiCStr) -> Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

impl fmt::Debug for AsciiCStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;
        for byte in self.to_bytes()
            .iter()
            .flat_map(|&b| ascii::escape_default(b))
        {
            f.write_char(byte as char)?;
        }
        write!(f, "\"")
    }
}

impl<'a> Default for &'a AsciiCStr {
    fn default() -> &'a AsciiCStr {
        static SLICE: &'static [libc::c_char] = &[0];
        unsafe { AsciiCStr::from_ptr(SLICE.as_ptr()) }
    }
}

impl Default for Box<AsciiCStr> {
    fn default() -> Box<AsciiCStr> {
        let boxed: Box<[u8]> = Box::from([0]);
        unsafe { mem::transmute(boxed) }
    }
}

impl ToOwned for AsciiCStr {
    type Owned = AsciiCString;

    fn to_owned(&self) -> AsciiCString {
        unsafe { AsciiCString::from_vec_unchecked(self.to_bytes_with_nul().to_vec()) }
    }
}

impl<'a> From<&'a AsciiCStr> for AsciiCString {
    fn from(s: &'a AsciiCStr) -> AsciiCString {
        s.to_owned()
    }
}

impl ops::Index<ops::RangeFull> for AsciiCString {
    type Output = AsciiCStr;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &AsciiCStr {
        self
    }
}

impl AsRef<AsciiCStr> for AsciiCStr {
    #[inline]
    fn as_ref(&self) -> &AsciiCStr {
        self
    }
}

impl AsRef<AsciiCStr> for AsciiCString {
    #[inline]
    fn as_ref(&self) -> &AsciiCStr {
        self
    }
}

impl<'a> From<&'a AsciiCStr> for Box<AsciiCStr> {
    fn from(s: &'a AsciiCStr) -> Box<AsciiCStr> {
        let boxed: Box<[u8]> = Box::from(s.to_bytes_with_nul());
        unsafe { mem::transmute(boxed) }
    }
}

impl From<AsciiCString> for Box<AsciiCStr> {
    #[inline]
    fn from(s: AsciiCString) -> Box<AsciiCStr> {
        s.into_boxed_c_str()
    }
}
