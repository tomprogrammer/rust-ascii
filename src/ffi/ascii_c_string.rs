use core::{fmt, mem, ops, ptr, slice};
use core::borrow::Borrow;
use std::error::Error;

use {libc, memchr, AsciiString};
use super::{AsciiCStr, FromBytesWithNulError};

/// A possible error value when converting an `AsciiString` from a byte vector or string.
/// It wraps an `AsAsciiStrError` which you can get through the `ascii_error()` method.
///
/// This is the error type for `AsciiString::from_ascii()` and
/// `IntoAsciiString::into_ascii_string()`. They will never clone or touch the content of the
/// original type; It can be extracted by the `into_source` method.
///
/// #Examples
/// ```
/// # use ascii::IntoAsciiString;
/// let err = "bø!".to_string().into_ascii_string().unwrap_err();
/// assert_eq!(err.ascii_error().valid_up_to(), 1);
/// assert_eq!(err.into_source(), "bø!".to_string());
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AsciiNulError<O> {
    error: FromBytesWithNulError,
    owner: O,
}
impl<O> AsciiNulError<O> {
    /// Get the position of the first non-ASCII byte or character.
    #[inline]
    pub fn ascii_error(&self) -> FromBytesWithNulError {
        self.error
    }
    /// Get back the original, unmodified type.
    #[inline]
    pub fn into_source(self) -> O {
        self.owner
    }
}

impl<O> fmt::Debug for AsciiNulError<O> {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.error, fmtr)
    }
}
impl<O> fmt::Display for AsciiNulError<O> {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.error, fmtr)
    }
}
impl<O> Error for AsciiNulError<O> {
    #[inline]
    fn description(&self) -> &str {
        self.error.description()
    }
    #[inline]
    fn cause(&self) -> Option<&Error> {
        Some(&self.error as &Error)
    }
}

/// A type representing an owned C-compatible ascii string.
///
/// This type serves the primary purpose of being able to safely generate a C-compatible ascii
/// string from a Rust byte slice or vector. An instance of this type is a static guarantee that the
/// underlying bytes contain no interior 0 bytes and only ascii charaters and the final byte is 0.
///
/// An `AsciiCString` is created from either a byte slice or a byte vector. A `u8` slice can be
/// obtained with the `as_bytes` method. Slices produced from an `AsciiCString` do *not* contain the
/// trailing nul terminator unless otherwise specified.
///
/// # Examples
///
/// ```no_run
/// # extern crate ascii;
/// # extern crate libc;
/// # fn main() {
/// use ascii::ffi::AsciiCString;
/// use libc::c_char;
///
/// extern {
///     fn my_printer(s: *const c_char);
/// }
///
/// let c_to_print = AsciiCString::new("Hello, world!").unwrap();
/// unsafe {
///     my_printer(c_to_print.as_ptr());
/// }
/// # }
/// ```
///
/// # Safety
///
/// `AsciiCString` is intended for working with traditional C-style strings (a sequence of non-null
/// bytes terminated by a single null byte); the primary use case for these kinds of strings is
/// interoperating with C-like code. Often you will need to transfer ownership to/from that external
/// code. It is strongly recommended that you thoroughly read through the documentation of
/// `AsciiCString` before use, as improper ownership management of `AsciiCString` instances can lead
/// to invalid memory accesses, memory leaks, and other memory errors.
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct AsciiCString {
    inner: Box<[u8]>,
}

impl AsciiCString {
    /// Creates a new C-compatible ascii string from a container of bytes.
    ///
    /// This method will consume the provided data and use the underlying bytes to construct a new
    /// ascii string, ensuring that there is a trailing 0 byte.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ascii;
    /// # extern crate libc;
    /// # fn main() {
    /// use ascii::ffi::AsciiCString;
    /// use libc::c_char;
    ///
    /// extern { fn puts(s: *const c_char); }
    ///
    /// let to_print = AsciiCString::new("Hello!").unwrap();
    /// unsafe {
    ///     puts(to_print.as_ptr());
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the bytes yielded contain an internal 0 byte or the
    /// bytes doesn't encode an ascii string. The error returned will contain the bytes as well as
    /// the position of the nul byte or non-ascii byte.
    pub fn new<T: Into<Vec<u8>> + AsRef<[u8]>>(bytes: T) -> Result<Self, AsciiNulError<T>> {
        match memchr::memchr(0, bytes.as_ref()) {
            Some(i) => Err(AsciiNulError {
                error: FromBytesWithNulError::interior_nul(i),
                owner: bytes,
            }),
            None => match bytes.as_ref().iter().position(|&b| b > 127) {
                Some(index) => Err(AsciiNulError {
                    error: FromBytesWithNulError::not_ascii(index),
                    owner: bytes,
                }),
                None => unsafe { Ok(Self::from_vec_unchecked(bytes.into())) },
            },
        }
    }

    /// Creates a C-compatible ascii string from a byte vector without checking for
    /// interior 0 bytes and ascii encoding.
    ///
    /// This method is equivalent to [`new`] except that no runtime assertion is made that `v`
    /// contains no 0 bytes and contains only ascii characters, and it requires an actual byte
    /// vector, not anything that can be converted to one with Into.
    ///
    /// [`new`]: #method.new
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let raw = b"foo".to_vec();
    /// unsafe {
    ///     let c_string = AsciiCString::from_vec_unchecked(raw);
    /// }
    /// ```
    pub unsafe fn from_vec_unchecked(mut v: Vec<u8>) -> Self {
        v.reserve_exact(1);
        v.push(0);
        AsciiCString {
            inner: v.into_boxed_slice(),
        }
    }

    /// Retakes ownership of an `AsciiCString` that was transferred to C.
    ///
    /// Additionally, the length of the ascii string will be recalculated from the pointer.
    ///
    /// # Safety
    ///
    /// This should only ever be called with a pointer that was earlier
    /// obtained by calling [`into_raw`] on a `AsciiCString`. Other usage (e.g. trying to take
    /// ownership of a string that was allocated by foreign code) is likely to lead
    /// to undefined behavior or allocator corruption.
    ///
    /// [`into_raw`]: #method.into_raw
    ///
    /// # Examples
    ///
    /// Create an `AsciiCString`, pass ownership to an `extern` function (via raw pointer), then
    /// retake ownership with `from_raw`:
    ///
    /// ```no_run
    /// # extern crate ascii;
    /// # extern crate libc;
    /// # fn main() {
    /// use ascii::ffi::AsciiCString;
    /// use libc::c_char;
    ///
    /// extern {
    ///     fn some_extern_function(s: *mut c_char);
    /// }
    ///
    /// let c_string = AsciiCString::new("Hello!").unwrap();
    /// let raw = c_string.into_raw();
    /// unsafe {
    ///     some_extern_function(raw);
    ///     let c_string = AsciiCString::from_raw(raw);
    /// }
    /// # }
    /// ```
    pub unsafe fn from_raw(ptr: *mut libc::c_char) -> Self {
        let len = libc::strlen(ptr) + 1; // Including the NUL byte
        let slice =
            slice::from_raw_parts_mut(ptr, len as usize) as *mut [libc::c_char] as *mut [u8];
        AsciiCString {
            inner: Box::from_raw(slice),
        }
    }

    /// Transfers ownership of the ascii string to a C caller.
    ///
    /// The pointer must be returned to Rust and reconstituted using
    /// [`from_raw`] to be properly deallocated. Specifically, one
    /// should *not* use the standard C `free` function to deallocate
    /// this string.
    ///
    /// Failure to call [`from_raw`] will lead to a memory leak.
    ///
    /// [`from_raw`]: #method.from_raw
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new("foo").unwrap();
    ///
    /// let ptr = c_string.into_raw();
    ///
    /// unsafe {
    ///     assert_eq!(b'f', *ptr as u8);
    ///     assert_eq!(b'o', *ptr.offset(1) as u8);
    ///     assert_eq!(b'o', *ptr.offset(2) as u8);
    ///     assert_eq!(b'\0', *ptr.offset(3) as u8);
    ///
    ///     // retake pointer to free memory
    ///     let _ = AsciiCString::from_raw(ptr);
    /// }
    /// ```
    #[inline]
    pub fn into_raw(self) -> *mut libc::c_char {
        Box::into_raw(self.into_inner()) as *mut libc::c_char
    }

    /// Converts the `AsciiCString` into a `String`.
    ///
    /// On failure, ownership of the original `AsciiCString` is returned.
    pub fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_bytes()) }
    }

    /// Converts the `AsciiCString` into an `AsciiString`.
    ///
    /// On failure, ownership of the original `AsciiCString` is returned.
    pub fn into_ascii_string(self) -> AsciiString {
        unsafe { AsciiString::from_ascii_unchecked(self.into_bytes()) }
    }

    /// Returns the underlying byte buffer.
    ///
    /// The returned buffer does **not** contain the trailing nul separator and
    /// it is guaranteed to not have any interior nul bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new("foo").unwrap();
    /// let bytes = c_string.into_bytes();
    /// assert_eq!(bytes, vec![b'f', b'o', b'o']);
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        let mut vec = self.into_inner().into_vec();
        let _nul = vec.pop();
        debug_assert_eq!(_nul, Some(0u8));
        vec
    }

    /// Equivalent to the [`into_bytes`] function except that the returned vector
    /// includes the trailing nul byte.
    ///
    /// [`into_bytes`]: #method.into_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new("foo").unwrap();
    /// let bytes = c_string.into_bytes_with_nul();
    /// assert_eq!(bytes, vec![b'f', b'o', b'o', b'\0']);
    /// ```
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.into_inner().into_vec()
    }

    /// Returns the contents of this `AsciiCString` as a slice of bytes.
    ///
    /// The returned slice does **not** contain the trailing nul separator and
    /// it is guaranteed to not have any interior nul bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new("foo").unwrap();
    /// let bytes = c_string.as_bytes();
    /// assert_eq!(bytes, &[b'f', b'o', b'o']);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[..self.inner.len() - 1]
    }

    /// Equivalent to the [`as_bytes`] function except that the returned slice
    /// includes the trailing nul byte.
    ///
    /// [`as_bytes`]: #method.as_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::AsciiCString;
    ///
    /// let c_string = AsciiCString::new("foo").unwrap();
    /// let bytes = c_string.as_bytes_with_nul();
    /// assert_eq!(bytes, &[b'f', b'o', b'o', b'\0']);
    /// ```
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.inner
    }

    /// Extracts a [`AsciiCStr`] slice containing the entire string.
    ///
    /// [`AsciiCStr`]: struct.AsciiCStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::{AsciiCString, AsciiCStr};
    ///
    /// let c_string = AsciiCString::new(b"foo".to_vec()).unwrap();
    /// let c_str = c_string.as_ascii_c_str();
    /// assert_eq!(c_str, AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap());
    /// ```
    #[inline]
    pub fn as_ascii_c_str(&self) -> &AsciiCStr {
        &*self
    }

    /// Converts this `AsciiCString` into a boxed [`AsciiCStr`].
    ///
    /// [`AsciiCStr`]: struct.AsciiCStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::ffi::{AsciiCString, AsciiCStr};
    ///
    /// let c_string = AsciiCString::new(b"foo".to_vec()).unwrap();
    /// let boxed = c_string.into_boxed_c_str();
    /// assert_eq!(&*boxed, AsciiCStr::from_bytes_with_nul(b"foo\0").unwrap());
    /// ```
    pub fn into_boxed_c_str(self) -> Box<AsciiCStr> {
        unsafe {
            Box::from_raw(Box::into_raw(self.into_inner()) as *mut [libc::c_char]
                as *mut AsciiCStr)
        }
    }

    /// Bypass "move out of struct which implements `Drop` trait" restriction.
    fn into_inner(self) -> Box<[u8]> {
        unsafe {
            let result = ptr::read(&self.inner);
            mem::forget(self);
            result
        }
    }
}
// Turns this `CString` into an empty string to prevent
// memory unsafe code from working by accident. Inline
// to prevent LLVM from optimizing it away in debug builds.
impl Drop for AsciiCString {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            *self.inner.get_unchecked_mut(0) = 0;
        }
    }
}

impl ops::Deref for AsciiCString {
    type Target = AsciiCStr;

    #[inline]
    fn deref(&self) -> &AsciiCStr {
        unsafe { AsciiCStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
}

impl fmt::Debug for AsciiCString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl From<AsciiCString> for Vec<u8> {
    #[inline]
    fn from(s: AsciiCString) -> Vec<u8> {
        s.into_bytes()
    }
}

impl Default for AsciiCString {
    /// Creates an empty `CString`.
    fn default() -> AsciiCString {
        let a: &AsciiCStr = Default::default();
        a.to_owned()
    }
}

impl Borrow<AsciiCStr> for AsciiCString {
    #[inline]
    fn borrow(&self) -> &AsciiCStr {
        self
    }
}

impl From<Box<AsciiCStr>> for AsciiCString {
    #[inline]
    fn from(s: Box<AsciiCStr>) -> AsciiCString {
        let ptr = Box::into_raw(s) as *mut [libc::c_char] as *mut [u8];
        unsafe {
            AsciiCString {
                inner: Box::from_raw(ptr),
            }
        }
    }
}
