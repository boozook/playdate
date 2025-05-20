//! Playdate video-stream API

use core::ffi::c_int;
use core::ops::Deref;
use core::ptr::NonNull;

use sys::macros::api_opt;
use sys::ffi::StreamPlayer as SysStreamPlayer;

use crate::Graphics;
use super::error;


impl Graphics {
	pub const fn video_stream(&self) -> VideoStream { VideoStream(self.0.videostream) }
}


type Api = &'static sys::ffi::PlaydateVideoStream;


#[derive(Clone, Copy)]
pub struct VideoStream(Api);

impl Deref for VideoStream {
	type Target = Api;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for VideoStream {
	fn default() -> Self { Self(sys::macros::api!(graphics.videostream)) }
}


pub struct StreamPlayer(NonNull<SysStreamPlayer>);

impl Drop for StreamPlayer {
	fn drop(&mut self) {
		if let Some(f) = api_opt!(graphics.videostream.freePlayer) {
			unsafe { f(self.0.as_ptr()) };
		}
	}
}


impl StreamPlayer {
	/// Equivalent to [`sys::ffi::PlaydateVideoStream::newPlayer`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::newPlayer")]
	pub fn new(api: Api) -> Result<Self, error::Alloc> {
		let ptr = unsafe { (api.newPlayer)() };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}

	/// Equivalent to [`sys::ffi::PlaydateVideoStream::setBufferSize`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::setBufferSize")]
	pub fn set_buffer_size(&mut self, api: Api, video: c_int, audio: c_int) {
		unsafe { (api.setBufferSize)(self.0.as_ptr(), video, audio) };
	}


	/// Equivalent to [`sys::ffi::PlaydateVideoStream::setFile`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::setFile")]
	pub fn set_file(&mut self, api: Api, file: &mut fs::file::File) {
		unsafe { (api.setFile)(self.0.as_ptr(), file.as_raw()) };
	}

	/* TODO: impl methods
		setHTTPConnection
		setTCPConnection
		getFilePlayer
	*/


	/// Equivalent to [`sys::ffi::PlaydateVideoStream::getVideoPlayer`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::getVideoPlayer")]
	pub fn video_player(&mut self, api: Api) -> Option<super::VideoPlayer> {
		let ptr = unsafe { (api.getVideoPlayer)(self.0.as_ptr()) };
		NonNull::new(ptr).map(super::VideoPlayer)
	}

	/// Equivalent to [`sys::ffi::PlaydateVideoStream::update`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::update")]
	pub fn update(&mut self, api: Api) -> bool { unsafe { (api.update)(self.0.as_ptr()) } }


	/// Equivalent to [`sys::ffi::PlaydateVideoStream::getBufferedFrameCount`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::getBufferedFrameCount")]
	pub fn buffered_frame_count(&mut self, api: Api) -> c_int {
		unsafe { (api.getBufferedFrameCount)(self.0.as_ptr()) }
	}

	/// Equivalent to [`sys::ffi::PlaydateVideoStream::getBytesRead`].
	#[doc(alias = "sys::ffi::PlaydateVideoStream::getBytesRead")]
	pub fn bytes_read(&mut self, api: Api) -> u32 { unsafe { (api.getBytesRead)(self.0.as_ptr()) } }
}
