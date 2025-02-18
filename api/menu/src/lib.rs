#![cfg_attr(not(test), no_std)]

#[macro_use]
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
use gfx::bitmap::AnyBitmap;

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
	/// Gets the display title of the menu item.
	///
	/// Returns [`sys::ffi::playdate_sys::getMenuItemTitle`]
	#[doc(alias = "sys::ffi::playdate_sys::getMenuItemTitle")]
	pub fn title(&self) -> Cow<'_, str> {
		let f = self.1.get_menu_item_title();
		unsafe { CStr::from_ptr(f(self.0) as _) }.to_string_lossy()
	}

	/// Sets the display title of the menu item.
	///
	/// Returns [`sys::ffi::playdate_sys::setMenuItemTitle`]
	#[doc(alias = "sys::ffi::playdate_sys::setMenuItemTitle")]
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
		/* TODO: check ptr is miss-aligned */
		{
			return None;
		}

		unsafe { (ptr as *mut CallbackUserData<UD>).as_mut() }
	}

	/// Set `userdata`, replace and return old userdata.
	pub fn set_userdata(&self, userdata: UD) -> Option<UD> {
		if let Some(existing) = self.get_userdata() {
			core::mem::replace(existing, userdata).into()
		} else {
			todo!()
		}
	}

	/// Gets the integer value of the menu item.
	///
	/// See also [`MenuItem::set_value`].
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::playdate_sys::getMenuItemValue")]
	pub fn value(&self) -> c_int {
		let f = self.1.get_menu_item_value();
		unsafe { f(self.0) }
	}

	/// Sets the integer value of the menu item.
	///
	/// For checkmark menu items ([`CheckMenuItem`]), `1` means checked, `0` unchecked.
	///
	/// For option menu items ([`OptionsMenuItem`]), the value indicates the array index of the currently selected option.
	///
	/// See also [`CheckMenuItem::is_checked`], [`OptionsMenuItem::selected_option`].
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setMenuItemValue`]
	#[doc(alias = "sys::ffi::playdate_sys::setMenuItemValue")]
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
		#[allow(unused_imports)]
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
	/// Equivalent to [`sys::ffi::playdate_sys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::playdate_sys::getMenuItemValue")]
	#[inline(always)]
	pub fn is_checked(&self) -> bool { self.value() == 1 }
}


impl<UD, Api: api::Api, const REM: bool> MenuItem<kind::Options, UD, Api, REM> {
	/// The array index of the currently selected option.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::playdate_sys::getMenuItemValue")]
	#[inline(always)]
	pub fn selected_option(&self) -> i32 { self.value() }
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


/// Removes all custom menu items from the system menu.
///
/// Equivalent to [`sys::ffi::playdate_sys::removeAllMenuItems`]
#[doc(alias = "sys::ffi::playdate_sys::removeAllMenuItems")]
#[inline(always)]
pub fn remove_all_menu_items() { remove_all_menu_items_with(api::Default::default()) }

/// Removes all custom menu items from the system menu.
///
/// Uses given `api`.
///
/// Equivalent to [`sys::ffi::playdate_sys::removeAllMenuItems`]
#[doc(alias = "sys::ffi::playdate_sys::removeAllMenuItems")]
pub fn remove_all_menu_items_with<Api: api::Api>(api: Api) {
	let f = api.remove_all_menu_items();
	unsafe { f() };
}

/// A game can optionally provide an image to be displayed alongside the system menu.
/// bitmap must be a 400x240 LCDBitmap. All important content should be in the
/// left half of the image in an area 200 pixels wide, as the menu will obscure the rest.
/// The right side of the image will be visible briefly as the menu animates in and out.
///
/// Optionally, a non-zero xoffset, can be provided. This must be a number between 0 and 200
/// and will cause the menu image to animate to a position offset left by xoffset pixels
/// as the menu is animated in.
///
/// This function could be called in response to the kEventPause event in your implementation
/// of `event_handler()`.
///
/// Equivalent to [`sys::ffi::playdate_sys::setMenuImage`]
#[doc(alias = "sys::ffi::playdate_sys::setMenuImage")]
#[inline(always)]
pub fn set_menu_image(bitmap: impl AnyBitmap, x_offset: c_int) {
	set_menu_image_with(api::Default::default(), bitmap, x_offset);
}


/// A game can optionally provide an image to be displayed alongside the system menu.
/// bitmap must be a 400x240 LCDBitmap. All important content should be in the
/// left half of the image in an area 200 pixels wide, as the menu will obscure the rest.
/// The right side of the image will be visible briefly as the menu animates in and out.
///
/// Optionally, a non-zero xoffset, can be provided. This must be a number between 0 and 200
/// and will cause the menu image to animate to a position offset left by xoffset pixels
/// as the menu is animated in.
///
/// This function could be called in response to the kEventPause event in your implementation
/// of `event_handler()`.
///
/// Use given `api`.
///
/// Equivalent to [`sys::ffi::playdate_sys::setMenuImage`]
#[doc(alias = "sys::ffi::playdate_sys::setMenuImage")]
pub fn set_menu_image_with<Api: api::Api>(api: Api, bitmap: impl AnyBitmap, x_offset: c_int) {
	let f = api.set_menu_image();
	unsafe { f(bitmap.as_raw(), x_offset) };
}

pub trait SystemMenu<Api: api::Api + Copy> {
	/// Removes all custom menu items from the system menu.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::removeAllMenuItems`]
	#[doc(alias = "sys::ffi::playdate_sys::removeAllMenuItems")]
	fn remove_all_menu_items(&self);

	/// A game can optionally provide an image to be displayed alongside the system menu.
	/// bitmap must be a 400x240 LCDBitmap. All important content should be in the
	/// left half of the image in an area 200 pixels wide, as the menu will obscure the rest.
	/// The right side of the image will be visible briefly as the menu animates in and out.
	///
	/// Optionally, a non-zero xoffset, can be provided. This must be a number between 0 and 200
	/// and will cause the menu image to animate to a position offset left by xoffset pixels
	/// as the menu is animated in.
	///
	/// This function could be called in response to the kEventPause event in your implementation
	/// of `event_handler()`.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setMenuImage`]
	#[doc(alias = "sys::ffi::playdate_sys::setMenuImage")]
	fn set_menu_image(&self, bitmap: impl AnyBitmap, x_offset: c_int);
}

impl<Api: system::api::Api + api::Api + Copy> SystemMenu<Api> for system::System<Api> {
	#[inline(always)]
	fn remove_all_menu_items(&self) {
		remove_all_menu_items_with(self.inner())
	}

	#[inline(always)]
	fn set_menu_image(&self, bitmap: impl AnyBitmap, x_offset: c_int) {
		set_menu_image_with(self.inner(), bitmap, x_offset)
	}
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
	use core::ptr::NonNull;
	use sys::ffi::LCDBitmap;
	use sys::ffi::PDMenuItem;
	use sys::ffi::PDMenuItemCallbackFunction;
	use sys::ffi::playdate_sys;


	/// Default system menu api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached system menu api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_sys);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(api!(system)) }
	}

	impl From<*const playdate_sys> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_sys) -> Self { Self(unsafe { ptr.as_ref() }.expect("system")) }
	}

	impl From<&'static playdate_sys> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_sys) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_sys>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_sys>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_sys>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_sys>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<system::api::Cache> for Cache {
		#[inline(always)]
		fn from(api: system::api::Cache) -> Self { Self(api.as_inner()) }
	}

	impl Api for system::api::Default {}

	impl Api for system::api::Cache {
		#[inline(always)]
		fn add_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.as_inner().addMenuItem.expect("addMenuItem")
		}

		#[inline(always)]
		fn add_checkmark_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        value: c_int,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.as_inner()
			    .addCheckmarkMenuItem
			    .expect("addCheckmarkMenuItem")
		}

		#[inline(always)]
		fn add_options_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        optionTitles: *mut *const c_char,
			                        optionsCount: c_int,
			                        f: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.as_inner().addOptionsMenuItem.expect("addOptionsMenuItem")
		}

		#[inline(always)]
		fn remove_menu_item(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) {
			self.as_inner().removeMenuItem.expect("removeMenuItem")
		}

		#[inline(always)]
		fn get_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> c_int {
			self.as_inner().getMenuItemValue.expect("getMenuItemValue")
		}

		#[inline(always)]
		fn set_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: c_int) {
			self.as_inner().setMenuItemValue.expect("setMenuItemValue")
		}

		#[inline(always)]
		fn get_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char {
			self.as_inner().getMenuItemTitle.expect("getMenuItemTitle")
		}

		#[inline(always)]
		fn set_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char) {
			self.as_inner().setMenuItemTitle.expect("setMenuItemTitle")
		}

		#[inline(always)]
		fn get_menu_item_userdata(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void {
			self.as_inner().getMenuItemUserdata.expect("getMenuItemUserdata")
		}

		#[inline(always)]
		fn remove_all_menu_items(&self) -> unsafe extern "C" fn() {
			self.as_inner().removeAllMenuItems.expect("removeAllMenuItems")
		}

		#[inline(always)]
		fn set_menu_image(&self) -> unsafe extern "C" fn(*mut LCDBitmap, i32) {
			self.as_inner().setMenuImage.expect("setMenuImage")
		}
	}


	impl Api for Cache {
		#[inline(always)]
		fn add_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.0.addMenuItem.expect("addMenuItem")
		}

		#[inline(always)]
		fn add_checkmark_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        value: c_int,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.0.addCheckmarkMenuItem.expect("addCheckmarkMenuItem")
		}

		#[inline(always)]
		fn add_options_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        optionTitles: *mut *const c_char,
			                        optionsCount: c_int,
			                        f: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			self.0.addOptionsMenuItem.expect("addOptionsMenuItem")
		}

		#[inline(always)]
		fn remove_menu_item(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) {
			self.0.removeMenuItem.expect("removeMenuItem")
		}

		#[inline(always)]
		fn get_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> c_int {
			self.0.getMenuItemValue.expect("getMenuItemValue")
		}

		#[inline(always)]
		fn set_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: c_int) {
			self.0.setMenuItemValue.expect("setMenuItemValue")
		}

		#[inline(always)]
		fn get_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char {
			self.0.getMenuItemTitle.expect("getMenuItemTitle")
		}

		#[inline(always)]
		fn set_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char) {
			self.0.setMenuItemTitle.expect("setMenuItemTitle")
		}

		#[inline(always)]
		fn get_menu_item_userdata(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void {
			self.0.getMenuItemUserdata.expect("getMenuItemUserdata")
		}

		#[inline(always)]
		fn remove_all_menu_items(&self) -> unsafe extern "C" fn() {
			self.0.removeAllMenuItems.expect("removeAllMenuItems")
		}
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_sys::addMenuItem`]
		#[doc(alias = "sys::ffi::playdate_sys::addMenuItem")]
		fn add_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addMenuItem)
		}

		/// Returns [`sys::ffi::playdate_sys::addCheckmarkMenuItem`]
		#[doc(alias = "sys::ffi::playdate_sys::addCheckmarkMenuItem")]
		fn add_checkmark_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        value: c_int,
			                        callback: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addCheckmarkMenuItem)
		}

		/// Returns [`sys::ffi::playdate_sys::addOptionsMenuItem`]
		#[doc(alias = "sys::ffi::playdate_sys::addOptionsMenuItem")]
		fn add_options_menu_item(
			&self)
			-> unsafe extern "C" fn(title: *const c_char,
			                        optionTitles: *mut *const c_char,
			                        optionsCount: c_int,
			                        f: PDMenuItemCallbackFunction,
			                        userdata: *mut c_void) -> *mut PDMenuItem {
			*sys::api!(system.addOptionsMenuItem)
		}

		/// Returns [`sys::ffi::playdate_sys::removeMenuItem`]
		#[doc(alias = "sys::ffi::playdate_sys::removeMenuItem")]
		fn remove_menu_item(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) {
			*sys::api!(system.removeMenuItem)
		}

		/// Returns [`sys::ffi::playdate_sys::getMenuItemValue`]
		#[doc(alias = "sys::ffi::playdate_sys::getMenuItemValue")]
		fn get_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> c_int {
			*sys::api!(system.getMenuItemValue)
		}

		/// Returns [`sys::ffi::playdate_sys::setMenuItemValue`]
		#[doc(alias = "sys::ffi::playdate_sys::setMenuItemValue")]
		fn set_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: c_int) {
			*sys::api!(system.setMenuItemValue)
		}

		/// Returns [`sys::ffi::playdate_sys::getMenuItemTitle`]
		#[doc(alias = "sys::ffi::playdate_sys::getMenuItemTitle")]
		fn get_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char {
			*sys::api!(system.getMenuItemTitle)
		}

		/// Returns [`sys::ffi::playdate_sys::setMenuItemTitle`]
		#[doc(alias = "sys::ffi::playdate_sys::setMenuItemTitle")]
		fn set_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char) {
			*sys::api!(system.setMenuItemTitle)
		}

		/// Returns [`sys::ffi::playdate_sys::getMenuItemUserdata`]
		#[doc(alias = "sys::ffi::playdate_sys::getMenuItemUserdata")]
		fn get_menu_item_userdata(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void {
			*sys::api!(system.getMenuItemUserdata)
		}

		/// Returns [`sys::ffi::playdate_sys::removeAllMenuItems`]
		#[doc(alias = "sys::ffi::playdate_sys::removeAllMenuItems")]
		fn remove_all_menu_items(&self) -> unsafe extern "C" fn() { *sys::api!(system.removeAllMenuItems) }

		/// Returns [`sys::ffi::playdate_sys::setMenuImage`]
		#[doc(alias = "sys::ffi::playdate_sys::setMenuImage")]
		fn set_menu_image(&self) -> unsafe extern "C" fn(*mut LCDBitmap, i32) { *sys::api!(system.setMenuImage)}
	}
}
