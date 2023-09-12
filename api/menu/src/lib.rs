#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]

extern crate sys;
extern crate alloc;

pub mod error;


use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::ffi::NulError;
use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_void;
use core::marker::PhantomData;
use sys::ffi::PDMenuItemCallbackFunction;
use sys::ffi::PDMenuItem;
use sys::ffi::CStr;
use sys::ffi::CString;

use error::{Error, ApiError};


pub type SimpleMenuItem<UserData = (), Api = api::Default, const REMOVE_ON_DROP: bool = true> =
	MenuItem<kind::Simple, UserData, Api, REMOVE_ON_DROP>;

pub type CheckMenuItem<UserData = (), Api = api::Default, const REMOVE_ON_DROP: bool = true> =
	MenuItem<kind::Check, UserData, Api, REMOVE_ON_DROP>;

pub type OptionsMenuItem<UserData = (), Api = api::Default, const REMOVE_ON_DROP: bool = true> =
	MenuItem<kind::Options, UserData, Api, REMOVE_ON_DROP>;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct MenuItem<Kind, UserData = (), Api= api::Default, const REMOVE_ON_DROP: bool = true>(*mut PDMenuItem, Api, PhantomData<Kind>, PhantomData<UserData>) where Kind: kind::Kind, UserData: Into<Box<UserData>>, Api: api::Api;


impl<UD, K: kind::Kind, Api: api::Api, const REM: bool> MenuItem<K, UD, Api, REM> {
	pub fn get_title(&self) -> Cow<'_, str> {
		let f = self.1.get_menu_item_title();
		unsafe { CStr::from_ptr(f(self.0) as _) }.to_string_lossy()
	}

	pub fn set_title<S: AsRef<str>>(&self, title: S) -> Result<(), NulError> {
		let f = self.1.set_menu_item_title();
		let s = CString::new(title.as_ref())?;
		unsafe { f(self.0, s.as_ptr() as *mut c_char) };
		core::mem::drop(s);
		Ok(())
	}

	pub fn get_userdata(&self) -> Option<&mut UD> { self.get_userdata_full().map(|(_, ud)| ud) }

	fn get_userdata_full(&self) -> Option<&mut CallbackUserData<UD>> {
		let f = self.1.get_menu_item_userdata();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null()
		/* TODO: or miss-aligned */
		{
			return None;
		}

		unsafe { (ptr as *mut CallbackUserData<UD>).as_mut() }
	}

	pub fn set_userdata(&self, userdata: UD) -> Option<UD> {
		if let Some(existing) = self.get_userdata() {
			// *ud = userdata
			core::mem::replace(existing, userdata).into()
		} else {
			todo!()
		}
	}

	pub fn get_value(&self) -> c_int {
		let f = self.1.get_menu_item_value();
		unsafe { f(self.0) }
	}

	pub fn set_value(&self, value: c_int) {
		let f = self.1.set_menu_item_value();
		unsafe { f(self.0, value) }
	}


	fn take_userdata(&mut self) -> Option<UD> {
		if self.0.is_null() {
			return None;
		}

		let f = self.1.get_menu_item_userdata();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			return None;
		} else if core::mem::size_of::<UD>() == 0
		// || ptr.addr() & (core::mem::align_of::<CallbackUserData<UD>>() - 1) != 0
		{
			// invalid pointer, mostly means that the userdata was not set/initialized
			return None;
		}

		let ud: CallbackUserData<UD> = *unsafe { Box::from_raw(ptr as *mut CallbackUserData<UD>) };
		let (_, userdata) = ud;
		Some(userdata)
	}
}


impl<UD: Sized, Api: api::Api, const REM: bool> MenuItem<kind::Check, UD, Api, REM> {
	#[inline(always)]
	pub fn new<S: AsRef<str>>(title: S,
	                          checked: bool,
	                          callback: Option<MenuItemCallback<UD>>,
	                          userdata: UD)
	                          -> Result<Self, ApiError>
		where Api: Default
	{
		Self::new_with(Api::default(), title, checked, callback, userdata)
	}

	pub fn new_with<S: AsRef<str>>(api: Api,
	                               title: S,
	                               checked: bool,
	                               callback: Option<MenuItemCallback<UD>>,
	                               userdata: UD)
	                               -> Result<Self, ApiError> {
		let (callback, userdata) = proxy_menu_parts::<_, _>(callback, userdata);
		let title = CString::new(title.as_ref())?;

		let ctor = api.add_checkmark_menu_item();
		let ptr = unsafe { ctor(title.as_ptr() as *mut c_char, checked as _, callback, userdata) };

		if ptr.is_null() {
			Err(Error::Alloc.into())
		} else {
			Ok(MenuItem(ptr, api, PhantomData, PhantomData))
		}
	}
}


impl<UD: Sized, Api: api::Api, const REM: bool> MenuItem<kind::Simple, UD, Api, REM> {
	#[inline(always)]
	pub fn new<S: AsRef<str>>(title: S,
	                          callback: Option<MenuItemCallback<UD>>,
	                          userdata: UD)
	                          -> Result<Self, ApiError>
		where Api: Default
	{
		Self::new_with(Api::default(), title, callback, userdata)
	}

	pub fn new_with<S: AsRef<str>>(api: Api,
	                               title: S,
	                               callback: Option<MenuItemCallback<UD>>,
	                               userdata: UD)
	                               -> Result<Self, ApiError> {
		let (callback, userdata) = proxy_menu_parts::<_, _>(callback, userdata);
		let title = CString::new(title.as_ref())?;

		let ctor = api.add_menu_item();
		let ptr = unsafe { ctor(title.as_ptr() as *mut c_char, callback, userdata) };

		if ptr.is_null() {
			Err(Error::Alloc.into())
		} else {
			Ok(MenuItem(ptr, api, PhantomData, PhantomData::<UD>))
		}
	}
}

impl<UD: Sized, Api: api::Api, const REM: bool> MenuItem<kind::Options, UD, Api, REM> {
	#[inline(always)]
	pub fn new<S: AsRef<str>, O: AsRef<[S]>>(title: S,
	                                         options: O,
	                                         callback: Option<MenuItemCallback<UD>>,
	                                         userdata: UD)
	                                         -> Result<Self, ApiError>
		where Api: Default
	{
		Self::new_with(Api::default(), title, options, callback, userdata)
	}

	pub fn new_with<S: AsRef<str>, O: AsRef<[S]>>(api: Api,
	                                              title: S,
	                                              options: O,
	                                              callback: Option<MenuItemCallback<UD>>,
	                                              userdata: UD)
	                                              -> Result<Self, ApiError> {
		use alloc::vec::Vec;


		let (callback, userdata) = proxy_menu_parts::<_, _>(callback, userdata);
		let title = CString::new(title.as_ref())?;

		let options = options.as_ref();
		let mut opts = Vec::with_capacity(options.len());
		for opt in options {
			let opt = CString::new(opt.as_ref())?;
			opts.push(opt)
		}
		let mut ptrs = Vec::with_capacity(options.len());
		ptrs.extend(opts.iter().map(|s| s.as_ptr()));

		let ctor = api.add_options_menu_item();
		let ptr = unsafe {
			ctor(
			     title.as_ptr() as *mut c_char,
			     ptrs.as_mut_ptr() as _,
			     ptrs.len() as _,
			     callback,
			     userdata,
			)
		};

		core::mem::drop(ptrs);
		core::mem::drop(opts);

		if ptr.is_null() {
			Err(Error::Alloc.into())
		} else {
			Ok(MenuItem(ptr, api, PhantomData, PhantomData::<UD>))
		}
	}
}


#[inline(always)]
fn proxy_menu_parts<UD: Sized, F: FnMut(&mut UD)>(cb: Option<F>,
                                                  ud: UD)
                                                  -> (PDMenuItemCallbackFunction, *mut c_void) {
	unsafe extern "C" fn proxy<UserData: Sized>(userdata: *mut c_void) {
		if let Some((callback, userdata)) = (userdata as *mut CallbackUserData<UserData>).as_mut() {
			callback(userdata)
		} else {
			panic!("user callback missed");
		}
	}

	if let Some(callback) = cb {
		// convert (callback, userdata) -> pointer:
		let ptr = Box::into_raw(Box::from((callback, ud)));
		(Some(proxy::<UD> as _), ptr as *mut _)
	} else {
		// we can get user data smaller:
		// convert userdata -> pointer:
		// let ptr = Box::into_raw(Box::from(userdata));
		// Ok((None, ptr as _))

		// but better to have same for consistency,
		// required for get/set userdata:
		fn noop<UserData: Sized>(_: &mut UserData) {}
		let ptr = Box::into_raw(Box::from((noop::<UD>, ud)));
		(None, ptr as *mut _)
	}
}


impl<UD, Api: api::Api, const REM: bool> MenuItem<kind::Check, UD, Api, REM> {
	#[inline(always)]
	pub fn is_checked(&self) -> bool { self.get_value() == 1 }
}


impl<UD, Api: api::Api, const REM: bool> MenuItem<kind::Options, UD, Api, REM> {
	#[inline(always)]
	/// The array index of the currently selected option.
	pub fn selected_option(&self) -> i32 { self.get_value() }
}


impl<UD, K: kind::Kind, Api: api::Api, const REM: bool> Drop for MenuItem<K, UD, Api, REM> {
	fn drop(&mut self) {
		if REM && !self.0.is_null() {
			// we have to drop userdata:
			self.take_userdata();
			let f = self.1.remove_menu_item();
			unsafe { f(self.0) };
		}
	}
}

impl<UD, K: kind::Kind, Api: api::Api, const REM: bool> MenuItem<K, UD, Api, REM> {
	#[inline(always)]
	pub fn remove(mut self) -> Option<UD> {
		let ud = self.take_userdata();

		let f = self.1.remove_menu_item();
		unsafe { f(self.0) };
		self.0 = core::ptr::null_mut() as _;

		ud
	}
}


pub fn remove_all_menu_items() {
	use api::Api;
	let f = api::Default::default().remove_all_menu_items();
	unsafe { f() };
}


type CallbackUserData<UserData> = (MenuItemCallback<UserData>, UserData);
type MenuItemCallback<T> = fn(userdata: &mut T);


pub mod kind {
	pub trait Kind {}


	#[derive(Debug)]
	pub struct Simple;

	#[derive(Debug)]
	pub struct Check;

	#[derive(Debug)]
	pub struct Options;

	impl Kind for Simple {}
	impl Kind for Check {}
	impl Kind for Options {}
}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_int;
	use core::ffi::c_void;
	use sys::ffi::PDMenuItem;
	use sys::ffi::PDMenuItemCallbackFunction;


	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;

	impl Api for Default {}


	pub trait Api {
		fn add_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addMenuItem)
		}

		fn add_checkmark_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        value: c_int,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addCheckmarkMenuItem)
		}

		fn add_options_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        optionTitles: *mut *const c_char,
			                        optionsCount: c_int,
			                        f: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addOptionsMenuItem)
		}

		fn remove_menu_item(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) {
			*sys::api!(system.removeMenuItem)
		}

		fn get_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> c_int {
			*sys::api!(system.getMenuItemValue)
		}

		fn set_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: c_int) {
			*sys::api!(system.setMenuItemValue)
		}

		fn get_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char {
			*sys::api!(system.getMenuItemTitle)
		}

		fn set_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char) {
			*sys::api!(system.setMenuItemTitle)
		}

		fn get_menu_item_userdata(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void {
			*sys::api!(system.getMenuItemUserdata)
		}

		fn remove_all_menu_items(&self) -> unsafe extern "C" fn() { *sys::api!(system.removeAllMenuItems) }
	}
}
