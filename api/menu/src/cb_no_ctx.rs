use alloc::boxed::Box;
use core::ffi::c_void;
use core::pin::Pin;
use sys::ffi::CStr;

use crate::kind;
use crate::error::Error;
use crate::Api;
use super::MenuItem;


// Boxed inner callback

impl<K, F> MenuItem<K, Pin<Box<F>>>
	where K: kind::Kind,
	      F: FnMut()
{
	unsafe extern "C" fn proxy(callback: *mut c_void) {
		if let Some(f) = (callback as *mut F).as_mut() {
			f()
		} else {
			panic!("missed userdata")
		}
	}
}


impl<F: FnMut()> MenuItem<kind::Simple, Pin<Box<F>>> {
	/// Creates a new menu item with callback.
	pub fn new_with(api: Api, title: impl AsRef<CStr>, callback: F) -> Result<Self, Error> {
		let title = title.as_ref();

		// box-pin callback
		let mut ud = Box::pin(callback);
		let pinned = ud.as_mut();
		let cbptr: *mut F = unsafe { core::mem::transmute(pinned) };

		let ctor = api.addMenuItem;
		let ptr = unsafe { ctor(title.as_ptr(), Some(Self::proxy), cbptr.cast()) };

		Self::from_with(ptr, ud)
	}
}


impl<F: FnMut()> MenuItem<kind::Check, Pin<Box<F>>> {
	/// Creates a new check- menu item with callback.
	pub fn new_with(api: Api, title: impl AsRef<CStr>, checked: bool, callback: F) -> Result<Self, Error> {
		let title = title.as_ref();

		// box-pin callback
		let mut ud = Box::pin(callback);
		let pinned = ud.as_mut();
		let cbptr: *mut F = unsafe { core::mem::transmute(pinned) };

		let ctor = api.addCheckmarkMenuItem;
		let ptr = unsafe { ctor(title.as_ptr(), checked as _, Some(Self::proxy), cbptr.cast()) };

		Self::from_with(ptr, ud)
	}
}


use alloc::vec::Vec;
impl<F: FnMut()> MenuItem<kind::Options, Pin<Box<F>>> {
	/// Creates a new choice- menu item with callback.
	pub fn new_with<S: AsRef<CStr>>(api: Api,
	                                title: impl AsRef<CStr>,
	                                options: impl AsRef<[S]>,
	                                callback: F)
	                                -> Result<Self, Error> {
		let title = title.as_ref();
		let options = options.as_ref();

		let mut opts = Vec::with_capacity(options.len());
		opts.extend(options.iter().map(|s| s.as_ref().as_ptr()));

		// box-pin callback
		let mut ud = Box::pin(callback);
		let pinned = ud.as_mut();
		let cbptr: *mut F = unsafe { core::mem::transmute(pinned) };

		let ctor = api.addOptionsMenuItem;
		let ptr = unsafe {
			ctor(
			     title.as_ptr(),
			     opts.as_mut_ptr().cast(),
			     opts.len() as _,
			     Some(Self::proxy),
			     cbptr.cast(),
			)
		};

		drop(opts);

		Self::from_with(ptr, ud)
	}
}
