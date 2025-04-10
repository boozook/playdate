//! Global Panic Handler implementation. Depends on `panic-handler` feature.
#![cfg(feature = "panic-handler")]


mod norm {
	#![cfg(not(miri))]
	use core::fmt::Arguments;
	use core::fmt::Write;
	use core::panic::Location;
	use core::panic::PanicInfo;
	use core::ffi::CStr;
	use crate::ffi::Playdate;


	/// Heapless.
	/// Stops the program execution with custom system-level error.
	///
	/// In case of missed [`crate::API`] (doesn't set) uses [`abort`](core::intrinsics::abort).
	#[cfg_attr(not(test), panic_handler)]
	// #[cfg_attr(all(feature = "panic-handler", not(test)), panic_handler)]
	fn panic(info: &PanicInfo) -> ! {
		#[cfg(all(feature = "entry-point", not(playdate)))]
		{
			crate::PANICKED.store(true, core::sync::atomic::Ordering::Relaxed);
		}

		if let Some(api) = crate::api() {
			match info.message().as_str() {
				Some(m) => error_str(api, m, info.location()),
				None => error_fmt(api, format_args!("{}", info.message()), info.location()),
			}
		} else {
			core::intrinsics::abort()
		}
	}


	const PWORD: &CStr = c"panic";

	#[track_caller]
	fn error_str(api: &Playdate, m: &str, l: Option<&Location<'_>>) -> ! {
		let error = api.system.error;

		if let Some(l) = l {
			unsafe {
				error(
				      c"%s @ %.*s:%d:%d: %.*s".as_ptr(),
				      PWORD.as_ptr(),
				      l.file().len(),
				      l.file().as_ptr(),
				      l.line(),
				      l.column(),
				      m.len(),
				      m.as_ptr(),
				)
			}
		} else {
			unsafe { error(c"%s: %.*s".as_ptr(), PWORD.as_ptr(), m.len(), m.as_ptr()) }
		}
	}

	#[track_caller]
	fn error_fmt(api: &Playdate, m: Arguments<'_>, l: Option<&Location<'_>>) -> ! {
		let error = api.system.error;
		let mut buf = crate::print::allocless::FmtBufDef::new();

		if buf.write_fmt(m).is_ok() {
			let m = buf.as_str();
			error_str(api, m, l);
		} else {
			if let Some(l) = l {
				unsafe {
					error(
					      c"%s @ %.*s:%d:%d".as_ptr(),
					      PWORD.as_ptr(),
					      l.file().len(),
					      l.file().as_ptr(),
					      l.line(),
					      l.column(),
					)
				}
			} else {
				unsafe { error(PWORD.as_ptr()) }
			}
		}
	}
}


// Miri does not support calling a c-variadic function.
// issue: https://github.com/rust-lang/miri/issues/1892
// So, that's why here is special implementation.
mod miri {
	#![cfg(miri)]
	use core::fmt::Write;
	use core::panic::Location;
	use core::panic::PanicInfo;


	// #[cfg_attr(all(feature = "panic-handler", not(test)), panic_handler)]
	#[cfg_attr(not(test), panic_handler)]
	fn panic(info: &PanicInfo) -> ! {
		let print_message = || {
			#[cfg(feature = "alloc")]
			unsafe {
				use alloc::string::ToString;
				let s = info.message().to_string();
				let s = s.trim();
				if !s.is_empty() {
					miri_write_to_stderr(": ".as_bytes());
					miri_write_to_stderr(s.as_bytes());
				}
			}
			#[cfg(not(feature = "alloc"))]
			unsafe {
				let mut buf = crate::print::allocless::FmtBufDef::new();
				if buf.write_fmt(format_args!("{}", info.message())).is_ok() {
					let s = buf.as_str().trim();
					if !s.is_empty() {
						miri_write_to_stderr(": ".as_bytes());
						miri_write_to_stderr(s.as_bytes());
					}
				}
			}
		};

		let print_location = |l: &Location<'_>, pre: &[u8], post: &[u8]| {
			for v in [l.line(), l.column()] {
				use crate::print::allocless::*;
				let mut buf = [0u8; 10];
				let mut index = buf.len() - 1;
				n2s(v as _, &mut index, &mut buf);

				unsafe {
					if !pre.is_empty() {
						miri_write_to_stderr(pre)
					}
					miri_write_to_stderr(&buf[index..]);
					if !post.is_empty() {
						miri_write_to_stderr(post)
					}
				}
			}
		};


		unsafe {
			if let Some(_) = crate::api() {
				match (info.message().as_str(), info.location()) {
					(None, None) => {
						miri_write_to_stderr("panic".as_bytes());
						print_message();
					},
					(Some(m), None) => {
						miri_write_to_stderr("panic: ".as_bytes());
						miri_write_to_stderr(m.as_bytes());
					},
					(None, Some(l)) => {
						miri_write_to_stderr("panic @ ".as_bytes());
						miri_write_to_stderr(l.file().as_bytes());
						print_location(l, &[b':'], &[]);
						print_message();
					},
					(Some(m), Some(l)) => {
						miri_write_to_stderr("panic @ ".as_bytes());
						miri_write_to_stderr(l.file().as_bytes());
						miri_write_to_stderr(&[b':']);
						print_location(l, &[], &[b':']);
						miri_write_to_stderr(&[b' ']);
						miri_write_to_stderr(m.as_bytes());
					},
				}
				miri_write_to_stderr("\n".as_bytes());
			} else {
				miri_write_to_stderr("no api, can't panic, but aborting\n".as_bytes());
			}
		}
		core::intrinsics::abort()
	}


	extern "Rust" {
		/// Miri-provided extern function to print (from the interpreter, not the
		/// program) the contents of a section of program memory, as bytes. Bytes
		/// written using this function will emerge from the interpreter's stderr.
		fn miri_write_to_stderr(bytes: &[u8]);
	}
}
