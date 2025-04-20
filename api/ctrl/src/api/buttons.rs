use core::ptr::null_mut;

use crate::Api;


mod sys {
	pub use sys::ffi::Buttons;
}


/// Buttons
#[derive(Clone, Copy)]
pub struct Buttons(pub(crate) Api);

impl Default for Buttons {
	fn default() -> Self { Self(api!(system)) }
}

impl Buttons {
	pub fn new(api: Api) -> Self { Self(api) }
}

impl Buttons {
	/// Returns the current buttons [`State`].
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline(always)]
	pub fn state(&self) -> State {
		let mut state = State::empty();
		self.state_to(&mut state);
		state
	}

	/// Writes the current buttons state to given [`State`].
	///
	/// Updates `current`, `pushed` and `released`.
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline(always)]
	pub fn state_to(&self, state: &mut State) {
		self.state_to_parts(&mut state.current, &mut state.pushed, &mut state.released)
	}

	/// Writes the current buttons state to given references.
	///
	/// See also [`state_to_some`][`Self::state_to_some`].
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline(always)]
	pub fn state_to_parts(&self,
	                      current: &mut sys::Buttons,
	                      pushed: &mut sys::Buttons,
	                      released: &mut sys::Buttons) {
		unsafe { (self.0.getButtonState)(current, pushed, released) }
	}

	/// Writes the current buttons state to given references.
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline(always)]
	pub fn state_to_some(&self,
	                     current: Option<&mut sys::Buttons>,
	                     pushed: Option<&mut sys::Buttons>,
	                     released: Option<&mut sys::Buttons>) {
		unsafe {
			(self.0.getButtonState)(
			                        current.map(|r| r as _).unwrap_or(null_mut()),
			                        pushed.map(|r| r as _).unwrap_or(null_mut()),
			                        released.map(|r| r as _).unwrap_or(null_mut()),
			)
		}
	}

	/// Requests & returns buttons currently down, over the previous update cycle.
	///
	/// Note: at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline]
	pub fn current(&self) -> sys::Buttons {
		let mut current = sys::Buttons(0);
		unsafe { (self.0.getButtonState)(&mut current, null_mut(), null_mut()) }
		current
	}

	/// Requests & returns buttons pushed over the previous update cycle.
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline]
	pub fn pushed(&self) -> sys::Buttons {
		let mut pushed = sys::Buttons(0);
		unsafe { (self.0.getButtonState)(null_mut(), &mut pushed, null_mut()) }
		pushed
	}

	/// Requests & returns buttons released over the previous update cycle.
	#[doc(alias = "sys::ffi::PlaydateSys::getButtonState")]
	#[inline]
	pub fn released(&self) -> sys::Buttons {
		let mut released = sys::Buttons(0);
		unsafe { (self.0.getButtonState)(null_mut(), null_mut(), &mut released) }
		released
	}
}

/// Represents buttons state.
///
/// * `current` indicates which buttons are currently down.
/// * `pushed` and `released` reflects which buttons were pushed or released over the previous update cycle.
#[derive(Clone, Copy)]
pub struct State {
	/// Buttons which are __currently down__.
	pub current: sys::Buttons,
	/// Buttons which were pushed over the previous update cycle.
	pub pushed: sys::Buttons,
	/// Buttons which were released over the previous update cycle.
	pub released: sys::Buttons,
}

impl State {
	pub const fn empty() -> Self {
		Self { current: sys::Buttons(0),
		       pushed: sys::Buttons(0),
		       released: sys::Buttons(0) }
	}

	pub const fn is_empty(&self) -> bool {
		self.current_is_empty() && self.pushed_is_empty() && self.released_is_empty()
	}

	pub const fn current_is_empty(&self) -> bool { self.current.0 == 0 }
	pub const fn pushed_is_empty(&self) -> bool { self.pushed.0 == 0 }
	pub const fn released_is_empty(&self) -> bool { self.released.0 == 0 }
}


impl core::fmt::Debug for State {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		use crate::buttons::Buttons;

		f.debug_struct("Buttons")
		 .field("current", &self.current.display())
		 .field("pushed", &self.pushed.display())
		 .field("released", &self.released.display())
		 .finish()
	}
}
