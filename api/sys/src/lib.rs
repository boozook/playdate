// https://doc.rust-lang.org/core/ffi/index.html
// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![cfg_attr(not(test), no_std)]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
// panic
#![cfg_attr(feature = "panic-handler", feature(panic_info_message))]
// allocator
#![cfg_attr(feature = "allocator", feature(alloc_error_handler))]
#![cfg_attr(feature = "allocator", feature(allocator_api))]
#![cfg_attr(feature = "allocator", feature(alloc_layout_extra))]
// error
#![feature(error_in_core)]
#![cfg_attr(feature = "try-trait-v2", feature(try_trait_v2))]
// experimental features
#![cfg_attr(feature = "bindings-derive-constparamty",
            feature(adt_const_params),
            allow(incomplete_features))]


#[allow(unused_imports)]
#[macro_use]
pub extern crate alloc;

pub mod log;
pub mod traits;

mod sys;
pub use sys::*;


pub mod ffi {
	#![allow(non_upper_case_globals)]
	#![allow(non_camel_case_types)]
	#![allow(non_snake_case)]
	#![cfg_attr(test, allow(deref_nullptr))]
	#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
	//! Low-level Playdate C-API.
	//!
	#![doc = concat!("\nSDK version: `", env!("PD_SDK_VERSION"), "`\n")]
	//!
	//! Original official docs: [latest][], [current][].
	//!
	//! [latest]: https://sdk.play.date/Inside%20Playdate%20with%20C.html
	#![doc = concat!("[current]: https://sdk.play.date/", env!("PD_SDK_VERSION"), "/Inside%20Playdate%20with%20C.html")]
	//!

	// include bindings:
	include!(env!("PD_BINDINGS_PATH"));

	/// Preferred `CString` to use.
	pub use alloc::ffi::CString;
	/// Preferred `CStr` to use.
	pub use core::ffi::CStr;
	pub use core::str::Utf8Error;
}


#[cfg(feature = "entry-point")]
#[no_mangle]
/// Simple minimal proxy entry point.
/// Registers API endpoint when called with `event` matches `PDSystemEvent::kEventInit`.
/// It needed for allocators and panic handler.
///
/// Linking requires rust-abi symbol `event_handler: fn(*const PlaydateAPI, PDSystemEvent, arg: u32) -> c_int`
pub extern "C" fn eventHandlerShim(api: *const ffi::PlaydateAPI,
                                   event: ffi::PDSystemEvent,
                                   arg: u32)
                                   -> core::ffi::c_int {
	extern "Rust" {
		fn event_handler(api: *const ffi::PlaydateAPI, event: ffi::PDSystemEvent, arg: u32) -> EventLoopCtrl;

		#[cfg(not(playdate))]
		// This is atomic in the `sys::proc::error`.
		static END_WITH_ERR: core::sync::atomic::AtomicBool;
	}

	if let ffi::PDSystemEvent::kEventInit = event {
		unsafe { API = api }
	}

	#[cfg(not(playdate))]
	if unsafe { END_WITH_ERR.load(core::sync::atomic::Ordering::Relaxed) } {
		return EventLoopCtrl::Stop.into();
	}

	unsafe { event_handler(api, event, arg) }.into()
}

#[cfg(feature = "entry-point")]
pub use entry_point_ctrl::*;
#[cfg(feature = "entry-point")]
mod entry_point_ctrl {
	use core::ffi::c_int;


	#[repr(i32)]
	pub enum EventLoopCtrl {
		Continue = 0,
		Stop = 1,
	}

	impl EventLoopCtrl {
		pub const fn into(self) -> c_int { self as _ }
	}

	impl From<bool> for EventLoopCtrl {
		fn from(value: bool) -> Self { if value { Self::Continue } else { Self::Stop } }
	}

	impl<T, E> From<Result<T, E>> for EventLoopCtrl {
		fn from(res: Result<T, E>) -> Self { if res.is_ok() { Self::Continue } else { Self::Stop } }
	}

	#[cfg(feature = "try-trait-v2")]
	mod impl_trait_v2 {
		use super::*;
		use core::fmt::Display;
		use core::ops::FromResidual;
		use core::convert::Infallible;

		impl<E: Display> FromResidual<Result<Infallible, E>> for EventLoopCtrl {
			#[track_caller]
			fn from_residual(residual: Result<Infallible, E>) -> Self {
				if let Err(err) = residual {
					crate::println!("Error: {err}");
					// panic!("{err}");
					Self::Stop
				} else {
					Self::Continue
				}
			}
		}
	}
}


pub mod info {
	//! Build info.
	/// Version of the Playdate SDK used for build.
	pub const SDK_VERSION: &str = env!("PD_SDK_VERSION");
}


/// Needed to build debug & executable binaries.
#[cfg(feature = "eh-personality")]
#[lang = "eh_personality"]
#[cfg(all(not(test), not(doc)))]
// #[cfg(not(target = "thumbv7em-none-eabihf"))]
extern "C" fn eh_personality() {}

pub mod misc {
	#[macro_export]
	/// Adds low-level symbols required by gcc for unwinding & exceptions (if `-fno-exceptions` or `-fno-rtti` not set).
	///
	/// There's just dummy- empty- no-op- implementation.
	/// Anyway these symbols will be removed at the final (thanks DCE, LTO, linking with SDK/link_map.ld).
	macro_rules! ll_symbols {
		() => {
			#[doc(hidden)]
			#[no_mangle]
			// GCC unwinding
			pub extern "C" fn __exidx_start() { unimplemented!() }

			#[no_mangle]
			#[doc(hidden)]
			// GCC unwinding
			pub extern "C" fn __exidx_end() { unimplemented!() }

			#[doc(hidden)]
			#[no_mangle]
			#[cfg(not(target_os = "windows"))]
			// there should be loop
			pub extern "C" fn _exit() {}

			#[doc(hidden)]
			#[no_mangle]
			// there should be loop
			pub extern "C" fn _kill() {}

			#[doc(hidden)]
			#[no_mangle]
			pub extern "C" fn _getpid() -> core::ffi::c_int { 0 }

			#[doc(hidden)]
			#[no_mangle]
			// it not needed on MacOS
			#[cfg(not(target_os = "macos"))]
			// TODO: Somehow link with proper impl: https://stackoverflow.com/q/76439798/829264
			pub extern "C" fn _sbrk() {}
		};
	}
}


#[cfg(target_os = "macos")]
#[link(name = "System")]
extern "C" {}
#[cfg(all(target_os = "windows", target_feature = "crt-static"))]
#[link(name = "libcmt")]
extern "C" {}
#[cfg(all(target_os = "windows", not(target_feature = "crt-static")))]
#[link(name = "msvcrt")]
extern "C" {}
