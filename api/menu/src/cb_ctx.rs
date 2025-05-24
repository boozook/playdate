use alloc::boxed::Box;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::transmute;
use core::pin::Pin;
use core::ptr::null_mut;
use core::ptr::NonNull;
use sys::ffi::CStr;
use sys::ffi::MenuItem as PDMenuItem;

use crate::kind;
use crate::error::Error;
use crate::Api;
use super::MenuItem;
use super::MenuItemRef;
use super::RemoveOnDrop;


// Boxed self with inner callback and self-ref via c-side userdata

impl<K, F> MenuItem<K, F>
	where K: kind::Kind,
	      F: FnMut(&MenuItemRef<K>)
{
	pub(super) unsafe extern "C" fn proxy_self(userdata: *mut c_void) {
		if let Some(this) = (userdata.cast::<Self>()).as_mut() {
			let item = MenuItemRef(this.as_ptr(), PhantomData);
			let cb = &mut this.2;
			cb(&item);
		} else {
			panic!("missed userdata")
		}
	}
}

impl<K, F: FnMut(&MenuItemRef<K>)> MenuItem<K, F> {
	/// Wrap ptr into this and set userdata=this
	pub(super) fn from_and_set_ud(api: Api, item: *mut PDMenuItem, callback: F) -> Result<Pin<Box<Self>>, Error> {
		NonNull::new(item).map(RemoveOnDrop)
		                  .map(move |ptr| {
			                  let item = MenuItem(ptr, PhantomData, callback);
			                  let mut item = Box::pin(item);

			                  let pinned: *mut MenuItem<kind::Simple, F> = unsafe { transmute(item.as_mut()) };
			                  unsafe { (api.setMenuItemUserdata)(item.as_raw_ptr(), pinned.cast()) };

			                  item
		                  })
		                  .ok_or(Error)
	}
}


impl<F: FnMut(&MenuItemRef<kind::Simple>)> MenuItem<kind::Simple, F> {
	/// Creates a new menu item with callback like FnMut(&SimpleMenuItemRef).
	pub fn new_with_ctx(api: Api, title: impl AsRef<CStr>, callback: F) -> Result<Pin<Box<Self>>, Error> {
		let title = title.as_ref();
		let ctor = api.addMenuItem;
		let item = unsafe { ctor(title.as_ptr(), Some(Self::proxy_self), null_mut()) };
		Self::from_and_set_ud(api, item, callback)
	}
}


impl<F: FnMut(&MenuItemRef<kind::Check>)> MenuItem<kind::Check, F> {
	/// Creates a new check- menu item with callback.
	pub fn new_with_ctx(api: Api,
	                    title: impl AsRef<CStr>,
	                    checked: bool,
	                    callback: F)
	                    -> Result<Pin<Box<Self>>, Error> {
		let title = title.as_ref();
		let ctor = api.addCheckmarkMenuItem;
		let item = unsafe { ctor(title.as_ptr(), checked as _, Some(Self::proxy_self), null_mut()) };
		Self::from_and_set_ud(api, item, callback)
	}
}


use alloc::vec::Vec;
impl<F: FnMut(&MenuItemRef<kind::Options>)> MenuItem<kind::Options, F> {
	/// Creates a new choice- menu item with callback.
	pub fn new_with_ctx<S: AsRef<CStr>>(api: Api,
	                                    title: impl AsRef<CStr>,
	                                    options: impl AsRef<[S]>,
	                                    callback: F)
	                                    -> Result<Pin<Box<Self>>, Error> {
		let title = title.as_ref();
		let options = options.as_ref();

		let mut opts = Vec::with_capacity(options.len());
		opts.extend(options.iter().map(|s| s.as_ref().as_ptr()));

		let ctor = api.addOptionsMenuItem;
		let item = unsafe {
			ctor(
			     title.as_ptr(),
			     opts.as_mut_ptr().cast(),
			     opts.len() as _,
			     Some(Self::proxy_self),
			     null_mut(),
			)
		};

		drop(opts);
		Self::from_and_set_ud(api, item, callback)
	}
}
