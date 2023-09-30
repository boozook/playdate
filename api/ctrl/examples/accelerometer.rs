#![no_std]
#[macro_use]
extern crate alloc;
use core::ptr::NonNull;

#[macro_use]
extern crate sys;
extern crate playdate_controls as controls;

use controls::peripherals::Accelerometer;

use display::Display;
use gfx::color::Color;
use sys::error::NullPtrError;
use sys::ffi::PlaydateAPI;
use sys::EventLoopCtrl;
use system::prelude::*;


const INITIAL_X: u32 = Display::COLUMNS / 2;
const INITIAL_Y: u32 = (Display::ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;


/// 2D point
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	const fn new(x: T, y: T) -> Point<T> { Point { x, y } }
}


fn init() -> EventLoopCtrl {
	Display::Default().set_refresh_rate(20.);

	// State of the example app:
	let pos = Point::new(INITIAL_X as i32, INITIAL_Y as i32);

	// Create cached end-points that we using every update
	let system = system::System::Cached();
	let graphics = gfx::Graphics::Cached();

	// Register update handler
	// Just to draw current playback position
	system.set_update_callback_boxed(
	                                 move |pos| {
		                                 graphics.clear(Color::WHITE);

		                                 // get accelerometer data
		                                 let (x, y, z) = Accelerometer::get().ok_or(NullPtrError)?;

		                                 // render state to string
		                                 let text = format!("[{x:.2},{y:.2},{z:.2}]");
		                                 // get width (screen-size) of text
		                                 let font = Default::default();
		                                 let text_width = graphics.get_text_width(&text, font, 0)?;
		                                 // render text
		                                 graphics.draw_text(text, pos.x, pos.y)?;

		                                 // update label position
		                                 pos.x = (Display::COLUMNS as f32 * x) as _;
		                                 pos.y = (Display::ROWS as f32 * y) as _;

		                                 // check screen boundaries
		                                 if pos.x < 0 {
			                                 pos.x = 0
		                                 } else if pos.x > Display::COLUMNS as i32 - text_width {
			                                 pos.x = Display::COLUMNS as i32 - text_width
		                                 }
		                                 if pos.y < 0 {
			                                 pos.y = 0
		                                 } else if pos.y > Display::ROWS as i32 - TEXT_HEIGHT as i32 {
			                                 pos.y = Display::ROWS as i32 - TEXT_HEIGHT as i32
		                                 }

		                                 system.draw_fps(0, 0);

		                                 UpdateCtrl::Continue
	                                 },
	                                 pos,
	);

	EventLoopCtrl::Continue
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	match event {
		SystemEvent::Init => {
			// turn on the accelerometer
			Accelerometer::enable().ok_or(NullPtrError)?;
			return init();
		},

		SystemEvent::Resume | SystemEvent::Unlock => {
			// turn on the accelerometer
			Accelerometer::enable().ok_or(NullPtrError)?;
		},

		SystemEvent::Terminate | SystemEvent::Pause | SystemEvent::Lock => {
			// turn off the accelerometer
			Accelerometer::disable().ok_or(NullPtrError)?;
		},

		// Ignore any other events, just for this minimalistic example
		_ => {},
	}

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
