// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// Core/lib/string.rs
//! Dynamically-allocated string type
//!
//! Acts every similarly to the rust std's String type.
use core::{ops,cmp,fmt};
use vec::Vec;

/// String type
#[derive(Clone,PartialOrd,Ord,PartialEq,Eq)]
pub struct String(Vec<u8>);

/// String backed to a statically-allocated buffer
pub struct FixedString<Buf: AsMut<[u8]>+AsRef<[u8]>>
{
	data: Buf,
	len: usize,
}

impl String
{
	/// Create a new empty string (with no allocation)
	pub fn new() -> String {
		String(Vec::new())
	}
	/// Create a pre-allocated string capable of holding `cap` bytes
	pub fn with_capacity(cap: usize) -> String {
		String(Vec::with_capacity(cap))
	}
	/// Create a string from a string slice
	pub fn from_str(string: &str) -> String {
		let mut v = Vec::new();
		v.push_all(string.as_bytes());
		String(v)
	}
	/// Create a string from a byte slice
	pub fn from_utf8_lossy(slice: &[u8]) -> ::borrow::Cow<str> {
		let mut i = match ::str::from_utf8(&self.0)
			{
			Ok(v) => return Cow::Borrowed(v),
			Err(e) => e.valid_up_to(),
			};

		todo!("String::from_utf8_lossy");
	}
	/// Create a string from a `fmt::Arguments` instance (used by `format!`)
	pub fn from_args(args: fmt::Arguments) -> String {
		use core::fmt::Write;
		let mut ret = String::new();
		let _ = write!(&mut ret, "{}", args);
		ret
	}
	
	pub fn push(&mut self, c: char) {
		let mut buf = [0,0,0,0];
		let len = c.encode_utf8(&mut buf).expect("Four bytes not enough for utf-8?");
		self.0.push_all(&buf[..len]);
	}
	/// Append `s` to the string
	pub fn push_str(&mut self, s: &str) {
		self.0.push_all(s.as_bytes());
	}
	pub fn pop(&mut self) -> Option<char> {
		for i in (0 .. self.0.len()).rev() {
			if self.is_char_boundary(i) {
				let rv = self.char_at(i);
				self.0.truncate(i);
				return Some( rv );
			}
		}
		None
	}

	pub fn clear(&mut self) {
		self.0.clear()
	}
	
	/// Unsafely obtain a borrow of the internal Vec
	pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8> {
		&mut self.0
	}
	
	/// Return the string as a &str
	fn as_slice(&self) -> &str {
		let bytes: &[u8] = self.0.as_ref();
		// SAFE: Valid UTF-8
		unsafe { ::core::str::from_utf8_unchecked(bytes) }
	}
}

impl Default for String {
	fn default() -> String { String::new() }
}

impl fmt::Write for String
{
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result
	{
		self.push_str(s);
		Ok( () )
	}
}

impl ops::Deref for String
{
	type Target = str;
	fn deref(&self) -> &str {
		self.as_slice()
	}
}

impl fmt::Display for String
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		<str as fmt::Display>::fmt(&self, f)
	}
}
impl fmt::Debug for String
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		<str as fmt::Debug>::fmt(&self, f)
	}
}

impl<'a> ::core::iter::FromIterator<&'a str> for String {
	fn from_iter<T: IntoIterator<Item=&'a str>>(it: T) -> String {
		let mut rv = String::new();
		for s in it {
			rv.push_str(s)
		}
		rv
	}
}

impl<'a> From<&'a str> for String
{
	fn from(v: &str) -> String {
		String::from_str(v)
	}
}
//impl<T: ::lib::error::Error> From<T> for String {
//	fn from(v: T) -> String {
//		format!("{}", v)
//	}
//}

impl PartialEq<str> for String {
	fn eq(&self, v: &str) -> bool {
		<str as PartialEq>::eq(&self, v)
	}
}
impl PartialOrd<str> for String {
	fn partial_cmp(&self, v: &str) -> Option<cmp::Ordering> {
		<str as PartialOrd>::partial_cmp(&self, v)
	}
}
impl<'a> PartialEq<&'a str> for String {
	fn eq(&self, v: & &'a str) -> bool {
		<str as PartialEq>::eq(&self, *v)
	}
}
impl<'a> PartialOrd<&'a str> for String {
	fn partial_cmp(&self, v: & &'a str) -> Option<cmp::Ordering> {
		<str as PartialOrd>::partial_cmp(&self, *v)
	}
}


impl<B: AsMut<[u8]>+AsRef<[u8]>> FixedString<B>
{
	/// Create a new fixed-capacity string using the provided buffer
	pub fn new(backing: B) -> FixedString<B> {
		assert!(backing.as_ref().len() > 0);
		FixedString {
			data: backing,
			len: 0,
		}
	}
	pub fn push_char(&mut self, c: char) {
		match c.encode_utf8(&mut self.data.as_mut()[self.len..])
		{
		Some(l) => self.len += l,
		None => todo!("Freeze string once allocation exceeded"),
		}
	}
	/// Append a slice
	pub fn push_str(&mut self, s: &str) {
		self.extend( s.chars() );
	}
	
	pub fn clear(&mut self) {
		self.len = 0;
	}
}
impl<B: AsMut<[u8]>+AsRef<[u8]>> ::core::iter::Extend<char> for FixedString<B>
{
	fn extend<T>(&mut self, iterable: T)
	where
		T: ::core::iter::IntoIterator<Item=char>
	{
		for c in iterable {
			self.push_char(c);
		}
	}
}
impl<B: AsMut<[u8]>+AsRef<[u8]>> ops::Deref for FixedString<B>
{
	type Target = str;
	fn deref(&self) -> &str {
		let bytes = &self.data.as_ref()[..self.len];
		// SAFE: Valid UTF-8
		unsafe { ::core::str::from_utf8_unchecked(bytes) }
	}
}
impl<B: AsMut<[u8]>+AsRef<[u8]>> ::core::fmt::Display for FixedString<B> {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		::core::fmt::Display::fmt(&**self, f)
	}
}
impl<B: AsMut<[u8]>+AsRef<[u8]>> ::core::fmt::Write for FixedString<B> {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		self.push_str(s);
		Ok( () )
	}
}

/// Construct a `String` using a format string and arguments
#[macro_export]
macro_rules! format {
	($($arg:tt)*) => ($crate::lib::string::String::from_args(format_args!($($arg)*)))
}

// vim: ft=rust
