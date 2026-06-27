// https://doc.rust-lang.org/core/ffi/index.html
// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![no_std]
#![cfg_attr(not(test), no_main)]
#![allow(internal_features)]
#![feature(lang_items, core_intrinsics)]
// allocator:
#![cfg_attr(feature = "allocator", feature(alloc_error_handler))]
#![cfg_attr(feature = "allocator-api", feature(allocator_api, slice_ptr_get))]
// const features:
#![cfg_attr(feature = "const-types", feature(adt_const_params))]
// error, ctrl-flow:
#![feature(try_trait_v2)]
// docs:
#![doc(issue_tracker_base_url = "https://github.com/boozook/playdate/issues/")]
// testing:
#![cfg_attr(test, feature(test, try_with_capacity))]
// tracing:
#![feature(const_type_name)]
// utils:
#![feature(const_trait_impl, const_convert)]


#[cfg(test)]
extern crate test;

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod panic;
pub mod allocator;

pub mod print;
pub mod macros;
pub mod error;
pub mod utils;


//
// TODO: `API` could be non-static or thread-local for `cfg(test)` to make possible to run tests with mock in parallel.
//


/// Main unsafe API endpoint.
/// Needed globally for Drop, panic, allocator and macros.
static mut API: *const crate::ffi::Playdate = core::ptr::null_mut();

/// Reference to main (root) API endpoint.
type ApiRef = Option<&'static crate::ffi::Playdate>;

#[inline(always)]
/// Returns reference to main API endpoint.
pub fn api() -> ApiRef { unsafe { API.as_ref() } }

#[inline(always)]
/// Sets main API endpoint.
/// Needed for Drop, panic and allocator.
pub fn set_api(api: *const crate::ffi::Playdate) { unsafe { API = api } }


#[cfg(not(miri))]
#[cfg(not(all(test, mockrt)))]
#[cfg(not(all(test, mockrt = "alloc")))]
#[cfg(not(all(test, mockrt = "std")))]
pub mod ffi {
	#![cfg_attr(test, allow(deref_nullptr))]
	#![allow(unpredictable_function_pointer_comparisons)]
	#![allow(unsafe_attr_outside_unsafe, unsafe_op_in_unsafe_fn)]
	#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
	#![allow(unnecessary_transmutes)]
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
	include!(env!("PD_BINDINGS_PATH"));

	/// Preferred `CString` to use.
	#[cfg(feature = "alloc")]
	pub use alloc::ffi::CString;
	/// Preferred `CStr` to use.
	pub use core::ffi::CStr;


	/// Entry-point type, event handler.
	/// Must be not mangled and symbol named "eventHandlerShim".
	pub type EventHandler =
		extern "C" fn(api: *const Playdate, event: SystemEvent, arg: u32) -> ::core::ffi::c_int;
}


// Mock:
cfg_select! {
	// Replace bindings with mock's for miri or "test with mock" only:
	any(miri, all(test, any(mockrt, mockrt = "alloc", mockrt = "std"))) => {
		/// Runtime Mock
		pub extern crate mock;
		pub use mock::ffi;
	}
	all(miri, all(test, any(mockrt, mockrt = "alloc", mockrt = "std"))) => {
		compile_error!("Because of feature-poisoning, it is an error to use mock with enabled std (cfg: mockrt = \"std\") with miri (cfg: miri)");
	}
	_ => {}
}


#[cfg(test)]
mod allocation {
	//! Just to ensure that all basics is working.
	//! Testing testing system including mock integration.
	// use super::*;


	#[test]
	#[cfg(all(not(miri), any(mockrt, mockrt = "alloc")))]
	fn realloc() {
		use core::ffi::c_void;
		// use mock::ffi::realloc;

		unsafe extern "C" {
			#[link_name = "test_pd_realloc"]
			pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
		}

		let ptr = unsafe { realloc(core::ptr::null_mut(), 1024) };

		assert!(!ptr.is_null());
		assert!(ptr.is_aligned());

		let ptr = unsafe { realloc(core::ptr::null_mut(), 0) };
		assert!(ptr.is_null());
	}

	#[test]
	#[cfg(feature = "alloc")]
	fn alloc() {
		let res = alloc::vec::Vec::try_with_capacity(1024 * 1024).map(|mut vec| {
			                                                         vec.push(42);
			                                                         vec
		                                                         });
		assert!(res.is_ok());

		let mut vec = res.unwrap();
		vec.push(42);
		assert_eq!(2, vec.len());

		drop(vec);
	}
}


/// Simple minimal proxy entry point.
/// Registers API endpoint, that needed for allocators and panic handler,
/// when called with `event` matches [`SystemEvent::Init`].
///
/// Linking requires rust-abi symbol
/// `event_handler: fn(api: &'static `[`Playdate`](ffi::Playdate)`, event: `[`SystemEvent`](ffi::SystemEvent)`, key: u32) -> `[`EventLoopCtrl`](ctrl::EventLoopCtrl).
///
/// ```rust,no_run
/// #![no_std]
/// #[macro_use]
/// extern crate playdate_sys as pd;
/// use pd::ctrl::EventLoopCtrl;
/// use pd::ffi::{Playdate, SystemEvent}
///
/// #[no_mangle]
/// fn event_handler(api: &'static Playdate, event: SystemEvent, key: u32) -> EventLoopCtrl {
/// 	EventLoopCtrl::Stop
/// }
/// ```
#[cfg(feature = "entry-point")]
#[unsafe(no_mangle)]
#[cfg_attr(playdate, unsafe(naked))] // just simple trampoline
pub extern "C" fn eventHandlerShim(api: *const ffi::Playdate,
                                   event: ffi::SystemEvent,
                                   arg: u32)
                                   -> core::ffi::c_int {
	unsafe extern "Rust" {
		safe fn event_handler(api: *const ffi::Playdate, event: ffi::SystemEvent, arg: u32) -> ctrl::EventLoopCtrl;
	}


	#[cfg(not(playdate))]
	{
		if ffi::SystemEvent::Init == event {
			unsafe { API = api }
		}

		if api.is_null() || PANICKED.load(core::sync::atomic::Ordering::Relaxed) {
			ctrl::EventLoopCtrl::Stop
		} else {
			event_handler(api, event, arg)
		}.into()
	}

	// https://blog.rust-lang.org/2025/07/03/stabilizing-naked-functions
	#[cfg(playdate)]
	core::arch::naked_asm!(
		// ARMv7E-M Thumb2 AAPCS: r0=api, r1=event, r2=_
		"cmp     r1, #0",         // event eq 0 (SystemEvent::Init=0)
		"bne     1f",
		"ldr     r3, ={api}",     // load &raw API
		"str     r0, [r3]",       // store api arg into API
		"1:",
		"ldr     r3, ={handler}", // load &raw event_handler
		"bx      r3",             // tail-call (returns c_int directly)
		api     = sym API,
		handler = sym event_handler
	);
}

// This is atomic because the env is the simulator that is asymchronous and built on SDL.
#[cfg(all(feature = "entry-point", not(playdate)))]
static PANICKED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);


pub mod ctrl {
	use core::ffi::{c_uint, c_int};


	/// Event Loop ctrl-flow return value.
	///
	/// Initially it represents command to continue or stop the event loop for the system,
	/// should be returned from event handler [`eventHandlerShim`][].
	/// In those days, specific values meant `0` is for ok and `1` is for error.
	///
	/// But seems to in actual version of PdOs this behavior is disabled.
	/// So actually for the PdOs it doesn’t matter what value is used 🤷‍♂️
	/// _but it can be implemented in some proxy-level event-loop._
	///
	/// [`eventHandlerShim`]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_game_initialization
	#[repr(i32)]
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum EventLoopCtrl {
		/// Continue normally.
		Continue = 0,
		/// Stop the event loop because of an error.
		Stop = 1,
	}

	impl From<EventLoopCtrl> for c_int {
		fn from(v: EventLoopCtrl) -> Self { v as _ }
	}
	impl From<c_int> for EventLoopCtrl {
		fn from(v: c_int) -> Self { if v == 0 { Self::Continue } else { Self::Stop } }
	}
	impl From<EventLoopCtrl> for c_uint {
		fn from(v: EventLoopCtrl) -> Self { v as _ }
	}
	impl From<c_uint> for EventLoopCtrl {
		fn from(v: c_uint) -> Self { if v == 0 { Self::Continue } else { Self::Stop } }
	}
	impl From<EventLoopCtrl> for bool {
		fn from(v: EventLoopCtrl) -> Self { matches!(v, EventLoopCtrl::Continue) }
	}
	impl From<bool> for EventLoopCtrl {
		fn from(v: bool) -> Self { unsafe { core::mem::transmute(v as i32) } }
	}


	/// Update callback return value - signal to update the display for the PdOs.
	///
	/// This should be returned from update-callback registered with [`setUpdateCallback`].
	///
	/// Starting from [PdOs v1.12][changelog] the update function should return a non-zero number to tell the system to update the display,
	/// or zero if update isn’t needed.
	/// _Works on hardware_, but may not in simulator.
	///
	/// [changelog]: https://sdk.play.date/changelog/#_1_12_0
	/// [`setUpdateCallback`]: crate::ffi::PlaydateSys::setUpdateCallback
	#[repr(i32)]
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum UpdateDisplayCtrl {
		/// Tell the system that to update the display isn’t needed.
		Nope = 0,
		/// Tell the system to update the display.
		Needed = 1,
	}

	impl From<()> for UpdateDisplayCtrl {
		fn from(_: ()) -> Self { Self::Nope }
	}

	impl From<UpdateDisplayCtrl> for c_int {
		fn from(v: UpdateDisplayCtrl) -> Self { v as _ }
	}
	impl From<c_int> for UpdateDisplayCtrl {
		fn from(v: c_int) -> Self { if v == 0 { Self::Nope } else { Self::Needed } }
	}
	impl From<UpdateDisplayCtrl> for c_uint {
		fn from(v: UpdateDisplayCtrl) -> Self { v as _ }
	}
	impl From<c_uint> for UpdateDisplayCtrl {
		fn from(v: c_uint) -> Self { if v == 0 { Self::Nope } else { Self::Needed } }
	}
	impl From<UpdateDisplayCtrl> for bool {
		fn from(v: UpdateDisplayCtrl) -> Self { matches!(v, UpdateDisplayCtrl::Needed) }
	}
	impl From<bool> for UpdateDisplayCtrl {
		fn from(v: bool) -> Self { unsafe { core::mem::transmute(v as i32) } }
	}


	mod impl_try {
		use core::fmt::Display;
		use core::ops::FromResidual;
		use core::convert::Infallible;
		use crate::macros::{api, api_opt};
		use super::*;


		impl core::ops::Try for EventLoopCtrl {
			type Output = Self;
			fn from_output(output: Self::Output) -> Self { Self::from(output) }

			type Residual = c_int;
			fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
				if matches!(self, Self::Stop) {
					core::ops::ControlFlow::Break(self.into())
				} else {
					core::ops::ControlFlow::Continue(self)
				}
			}
		}

		impl FromResidual<c_int> for EventLoopCtrl {
			#[track_caller]
			fn from_residual(residual: c_int) -> Self {
				let res = EventLoopCtrl::from(residual);
				if res == Self::Stop {
					sim_try_full_stop();
					panic!("{res:?}");
				}
				res
			}
		}

		impl<E: Display> FromResidual<Result<Infallible, E>> for EventLoopCtrl {
			#[track_caller]
			fn from_residual(residual: Result<Infallible, E>) -> Self {
				sim_try_full_stop();
				panic!("{}", unsafe { residual.unwrap_err_unchecked() });
			}
		}


		fn sim_try_full_stop() {
			if api_opt!(graphics.getDebugBitmap).flatten().is_some() {
				unsafe { api!(system.setUpdateCallback)(None, core::ptr::null_mut()) }
			}
		}
	}
}


//
// ‼ following is needed when building with arm-gcc and linking with its stdlib
//
// TODO: refactor, rename `ll_symbols` and others


/// Needed to build debug & executable binaries.
#[cfg(feature = "eh-personality")]
#[lang = "eh_personality"]
#[cfg(all(not(test), not(doc)))]
// #[cfg(not(target = "thumbv7em-none-eabihf"))]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
pub mod misc {
	#[macro_export]
	/// Adds low-level symbols required by gcc for unwinding & exceptions (if `-fno-exceptions` or `-fno-rtti` not set).
	///
	/// There's just dummy- empty- no-op- implementation.
	/// Anyway these symbols may be removed at the final (thanks DCE, LTO, linking with SDK/link_map.ld).
	///
	/// __Needed for device target when building with arm-gcc and linking with its stdlib.__
	macro_rules! ll_symbols {
		() => {
			#[doc(hidden)]
			#[unsafe(no_mangle)]
			// GCC unwinding
			pub extern "C" fn __exidx_start() { unimplemented!() }

			#[unsafe(no_mangle)]
			#[doc(hidden)]
			// GCC unwinding
			pub extern "C" fn __exidx_end() { unimplemented!() }

			#[doc(hidden)]
			#[unsafe(no_mangle)]
			#[cfg(not(target_os = "windows"))]
			// there should be loop
			pub extern "C" fn _exit() {}

			#[doc(hidden)]
			#[unsafe(no_mangle)]
			// there should be loop
			pub extern "C" fn _kill() {}

			#[doc(hidden)]
			#[unsafe(no_mangle)]
			pub extern "C" fn _getpid() -> core::ffi::c_int { 0 }

			#[doc(hidden)]
			#[unsafe(no_mangle)]
			// it not needed on MacOS
			#[cfg(not(target_os = "macos"))]
			// TODO: Somehow link with proper impl: https://stackoverflow.com/q/76439798/829264
			pub extern "C" fn _sbrk() {}
		};
	}
}

#[cfg(not(test))]
#[cfg(target_os = "macos")]
#[link(name = "System")]
unsafe extern "C" {}
#[cfg(all(target_os = "windows", target_feature = "crt-static"))]
#[link(name = "libcmt")]
unsafe extern "C" {}
#[cfg(all(target_os = "windows", not(target_feature = "crt-static")))]
#[link(name = "msvcrt")]
unsafe extern "C" {}
