//! Playdate video API

use core::ffi::{c_int, c_float};
use core::ops::Deref;
use core::ptr::NonNull;

use sys::macros::api_opt;
use sys::ffi::VideoPlayer as SysVideoPlayer;
use sys::ffi::CStr;
use fs::path::Path;

use crate::Graphics;
use crate::bitmap::{AsBitmap, Borrowed};


pub mod stream;


impl Graphics {
	pub const fn video(&self) -> Video { Video(self.0.video) }
}


type Api = &'static sys::ffi::PlaydateVideo;


#[derive(Clone, Copy)]
pub struct Video(Api);

impl Deref for Video {
	type Target = Api;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for Video {
	fn default() -> Self { Self(sys::macros::api!(graphics.video)) }
}


pub struct VideoPlayer(NonNull<SysVideoPlayer>);

impl Drop for VideoPlayer {
	fn drop(&mut self) {
		if let Some(f) = api_opt!(graphics.video.freePlayer) {
			unsafe { f(self.0.as_ptr()) };
		}
	}
}


impl VideoPlayer {
	/// Opens the `pdv` file at path and returns a new video player object for rendering its frames.
	///
	/// Calls [`sys::ffi::PlaydateVideo::loadVideo`].
	#[doc(alias = "sys::ffi::PlaydateVideo::loadVideo")]
	pub fn load<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, error::LoadError> {
		let path = path.as_ref();
		let ptr = unsafe { (api.loadVideo)(path.as_ptr()) };
		if ptr.is_null() {
			if let Some(err) = fs::error::latest(::sys::api!(file)) {
				Err(error::LoadError::Fs(fs::error::Owned::from(err)))
			} else {
				Err(error::LoadError::Alloc(error::Alloc))
			}
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Sets the rendering destination for the video player to the given bitmap.
	///
	/// If the function fails, it returns [`Error::Video`] if err-message supplied by C-API,
	/// or [`Error::Unknown`] in other cases.
	///
	/// Calls [`sys::ffi::PlaydateVideo::setContext`].
	#[doc(alias = "sys::ffi::PlaydateVideo::setContext")]
	pub fn set_context<'a, 'b: 'a>(&'a self,
	                               api: Api,
	                               bitmap: &'b impl AsBitmap)
	                               -> Result<(), error::Borrowed<'a>> {
		if unsafe { (api.setContext)(self.0.as_ptr(), bitmap.as_raw().as_ptr()) } != 0 {
			Ok(())
		} else {
			Err(
			    self.get_error(api)
			        .map_or(error::VideoError::Unknown, error::Borrowed::from),
			)
		}
	}

	/// Gets the rendering destination for the video player.
	///
	/// If no rendering context has been set allocates a context bitmap with the same dimensions as the video will be allocated.
	///
	/// Calls [`sys::ffi::PlaydateVideo::getContext`].
	#[doc(alias = "sys::ffi::PlaydateVideo::getContext")]
	pub fn get_context(&self, api: Api) -> Result<Borrowed<'_>, error::Alloc> {
		let ptr = unsafe { (api.getContext)(self.0.as_ptr()) };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(Borrowed::from_ptr(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Calls [`sys::ffi::PlaydateVideo::useScreenContext`].
	#[doc(alias = "sys::ffi::PlaydateVideo::useScreenContext")]
	pub fn use_screen_context(&self, api: Api) { unsafe { (api.useScreenContext)(self.0.as_ptr()) } }

	/// Renders frame number `n` into the current context.
	///
	/// In case of error, it returns [`Error::Video`] if err-message supplied by C-API,
	/// or [`Error::Unknown`] in other cases.
	///
	/// Calls [`sys::ffi::PlaydateVideo::renderFrame`].
	#[doc(alias = "sys::ffi::PlaydateVideo::renderFrame")]
	pub fn render_frame(&mut self, api: Api, n: c_int) -> Result<(), error::Borrowed> {
		if unsafe { (api.renderFrame)(self.0.as_ptr(), n) } != 0 {
			Ok(())
		} else {
			Err(
			    self.get_error(api)
			        .map_or(error::VideoError::Unknown, error::Borrowed::from),
			)
		}
	}

	/// Retrieves information about the video state.
	///
	/// Calls [`sys::ffi::PlaydateVideo::renderFrame`].
	#[doc(alias = "sys::ffi::PlaydateVideo::renderFrame")]
	pub fn info(&self, api: Api) -> VideoPlayerInfo {
		let mut info = VideoPlayerInfo::default();
		self.info_to(api, &mut info);
		info
	}

	/// Retrieves information about the video state, by passing values into given `info`.
	///
	/// Calls [`sys::ffi::PlaydateVideo::renderFrame`].
	#[doc(alias = "sys::ffi::PlaydateVideo::renderFrame")]
	pub fn info_to(&self, api: Api, info: &mut VideoPlayerInfo) {
		unsafe {
			(api.getInfo)(
			              self.0.as_ptr(),
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
	/// Example:
	/// ```no_run
	/// let mut frame_count = Some(0);
	/// let mut current_frame = Some(0);
	/// player.info_raw(None, None, None,
	///                 frame_count.as_mut(),
	///                 current_frame.as_mut()
	///                );
	/// println!( "{}/{}", current_frame.unwrap(), frame_count.unwrap());
	/// ```
	/// Calls [`sys::ffi::PlaydateVideo::renderFrame`].
	#[doc(alias = "sys::ffi::PlaydateVideo::renderFrame")]
	pub fn info_to_opt(&self,
	                   api: Api,
	                   width: Option<&mut c_int>,
	                   height: Option<&mut c_int>,
	                   frame_rate: Option<&mut c_float>,
	                   frame_count: Option<&mut c_int>,
	                   current_frame: Option<&mut c_int>) {
		unsafe {
			use core::ptr::null_mut;
			(api.getInfo)(
			              self.0.as_ptr(),
			              width.map_or(null_mut() as _, |v| v),
			              height.map_or(null_mut() as _, |v| v),
			              frame_rate.map_or(null_mut() as _, |v| v),
			              frame_count.map_or(null_mut() as _, |v| v),
			              current_frame.map_or(null_mut() as _, |v| v),
			)
		};
	}


	/// Returns [`CStr`] describing the most recent error.
	///
	/// String-slice is borrowed by C, so it should be used immediately or converted to something owned.
	///
	/// Calls [`sys::ffi::PlaydateVideo::getError`].
	#[doc(alias = "sys::ffi::PlaydateVideo::getError")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn get_error<'t>(&self, api: Api) -> Option<&'t CStr> {
		let ptr = unsafe { (api.getError)(self.0.as_ptr()) };
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr) }.into()
		}
	}
}


#[derive(Debug, Clone, Default)]
pub struct VideoPlayerInfo {
	pub width: c_int,
	pub height: c_int,
	pub frame_rate: c_float,
	pub frame_count: c_int,
	pub current_frame: c_int,
}


pub mod error {
	use core::ffi::CStr;
	use core::fmt;

	use alloc::borrow::ToOwned as _;
	use sys::ffi::CString;

	pub use crate::error::*;


	/// Owned Video Error.
	pub type Owned = VideoError<CString>;
	/// Borrowed Video Error, with message borrowed from C part.
	pub type Borrowed<'t> = VideoError<&'t CStr>;


	#[derive(Debug)]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted into an owned."]
	pub enum VideoError<T: AsRef<CStr>> {
		Error(T),
		Unknown,
	}

	impl From<Borrowed<'_>> for Owned {
		fn from(err: Borrowed) -> Self { err.into_owned() }
	}
	impl<'t> From<&'t Owned> for Borrowed<'t> {
		fn from(err: &'t Owned) -> Self {
			match err {
				VideoError::Error(err) => VideoError::Error(err.as_c_str()),
				VideoError::Unknown => VideoError::Unknown,
			}
		}
	}


	impl Borrowed<'_> {
		pub fn into_owned(self) -> Owned {
			match self {
				Self::Error(err) => VideoError::Error(err.to_owned()),
				Self::Unknown => VideoError::Unknown,
			}
		}
	}


	impl<T: fmt::Debug + AsRef<CStr>> core::error::Error for VideoError<T> {}

	impl<T: fmt::Debug + AsRef<CStr>> fmt::Display for VideoError<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match &self {
				Self::Error(cs) => {
					match cs.as_ref().to_str() {
						Ok(err) => err.fmt(f),
						Err(_) => f.write_fmt(format_args!("{cs:?}")),
					}
				},
				Self::Unknown => write!(f, "video err"),
			}
		}
	}

	impl<'t: 'op, 'op> From<&'t CStr> for Borrowed<'op> {
		fn from(err: &'t CStr) -> Self { Self::Error(err) }
	}

	impl From<&'_ CStr> for Owned {
		fn from(err: &'_ CStr) -> Self { Self::Error(err.to_owned()) }
	}
}
