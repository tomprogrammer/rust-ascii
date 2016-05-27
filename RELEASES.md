Unreleased
==========
* Convert `Ascii` into an enum with a variant for every ASCII character.
* Implement `From<{&,&mut,Box<}AsciiStr>` for `[Ascii]`, `[u8]` and `str`
* Implement `From<{&,&mut,Box<}[Ascii]>`, `As{Ref,Mut}<[Ascii]` and Default for `AsciiStr`
* Add `IntoAsciiString` and `IntoAsciiChar` to replace `OwnedAsciiCast`.
* Add `AsAsciiStr` to replace `AsciiCast`.
* Add `Ascii::is_whitespace()`
* Running the unit tests now require Rust 1.7.0.

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
* Implement `Index` for AsciiStr and AsciiString.
* Implement `Default`,`FromIterator`,`Extend` and `Add` for `AsciiString`
* Added inherent methods on AsciiString:
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
