use core::ffi::*;
use crate::ffi::*;


#[cfg(miri)]
use self::miri::*;
#[cfg(miri)]
mod miri {
	use super::*;
	use crate::miri::*;

	#[export_name = "test_pd_realloc"] // prevent collision with OS's realloc
	pub unsafe extern "C" fn realloc(p: *mut c_void, size: usize) -> *mut c_void {
		const ALIGN: usize = align_of::<usize>();

		if size == 0 {
			unimplemented!("miri can't deal with unknown size");
			miri_dealloc(p.cast(), size, ALIGN);
			core::ptr::null_mut()
		} else {
			miri_alloc(size, ALIGN).cast()
		}
	}


	pub unsafe extern "C" fn error(fmt: *const c_char, ...) -> ! {
		let s = CStr::from_ptr(fmt);
		miri_write_to_stderr(s.to_bytes());
		core::intrinsics::abort()
	}

	pub unsafe extern "C" fn logToConsole(fmt: *const c_char, ...) {
		let s = CStr::from_ptr(fmt);
		miri_write_to_stdout(s.to_bytes());
	}
}


#[cfg(not(miri))]
use self::host::realloc;
#[cfg(not(any(miri, test, feature = "std")))]
use self::host::*;
#[cfg(not(miri))]
mod host {
	use super::*;
	use core::alloc::Layout;
	use ::alloc::alloc;


	#[export_name = "test_pd_realloc"] // prevent collision with OS's realloc
	pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
		// {
		// 	println!("realloc: {ptr:?} {size}");
		// }
		const ALIGN: usize = align_of::<usize>();

		let align = ALIGN.max(size_of::<usize>());

		// do we need to track allocated pointers because of unknown old size?
		//                                              for realloc & dealloc.

		if size == 0 {
			let layout = Layout::from_size_align_unchecked(0, align);
			alloc::dealloc(ptr.cast(), layout);
			core::ptr::null_mut()
		} else {
			if ptr.is_null() {
				let layout = Layout::from_size_align_unchecked(size, align);
				alloc::alloc(layout).cast()
			} else {
				let layout = Layout::from_size_align_unchecked(0, align);
				alloc::realloc(ptr.cast(), layout, size).cast()
			}
		}
	}


	pub unsafe extern "C" fn error(_: *const c_char, ...) -> ! { core::intrinsics::abort() }
	pub unsafe extern "C" fn logToConsole(_: *const c_char, ...) {}
}


#[cfg(not(miri))]
#[cfg(any(test, feature = "std"))]
use self::std::*;
#[cfg(not(miri))]
#[cfg(any(test, feature = "std"))]
mod std {
	use super::*;
	pub use host::realloc;

	pub unsafe extern "C" fn error(fmt: *const c_char, ...) -> ! {
		let s = CStr::from_ptr(fmt);
		eprintln!("panicing with: {s:?}");
		core::intrinsics::abort()
	}

	pub unsafe extern "C" fn logToConsole(fmt: *const c_char, ...) {
		let s = CStr::from_ptr(fmt);
		let s = s.to_string_lossy();
		println!("{}", s.trim_end());
	}
}


static mut UPDATE: Option<(CallbackFunction, *mut c_void)> = None;

pub fn call_update() -> Option<c_int> {
	if let Some((Some(update), userdata)) = unsafe { UPDATE.as_ref() } {
		let res = unsafe { update(*userdata) };
		Some(res)
	} else {
		None
	}
}

unsafe extern "C" fn setUpdateCallback(update: CallbackFunction, userdata: *mut c_void) {
	#[cfg(miri)]
	{
		crate::miri::miri_write_to_stdout("call set_update_callback\n".as_ref());
		crate::miri::miri_run_provenance_gc();
	}

	// mark
	#[cfg(miri)]
	if !userdata.is_null() {
		use crate::miri::*;
		miri_pointer_name(userdata.cast(), 0, c"userdata".to_bytes());

		// track:
		let id = miri_get_alloc_id(userdata.cast());
		miri_print_borrow_state(id, true);
	}


	if let Some((_, userdata)) = UPDATE.replace((update, userdata)) {
		// mark
		#[cfg(miri)]
		if !userdata.is_null() {
			use crate::miri::*;
			miri_pointer_name(userdata.cast(), 0, c"replaced".to_bytes());

			// track:
			miri_run_provenance_gc();
			let id = crate::miri::miri_get_alloc_id(userdata.cast());
			miri_print_borrow_state(id, true);
		}
	}
}


unsafe extern "C" fn formatString(ret: *mut *mut c_char, fmt: *const c_char, ...) -> c_int { 0 }


unsafe extern "C" fn drawFPS(x: core::ffi::c_int, y: core::ffi::c_int) {}


static mut BTN_STATE: (Buttons, Buttons, Buttons) = (Buttons(0), Buttons(0), Buttons(0));
pub fn setButtonState(current: Buttons, pushed: Buttons, released: Buttons) {
	unsafe {
		BTN_STATE = (current, pushed, released);
	}
}

unsafe extern "C" fn getButtonState(current: *mut Buttons, pushed: *mut Buttons, released: *mut Buttons) {
	unsafe {
		if !current.is_null() {
			#[cfg(miri)]
			crate::miri::miri_pointer_name(current.cast(), 0, c"btns-current".to_bytes());
			*current = BTN_STATE.0.clone();
		}

		if !pushed.is_null() {
			#[cfg(miri)]
			crate::miri::miri_pointer_name(current.cast(), 0, c"btns-pushed".to_bytes());
			*pushed = BTN_STATE.1.clone();
		}

		if !released.is_null() {
			#[cfg(miri)]
			crate::miri::miri_pointer_name(current.cast(), 0, c"btns-released".to_bytes());
			*released = BTN_STATE.2.clone();
		}
	}
}


pub static SYSTEM: PlaydateSys = PlaydateSys { realloc,
                                             //   formatString,
                                               logToConsole,
                                               error,
                                             //   getLanguage: None,
                                             //   getCurrentTimeMilliseconds: None,
                                             //   getSecondsSinceEpoch: None,
                                               drawFPS,
                                               setUpdateCallback,
                                               getButtonState,
                                             //   setPeripheralsEnabled: None,
                                             //   getAccelerometer: None,
                                             //   getCrankChange: None,
                                             //   getCrankAngle: None,
                                             //   isCrankDocked: None,
                                             //   setCrankSoundsDisabled: None,
                                             //   getFlipped: None,
                                             //   setAutoLockDisabled: None,
                                             //   setMenuImage: None,
                                             //   addMenuItem: None,
                                             //   addCheckmarkMenuItem: None,
                                             //   addOptionsMenuItem: None,
                                             //   removeAllMenuItems: None,
                                             //   removeMenuItem: None,
                                             //   getMenuItemValue: None,
                                             //   setMenuItemValue: None,
                                             //   getMenuItemTitle: None,
                                             //   setMenuItemTitle: None,
                                             //   getMenuItemUserdata: None,
                                             //   setMenuItemUserdata: None,
                                             //   getReduceFlashing: None,
                                             //   getElapsedTime: None,
                                             //   resetElapsedTime: None,
                                             //   getBatteryPercentage: None,
                                             //   getBatteryVoltage: None,
                                             //   getTimezoneOffset: None,
                                             //   shouldDisplay24HourTime: None,
                                             //   convertEpochToDateTime: None,
                                             //   convertDateTimeToEpoch: None,
                                             //   clearICache: None,
                                             //   setButtonCallback: None,
                                             //   setSerialMessageCallback: None,
                                             //   vaFormatString: None,
                                             //   parseString: None,
                                             //   delay: None,
                                             //   getServerTime: None,
                                             //   sendMirrorData: todo!()
															};
