use core::ffi::CStr;


pub fn print(api: &'static crate::ffi::Playdate, fmt: core::fmt::Arguments<'_>) {
	if let Some(s) = fmt.as_str() {
		print_str(api, s);
	} else {
		#[cfg(feature = "alloc")]
		{
			use alloc::string::ToString;
			let s = fmt.to_string();
			print_str(api, s.as_str());
		}
		#[cfg(not(feature = "alloc"))]
		{
			use core::fmt::Write;
			let mut buf = fmt::FmtBufDef::new();
			if buf.write_fmt(fmt).is_ok() {
				print_str(api, buf.as_str());
			} else {
				print_str(api, "printing err: fmt on stack failed");
			}
		}
	}
}


pub fn print_cstr(api: &'static crate::ffi::Playdate, s: &CStr) {
	unsafe { (api.system.logToConsole)(s.as_ptr().cast()) }
}


#[cfg(not(miri))]
pub fn print_str(api: &'static crate::ffi::Playdate, s: &str) {
	const TMT: &CStr = c"%.*s";
	unsafe { (api.system.logToConsole)(TMT.as_ptr().cast(), s.len(), s.as_ptr()) }
}


// Miri does not support calling a c-variadic function.
// issue: https://github.com/rust-lang/miri/issues/1892
#[cfg(miri)]
pub fn print_str(_: &'static crate::ffi::Playdate, s: &str) {
	unsafe {
		miri_write_to_stdout(s.as_bytes());
		miri_write_to_stdout(&[b'\n']);
	}
}


#[cfg(miri)]
extern "Rust" {
	/// Miri-provided extern function to print (from the interpreter, not the
	/// program) the contents of a section of program memory, as bytes. Bytes
	/// written using this function will emerge from the interpreter's stdout.
	pub fn miri_write_to_stdout(bytes: &[u8]);
}


/// Heapless ASCII formatting.
pub mod fmt {
	use core::mem::MaybeUninit;
	use core::{fmt, str};


	/// A lookup table to prevent the need for conditional branching
	/// The value of the remainder of each step will be used as the index
	const LOOKUP: &[u8] = b"0123456789";
	/// A lookup table optimized for decimal lookups. Each two indices represents one possible number.
	const DEC_LOOKUP: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                    2021222324252627282930313233343536373839\
                                    4041424344454647484950515253545556575859\
                                    6061626364656667686970717273747576777879\
                                    8081828384858687888990919293949596979899";

	/// Render `num` base 10 to string `buf`, left-aligning.
	/// Modifies `index` pointing to the start of actual string in `buf`.
	///
	/// Size of the buffer must be:
	/// - `u8` requires at least 3 bytes
	/// - `u16` requires at least 5 bytes
	/// - `u32` requires at least 10 bytes
	/// - `u64` requires at least 20 bytes
	/// - `i8` requires at least 4 bytes
	/// - `i16` requires at least 6 bytes
	/// - `i32` requires at least 11 bytes
	/// - `i64` requires at least 20 bytes
	///
	/// ## Example
	/// ```rust
	/// use playdate_sys::print::fmt::*;
	/// let mut buf = [0u8; 10];
	/// let mut i = buf.len() - 1;
	/// n2ascii(42, &mut i, &mut buf);
	/// assert_eq!(7, i);
	/// assert_eq!(&[b'4', b'2'], &buf[8..]);
	/// ```
	pub fn n2ascii(num: usize, index: &mut usize, buf: &mut [u8]) {
		let mut v = num;
		// Decode four characters at the same time
		while v > 9999 {
			debug_assert!(*index > 3);
			let rem = (v % 10000) as u16;
			let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
			buf[*index - 3..*index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
			*index = index.wrapping_sub(4);
			v /= 10000;
		}
		if v > 999 {
			debug_assert!(*index > 2);
			let (frst, scnd) = ((v / 100) * 2, (v % 100) * 2);
			buf[*index - 3..*index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
			*index = index.wrapping_sub(4);
		} else if v > 99 {
			debug_assert!(*index > 1);
			let section = (v as u16 / 10) * 2;
			buf[*index - 2..*index].copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
			buf[*index] = LOOKUP[(v % 10) as usize];
			*index = index.wrapping_sub(3);
		} else if v > 9 {
			debug_assert!(*index > 0);
			v *= 2;
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[v as usize..v as usize + 2]);
			*index = index.wrapping_sub(2);
		} else {
			buf[*index] = LOOKUP[v as usize];
			*index = index.wrapping_sub(1);
		}
	}


	/// On-stack format buffer with default length (inner on-stack buffer size) determined by cfg.
	///
	/// Allowed `cfg` values for `format_buffer` are: `0`, `128`, `256`, `512`, `1024`.
	/// Default value is `1024`.
	pub type FmtBufDef = FmtBuf<FMT_BUF_LEN>;

	/// Default buffer length determined by cfg.
	const FMT_BUF_LEN: usize = cfg_match! {{
		format_buffer = "0" => { 0 }
		format_buffer = "128" => { 128 }
		format_buffer = "256" => { 256 }
		format_buffer = "512" => { 512 }
		format_buffer = "1024" => { 1024 }
		_ => { 1024 }
	}};

	/// On-stack format buffer with constant length.
	pub struct FmtBuf<const LEN: usize> {
		buf: [MaybeUninit<u8>; LEN],
		len: usize,
	}

	impl<const LEN: usize> FmtBuf<LEN> {
		#[inline]
		pub const fn new() -> Self {
			Self { buf: [MaybeUninit::uninit(); LEN],
			       len: 0 }
		}

		#[inline]
		pub fn as_str(&self) -> &str {
			// SAFETY: `buf` is only written to by the `fmt::Write::write_str` implementation
			// which writes a valid UTF-8 string to `buf` and correctly sets `len`.
			unsafe {
				let s = self.buf[..self.len].assume_init_ref();
				str::from_utf8_unchecked(s)
			}
		}
	}

	impl<const LEN: usize> fmt::Write for FmtBuf<LEN> {
		fn write_str(&mut self, s: &str) -> fmt::Result {
			let bytes = s.as_bytes();

			if let Some(buf) = self.buf.get_mut(self.len..(self.len + bytes.len())) {
				buf.write_copy_of_slice(bytes);
				self.len += bytes.len();
				Ok(())
			} else {
				Err(fmt::Error)
			}
		}
	}
}
