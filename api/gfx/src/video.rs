//! Playdate video API

use core::ffi::{c_char, c_int, c_float};

use sys::ffi::LCDVideoPlayer;
use sys::ffi::{CString, CStr};
use fs::Path;

use crate::Graphics;
use crate::bitmap::{AnyBitmap, BitmapRef};
use crate::error::ApiError;
use crate::error::Error;


#[derive(Debug, Clone, Copy)]
pub struct Video<Api: api::Api = api::Default>(Api);

impl Video<api::Default> {
	/// Creates default [`Video`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Video<api::Cache> {
	/// Creates [`Video`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Video<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Video<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Video<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}

impl<Api: api::Api + Copy> Video<Api> {
	/// Opens the `pdv` file at path and returns a new [`VideoPlayer`] for rendering its frames.
	///
	/// Calls [`sys::ffi::playdate_video::loadVideo`].
	#[doc(alias = "sys::ffi::playdate_video::loadVideo")]
	pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<VideoPlayer<Api>, ApiError> {
		VideoPlayer::load_with(self.0, path)
	}
}


impl<Api: crate::api::Api> Graphics<Api> {
	/// Creates a new [`Video`] instance with [cached api](api::Cache).
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::video`]
	#[doc(alias = "sys::ffi::playdate_graphics::video")]
	pub fn video(&self) -> Video<api::Cache> { Video::new_with(self.0.video::<api::Cache>()) }

	/// Creates a new [`Video`] instance using given `api`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::video`]
	#[doc(alias = "sys::ffi::playdate_graphics::video")]
	pub fn video_with<VideoApi: api::Api>(&self, api: VideoApi) -> Video<VideoApi> { Video::new_with(api) }
}


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct VideoPlayer<Api: api::Api = api::Default, const FREE_ON_DROP: bool = true>(*mut LCDVideoPlayer, Api);

impl<Api: api::Api, const FOD: bool> Drop for VideoPlayer<Api, FOD> {
	fn drop(&mut self) {
		if FOD && !self.0.is_null() {
			let f = self.1.free_player();
			unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();
		}
	}
}


impl<Api: api::Api + Copy> VideoPlayer<Api, true> {
	/// Convert this video player into the same but that will not be freed on drop.
	/// That means that only C-part of the player will __not__ be freed.
	///
	/// __Safety is guaranteed by the caller.__
	pub fn into_shared(mut self) -> VideoPlayer<Api, false> {
		let res = VideoPlayer(self.0, self.1);
		self.0 = core::ptr::null_mut();
		res
	}
}


impl<Api: api::Api> VideoPlayer<Api, true> {
	/// Opens the `pdv` file at path and returns a new video player object for rendering its frames.
	///
	/// Calls [`sys::ffi::playdate_video::loadVideo`].
	#[doc(alias = "sys::ffi::playdate_video::loadVideo")]
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ApiError>
		where Api: Default {
		let api = Api::default();
		Self::load_with(api, path)
	}

	/// Opens the `pdv` file at path and returns a new video player object for rendering its frames.
	///
	/// Calls [`sys::ffi::playdate_video::loadVideo`].
	#[doc(alias = "sys::ffi::playdate_video::loadVideo")]
	pub fn load_with<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, ApiError> {
		let path = CString::new(path.as_ref())?;

		let f = api.load_video();
		let ptr = unsafe { f(path.as_ptr() as *mut c_char) };
		if ptr.is_null() {
			// Maybe we able to `get_error` for null pointer?
			Err(Error::Alloc.into())
		} else {
			Ok(Self(ptr, api))
		}
	}
}


impl<Api: api::Api, const FOD: bool> VideoPlayer<Api, FOD> {
	/// Sets the rendering destination for the video player to the given bitmap.
	///
	/// If the function fails, it returns [`Error::Video`] if err-message supplied by C-API,
	/// or [`Error::Unknown`] in other cases.
	///
	/// Calls [`sys::ffi::playdate_video::setContext`].
	#[doc(alias = "sys::ffi::playdate_video::setContext")]
	pub fn set_context<'a, 'b: 'a>(&'a self, bitmap: &'b impl AnyBitmap) -> Result<(), Error> {
		let f = self.1.set_context();
		if unsafe { f(self.0, bitmap.as_raw()) } != 0 {
			Ok(())
		} else {
			Err(self.get_error().unwrap_or(Error::Unknown))
		}
	}

	/// Gets the rendering destination for the video player.
	///
	/// If no rendering context has been set allocates a context bitmap with the same dimensions as the video will be allocated.
	///
	/// Calls [`sys::ffi::playdate_video::getContext`].
	#[doc(alias = "sys::ffi::playdate_video::getContext")]
	pub fn get_context(&self) -> Result<BitmapRef<'_>, Error> {
		let f = self.1.get_context();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(BitmapRef::from(ptr))
		}
	}


	/// Calls [`sys::ffi::playdate_video::useScreenContext`].
	#[doc(alias = "sys::ffi::playdate_video::useScreenContext")]
	pub fn use_screen_context(&self) {
		let f = self.1.use_screen_context();
		unsafe { f(self.0) }
	}

	/// Renders frame number `n` into the current context.
	///
	/// In case of error, it returns [`Error::Video`] if err-message supplied by C-API,
	/// or [`Error::Unknown`] in other cases.
	///
	/// Calls [`sys::ffi::playdate_video::renderFrame`].
	#[doc(alias = "sys::ffi::playdate_video::renderFrame")]
	pub fn render_frame(&self, n: c_int) -> Result<(), Error> {
		let f = self.1.render_frame();
		if unsafe { f(self.0, n) } != 0 {
			Ok(())
		} else {
			Err(self.get_error().unwrap_or(Error::Unknown))
		}
	}

	/// Retrieves information about the video.
	///
	/// Calls [`sys::ffi::playdate_video::renderFrame`].
	#[doc(alias = "sys::ffi::playdate_video::renderFrame")]
	pub fn info(&self) -> VideoPlayerOutInfo {
		let mut info = VideoPlayerOutInfo::default();
		self.info_to(&mut info);
		info
	}

	/// Retrieves information about the video, by passing values into given `info`.
	///
	/// Calls [`sys::ffi::playdate_video::renderFrame`].
	#[doc(alias = "sys::ffi::playdate_video::renderFrame")]
	pub fn info_to(&self, info: &mut VideoPlayerOutInfo) {
		let f = self.1.get_info();
		unsafe {
			f(
			  self.0,
			  &mut info.width,
			  &mut info.height,
			  &mut info.frame_rate,
			  &mut info.frame_count,
			  &mut info.current_frame,
			)
		};
	}

	/// Retrieves information about the video, by passing optional mutable references.
	///
	/// Calls [`sys::ffi::playdate_video::renderFrame`].
	#[doc(alias = "sys::ffi::playdate_video::renderFrame")]
	pub fn info_raw(&self,
	                width: Option<&mut c_int>,
	                height: Option<&mut c_int>,
	                frame_rate: Option<&mut c_float>,
	                frame_count: Option<&mut c_int>,
	                current_frame: Option<&mut c_int>) {
		let f = self.1.get_info();
		unsafe {
			use core::ptr::null_mut;
			f(
			  self.0,
			  width.map_or(null_mut() as _, |v| v as *mut _),
			  height.map_or(null_mut() as _, |v| v as *mut _),
			  frame_rate.map_or(null_mut() as _, |v| v as *mut _),
			  frame_count.map_or(null_mut() as _, |v| v as *mut _),
			  current_frame.map_or(null_mut() as _, |v| v as *mut _),
			)
		};
	}


	/// Returns [`Error`] with text describing the most recent error.
	///
	/// Inner text is borrowed by C, so it should be used immediately or converted to something owned.
	///
	/// See also [`VideoPlayer::get_error_cstr`].
	///
	/// Calls [`sys::ffi::playdate_video::getError`].
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	#[inline(always)]
	pub fn get_error(&self) -> Option<Error> { self.get_error_cstr().map(Error::video_from) }

	/// Returns [`CStr`] describing the most recent error.
	///
	/// String-slice is borrowed by C, so it should be used immediately or converted to something owned.
	///
	/// Calls [`sys::ffi::playdate_video::getError`].
	#[doc(alias = "sys::ffi::playdate_video::getError")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn get_error_cstr(&self) -> Option<&CStr> {
		let f = self.1.get_error();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.into()
		}
	}
}


#[derive(Debug, Clone, Default)]
pub struct VideoPlayerOutInfo {
	pub width: c_int,
	pub height: c_int,
	pub frame_rate: c_float,
	pub frame_count: c_int,
	pub current_frame: c_int,
}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_float;
	use core::ffi::c_int;
	use core::ptr::NonNull;

	use sys::ffi::LCDBitmap;
	use sys::ffi::LCDVideoPlayer;
	use sys::ffi::playdate_video;


	/// Default video api end-point, ZST.
	///
	/// All calls approximately costs ~4 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached video api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Debug, Clone, Copy)]
	pub struct Cache(&'static playdate_video);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(graphics.video)) }
	}

	impl From<*const playdate_video> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_video) -> Self { Self(unsafe { ptr.as_ref() }.expect("video")) }
	}

	impl From<&'static playdate_video> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_video) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_video>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_video>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_video>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_video>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		#[inline(always)]
		fn load_video(&self) -> unsafe extern "C" fn(path: *const c_char) -> *mut LCDVideoPlayer {
			self.0.loadVideo.expect("loadVideo")
		}

		#[inline(always)]
		fn free_player(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) {
			self.0.freePlayer.expect("freePlayer")
		}

		#[inline(always)]
		fn set_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer, context: *mut LCDBitmap) -> c_int {
			self.0.setContext.expect("setContext")
		}

		#[inline(always)]
		fn use_screen_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) {
			self.0.useScreenContext.expect("useScreenContext")
		}

		#[inline(always)]
		fn render_frame(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer, n: c_int) -> c_int {
			self.0.renderFrame.expect("renderFrame")
		}

		#[inline(always)]
		fn get_error(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *const c_char {
			self.0.getError.expect("getError")
		}

		#[inline(always)]
		fn get_info(
			&self)
			-> unsafe extern "C" fn(p: *mut LCDVideoPlayer,
			                        outWidth: *mut c_int,
			                        outHeight: *mut c_int,
			                        outFrameRate: *mut c_float,
			                        outFrameCount: *mut c_int,
			                        outCurrentFrame: *mut c_int) {
			*sys::api!(graphics.video.getInfo)
		}

		#[inline(always)]
		fn get_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *mut LCDBitmap {
			self.0.getContext.expect("getContext")
		}
	}


	pub trait Api {
		/// Equivalent to [`sys::ffi::playdate_video::loadVideo`]
		#[doc(alias = "sys::ffi::playdate_video::loadVideo")]
		#[inline(always)]
		fn load_video(&self) -> unsafe extern "C" fn(path: *const c_char) -> *mut LCDVideoPlayer {
			*sys::api!(graphics.video.loadVideo)
		}

		/// Equivalent to [`sys::ffi::playdate_video::freePlayer`]
		#[doc(alias = "sys::ffi::playdate_video::freePlayer")]
		#[inline(always)]
		fn free_player(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) {
			*sys::api!(graphics.video.freePlayer)
		}

		/// Equivalent to [`sys::ffi::playdate_video::setContext`]
		#[doc(alias = "sys::ffi::playdate_video::setContext")]
		#[inline(always)]
		fn set_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer, context: *mut LCDBitmap) -> c_int {
			*sys::api!(graphics.video.setContext)
		}

		/// Equivalent to [`sys::ffi::playdate_video::useScreenContext`]
		#[doc(alias = "sys::ffi::playdate_video::useScreenContext")]
		#[inline(always)]
		fn use_screen_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) {
			*sys::api!(graphics.video.useScreenContext)
		}

		/// Equivalent to [`sys::ffi::playdate_video::renderFrame`]
		#[doc(alias = "sys::ffi::playdate_video::renderFrame")]
		#[inline(always)]
		fn render_frame(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer, n: c_int) -> c_int {
			*sys::api!(graphics.video.renderFrame)
		}

		/// Equivalent to [`sys::ffi::playdate_video::getError`]
		#[doc(alias = "sys::ffi::playdate_video::getError")]
		#[inline(always)]
		fn get_error(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *const c_char {
			*sys::api!(graphics.video.getError)
		}

		/// Equivalent to [`sys::ffi::playdate_video::getInfo`]
		#[doc(alias = "sys::ffi::playdate_video::getInfo")]
		#[inline(always)]
		fn get_info(
			&self)
			-> unsafe extern "C" fn(p: *mut LCDVideoPlayer,
			                        outWidth: *mut c_int,
			                        outHeight: *mut c_int,
			                        outFrameRate: *mut c_float,
			                        outFrameCount: *mut c_int,
			                        outCurrentFrame: *mut c_int) {
			*sys::api!(graphics.video.getInfo)
		}

		/// Equivalent to [`sys::ffi::playdate_video::getContext`]
		#[doc(alias = "sys::ffi::playdate_video::getContext")]
		#[inline(always)]
		fn get_context(&self) -> unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *mut LCDBitmap {
			*sys::api!(graphics.video.getContext)
		}
	}
}
