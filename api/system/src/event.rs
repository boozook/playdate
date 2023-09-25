pub use sys::ffi::PDSystemEvent as SystemEvent;


pub trait SystemEventExt {
	#![allow(non_upper_case_globals)]

	/// Program initialization.
	///
	/// After loading pdex.bin into memory, the system calls your event handler with this event.
	///
	/// Then you can supply your own run loop update function
	/// by calling [`System::set_update_callback`](crate::System::set_update_callback)
	/// or [`Update::set_update_handler`](crate::update::Update::set_update_handler) here.
	///
	/// If you don’t provide an update callback, the system initializes a Lua context
	/// and calls your event handler again with event [`InitLua`](SystemEventExt::InitLua).
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventInitage")]
	const Init: SystemEvent = SystemEvent::kEventInit;

	/// Program initialization in __lua context__.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventInitLua")]
	const InitLua: SystemEvent = SystemEvent::kEventInitLua;

	/// System going to locked state.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventLockage")]
	const Lock: SystemEvent = SystemEvent::kEventLock;

	/// System has been unlocked by user.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventUnlock")]
	const Unlock: SystemEvent = SystemEvent::kEventUnlock;

	/// Program execution paused.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventPausee")]
	const Pause: SystemEvent = SystemEvent::kEventPause;

	/// Program execution resumed after [pause](SystemEventExt::Pause).
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventResume")]
	const Resume: SystemEvent = SystemEvent::kEventResume;

	/// Program termination.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventTerminate")]
	const Terminate: SystemEvent = SystemEvent::kEventTerminate;

	/// Simulator key is pressed.
	///
	/// When an arbitrary key is pressed __in the Simulator__
	/// your event handler is called with this event and the keycode of the key in the last argument.
	///
	/// See also [`KeyReleased`](SystemEventExt::KeyReleased).
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventKeyPressed")]
	const KeyPressed: SystemEvent = SystemEvent::kEventKeyPressed;

	/// Simulator key is released.
	///
	/// When an arbitrary key is released __in the Simulator__
	/// your event handler is called with this event and the keycode of the key in the last argument.
	///
	/// See also [`KeyPressed`](SystemEventExt::KeyPressed).
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventKeyReleased")]
	const KeyReleased: SystemEvent = SystemEvent::kEventKeyReleased;

	/// Low power warning by system.
	///
	/// At this point, it's a good idea to persistently save anything you need, such as a save-game.
	#[doc(alias = "sys::ffi::PDSystemEvent::kEventLowPower")]
	const LowPower: SystemEvent = SystemEvent::kEventLowPower;
}


impl SystemEventExt for SystemEvent {}
