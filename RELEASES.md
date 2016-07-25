Unreleased
==========
* Fix the implementation of `AsciiExt::to_ascii_lowercase()` for `AsciiChar` converting to uppercase. (introduced in 0.7.0)

Version 0.7.0 (2016-06-25)
==========
* Rename `Ascii` to `AsciiChar` and convert it into an enum. (with a variant for every ASCII character)
* Replace `OwnedAsciiCast` with `IntoAsciiString`.
* Replace `AsciiCast` with `As[Mut]AsciiStr` and `IntoAsciiChar`.
* Add *from[_ascii]_unchecked* methods.
* Replace *from_bytes* with *from_ascii* in method names.
* Return `std::error::Error`-implementing types on error instead of `Option::None` or `Err(())`.
* Implement `AsciiExt` without the `unstable` Cargo feature flag, which is removed.
* Require Rust 1.9 or later.
* Add `#[no_std]` support in a Cargo feature.
* Implement `From<{&,&mut,Box<}AsciiStr>` for `[Ascii]`, `[u8]` and `str`
* Implement `From<{&,&mut,Box<}[Ascii]>`, `As{Ref,Mut}<[Ascii]>` and Default for `AsciiStr`
* Implement `From<Vec<Ascii>>` for `AsciiString`.
* Implement `AsMut<AsciiStr>` for `AsciiString`.
* Stop some `Ascii::is_xxx()` methods from panicking.
* Add `Ascii::is_whitespace()`.
* Add `AsciiString::as_mut_slice()`.
* Add raw pointer methods on `AsciiString`:
  * `from_raw_parts`
  * `as_ptr`
  * `as_mut_ptr`

Version 0.6.0 (2015-12-30)
==========================
* Add `Ascii::from_byte()`
* Add `AsciiStr::trim[_{left,right}]()`

Version 0.5.4 (2015-07-29)
==========================
Implement `IndexMut` for AsciiStr and AsciiString.

Version 0.5.1 (2015-06-13)
==========================
* Add `Ascii::from()`.
* Implement `Index` for `AsciiStr` and `AsciiString`.
* Implement `Default`,`FromIterator`,`Extend` and `Add` for `AsciiString`
* Added inherent methods on `AsciiString`:
  * `with_capacity`
  * `push_str`
  * `capacity`
  * `reserve`
  * `reserve_exact`
  * `shrink_to_fit`
  * `push`
  * `truncate`
  * `pop`
  * `remove`
  * `insert`
  * `len`
  * `is_empty`
  * `clear`

Version 0.5.0 (2015-05-05)
==========================
First release compatible with Rust 1.0.0.
