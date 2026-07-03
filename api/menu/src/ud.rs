use core::ptr::null;
use core::ptr::null_mut;
use sys::ffi::CStr;

use crate::kind;
use crate::error::Error;
use crate::Api;
use super::MenuItem;


impl<T> MenuItem<kind::Simple, T> {
	/// Creates a new simple menu item without callback.
	pub fn new(api: Api, title: impl AsRef<CStr>, userdata: T) -> Result<Self, Error> {
		let title = title.as_ref();

		let ctor = api.addMenuItem;
		let ptr = unsafe { ctor(title.as_ptr(), None, null_mut()) };

		Self::from_with(ptr, userdata)
	}
}

impl<T> MenuItem<kind::Check, T> {
	/// Creates a new check- menu item without callback.
	pub fn new(api: Api, title: impl AsRef<CStr>, checked: bool, userdata: T) -> Result<Self, Error> {
		let title = title.as_ref();

		let ctor = api.addCheckmarkMenuItem;
		let ptr = unsafe { ctor(title.as_ptr(), checked as _, None, null_mut()) };

		Self::from_with(ptr, userdata)
	}
}


use alloc::vec::Vec;
impl<T> MenuItem<kind::Options, T> {
	/// Creates a new choice- menu item without callback.
	pub fn new<S: AsRef<CStr>>(api: Api,
	                           title: impl AsRef<CStr>,
	                           options: impl AsRef<[S]>,
	                           userdata: T)
	                           -> Result<Self, Error> {
		let title = title.as_ref();
		let options = options.as_ref();

		let mut opts = Vec::with_capacity(options.len());
		opts.extend(options.iter().map(|s| s.as_ref().as_ptr()));

		let ctor = api.addOptionsMenuItem;
		let ptr = unsafe {
			ctor(
			     title.as_ptr(),
			     opts.as_mut_ptr().cast(),
			     opts.len() as _,
			     None,
			     null_mut(),
			)
		};

		drop(opts);

		Self::from_with(ptr, userdata)
	}


	/// Creates a new choice- menu item without callback.
	pub fn new_exact<const N: usize, S: AsRef<CStr>>(api: Api,
	                                                 title: impl AsRef<CStr>,
	                                                 options: &[S; N],
	                                                 userdata: T)
	                                                 -> Result<Self, Error> {
		let title = title.as_ref();

		let mut opts = [null(); N];
		options.iter()
		       .zip(opts.iter_mut())
		       .for_each(|(s, p)| *p = s.as_ref().as_ptr());

		let ctor = api.addOptionsMenuItem;
		let ptr = unsafe {
			ctor(
			     title.as_ptr(),
			     opts.as_mut_ptr().cast(),
			     opts.len() as _,
			     None,
			     null_mut(),
			)
		};

		Self::from_with(ptr, userdata)
	}
}
