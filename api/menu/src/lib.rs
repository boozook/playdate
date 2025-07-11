#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl, const_deref)]
#![feature(allocator_api)]

#[macro_use]
extern crate sys;
extern crate alloc;

use core::ffi::c_int;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::NonNull;
use sys::ffi::MenuItem as PDMenuItem;
use sys::ffi::CStr;
use gfx::bitmap::AsBitmap;

use error::Error;


pub mod error;


type Api = &'static sys::ffi::PlaydateSys;


mod ud;

mod cb {
	mod ctx;
	mod no_ctx;
}


pub type SimpleMenuItem<UserData = ()> = MenuItem<kind::Simple, UserData>;
pub type CheckMenuItem<UserData = ()> = MenuItem<kind::Check, UserData>;
pub type OptionsMenuItem<UserData = ()> = MenuItem<kind::Options, UserData>;
pub type AnyMenuItem<UserData> = MenuItem<kind::Any, UserData>;

pub type SimpleMenuItemRef = MenuItemRef<kind::Simple>;
pub type CheckMenuItemRef = MenuItemRef<kind::Check>;
pub type OptionsMenuItemRef = MenuItemRef<kind::Options>;


#[repr(transparent)]
struct RemoveOnDrop(NonNull<PDMenuItem>);

impl Drop for RemoveOnDrop {
	fn drop(&mut self) {
		println!("dropping menu item");
		if let Some(api) = api_opt!(system) {
			unsafe { (api.removeMenuItem)(self.0.as_ptr()) };
		}
	}
}


#[derive(Debug)]
#[repr(transparent)]
/// Shared reference menu item.
pub struct MenuItemRef<K>(NonNull<PDMenuItem>, PhantomData<K>);

impl<K> MenuItemRef<K> {
	const fn as_raw_ptr(&self) -> *mut PDMenuItem { self.0.as_ptr() }
}


/// Owned menu item.
#[must_use]
pub struct MenuItem<Kind, Ud: ?Sized>(RemoveOnDrop, PhantomData<Kind>, Ud);

impl<K, Ud> MenuItem<K, Ud> {
	const fn as_ptr(&mut self) -> NonNull<PDMenuItem> { self.0.0 }
	const fn as_raw_ptr(&self) -> *mut PDMenuItem { self.0.0.as_ptr() }

	fn from_with(item: *mut PDMenuItem, userdata: Ud) -> Result<Self, Error> {
		NonNull::new(item).map(RemoveOnDrop)
		                  .map(move |ptr| Self(ptr, PhantomData, userdata))
		                  .ok_or(Error)
	}

	/// Kind-coercion, change kind to `any`.
	#[inline]
	pub fn into_any(self) -> AnyMenuItem<Ud> {
		let Self(ptr, _, ud) = self;
		MenuItem(ptr, PhantomData, ud)
	}
}


impl<Ud, K: kind::Kind> MenuItem<K, Ud> {
	#[inline]
	pub fn remove(self, api: Api) -> Ud {
		let Self(RemoveOnDrop(ptr, ..), _, ud) = self;
		let f = api.removeMenuItem;
		unsafe { f(ptr.as_ptr()) };
		ud
	}
}


impl<Ud, K> MenuItem<K, Ud> {
	pub const fn userdata(&self) -> &Ud { &self.2 }
	pub const fn userdata_mut(&mut self) -> &mut Ud { &mut self.2 }
}


#[duplicate::duplicate_item(
	Params MenuItem;
	[Ud, K: kind::Kind] [MenuItem<K, Ud>];
	[K: kind::Kind] [MenuItemRef<K>];
)]
impl<Params> MenuItem {
	/// Gets the display title of the menu item.
	///
	/// Returns [`sys::ffi::PlaydateSys::getMenuItemTitle`]
	#[doc(alias = "sys::ffi::PlaydateSys::getMenuItemTitle")]
	#[inline]
	pub fn title(&self, api: Api) -> &CStr {
		let f = api.getMenuItemTitle;
		unsafe { CStr::from_ptr(f(self.as_raw_ptr())) }
	}

	/// Sets the display title of the menu item.
	///
	/// Returns [`sys::ffi::PlaydateSys::setMenuItemTitle`]
	#[doc(alias = "sys::ffi::PlaydateSys::setMenuItemTitle")]
	#[inline]
	pub fn set_title(&self, api: Api, title: impl AsRef<CStr>) {
		let s = title.as_ref();
		let f = api.setMenuItemTitle;
		unsafe { f(self.as_raw_ptr(), s.as_ptr()) };
	}


	/// Gets the integer value of the menu item.
	///
	/// See also [`MenuItem::set_value`].
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::PlaydateSys::getMenuItemValue")]
	#[inline]
	pub fn value(&self, api: Api) -> c_int { unsafe { (api.getMenuItemValue)(self.as_raw_ptr()) } }

	/// Sets the integer value of the menu item.
	///
	/// For checkmark menu items ([`CheckMenuItem`]), `1` means checked, `0` unchecked.
	///
	/// For option menu items ([`OptionsMenuItem`]), the value indicates the array index of the currently selected option.
	///
	/// See also [`CheckMenuItem::is_checked`], [`OptionsMenuItem::selected_option`].
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::setMenuItemValue`]
	#[doc(alias = "sys::ffi::PlaydateSys::setMenuItemValue")]
	#[inline]
	pub fn set_value(&mut self, api: Api, value: c_int) {
		unsafe { (api.setMenuItemValue)(self.as_raw_ptr(), value) }
	}
}


#[duplicate::duplicate_item(
	Params MenuItem;
	[Ud] [MenuItem<kind::Check, Ud>];
	[  ] [MenuItemRef<kind::Check>];
)]
impl<Params> MenuItem {
	/// Equivalent to [`sys::ffi::PlaydateSys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::PlaydateSys::getMenuItemValue")]
	#[inline(always)]
	pub fn is_checked(&self, api: Api) -> bool { self.value(api) == 1 }

	/// Equivalent to [`sys::ffi::PlaydateSys::setMenuItemValue`]
	#[doc(alias = "sys::ffi::PlaydateSys::setMenuItemValue")]
	#[inline(always)]
	pub fn set_checked(&mut self, api: Api, value: bool) { self.set_value(api, value as _) }
}


#[duplicate::duplicate_item(
	Params MenuItem;
	[Ud] [MenuItem<kind::Options, Ud>];
	[  ] [MenuItemRef<kind::Options>];
)]
impl<Params> MenuItem {
	/// The array index of the currently selected option.
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::getMenuItemValue`]
	#[doc(alias = "sys::ffi::PlaydateSys::getMenuItemValue")]
	#[inline(always)]
	pub fn selected_option(&self, api: Api) -> i32 { self.value(api) }
}


#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Menu(Api);

impl const Deref for Menu {
	type Target = Api;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for Menu {
	fn default() -> Self { Self(api!(system)) }
}

impl Menu {
	pub const fn new(api: Api) -> Self { Self(api) }


	/// Removes all custom menu items from the system menu.
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::removeAllMenuItems`]
	#[doc(alias = "sys::ffi::PlaydateSys::removeAllMenuItems")]
	#[inline]
	pub fn remove_all_menu_items(&self) {
		// it is safe to remove all menu items, user ownes the menu-wrapper with the userdata (callback).
		unsafe { (self.0.removeAllMenuItems)() };
	}


	/// A game can optionally provide an image to be displayed alongside the system menu.
	/// bitmap must be a 400x240 Bitmap. All important content should be in the
	/// left half of the image in an area 200 pixels wide, as the menu will obscure the rest.
	/// The right side of the image will be visible briefly as the menu animates in and out.
	///
	/// Optionally, a non-zero `x_offset`, can be provided. This must be a number between 0 and 200
	/// and will cause the menu image to animate to a position offset left by `x_offset` pixels
	/// as the menu is animated in.
	///
	/// This function could be called in response to the [`SystemEvent::Pause`] event in your implementation
	/// of `event-handler`.
	///
	/// [`SystemEvent::Pause`]: sys::ffi::SystemEvent::Pause
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::setMenuImage`]
	#[doc(alias = "sys::ffi::PlaydateSys::setMenuImage")]
	#[inline]
	pub fn set_menu_image(&self, bitmap: impl AsBitmap, x_offset: c_int) {
		unsafe { (self.0.setMenuImage)(bitmap.as_raw().as_ptr(), x_offset) };
	}
}


pub mod kind {
	pub trait Kind {}


	#[derive(Debug)]
	pub struct Simple;

	#[derive(Debug)]
	pub struct Check;

	#[derive(Debug)]
	pub struct Options;

	#[derive(Debug)]
	pub struct Any;
	impl Kind for Any {}

	impl Kind for Simple {}
	impl Kind for Check {}
	impl Kind for Options {}
}
