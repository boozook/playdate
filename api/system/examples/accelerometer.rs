#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;

use display::Display;
use playdate_system::ctrl::api::accelerometer::Accelerometer;
use playdate_system::System;
use sys::ctrl::UpdateDisplayCtrl;
use sys::ffi::Playdate;
use sys::ctrl::EventLoopCtrl;
use sys::ffi::SystemEvent;


const INITIAL_X: u32 = Display::COLUMNS / 2;
const INITIAL_Y: u32 = Display::ROWS / 2;


/// 2D point
#[derive(Clone, PartialEq)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	const fn new(x: T, y: T) -> Point<T> { Point { x, y } }
}


fn update(accelerometer: Accelerometer) -> impl FnMut() -> UpdateDisplayCtrl {
	// State of the example app:
	let mut pos = Point::new(INITIAL_X as i32, INITIAL_Y as i32);
	let mut prev = pos.clone();

	// Register update handler
	// Just to draw current playback position
	move || {
		// get accelerometer data
		let (x, y, z) = accelerometer.get();

		// update point
		pos.x = (Display::COLUMNS as f32 * x) as _;
		pos.y = (Display::ROWS as f32 * y) as _;

		// screen boundaries
		if pos.x < 0 {
			pos.x = 0
		} else if pos.x > Display::COLUMNS as i32 {
			pos.x = Display::COLUMNS as i32
		}
		if pos.y < 0 {
			pos.y = 0
		} else if pos.y > Display::ROWS as i32 {
			pos.y = Display::ROWS as i32
		}

		if pos != prev {
			println!("[{x:.2}, {y:.2}, {z:.2}]");
			prev = pos.clone();
		}

		UpdateDisplayCtrl::Nope
	}
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	dbg!(e);

	let system = System::new(api.system);
	let accelerometer = system.input().accelerometer();

	match e {
		SystemEvent::Init => {
			// turn on the accelerometer
			accelerometer.enable();
			// set update callback
			system.update().set(update(accelerometer));
		},

		SystemEvent::Resume | SystemEvent::Unlock => {
			// turn on the accelerometer
			accelerometer.enable();
		},

		SystemEvent::Terminate | SystemEvent::Pause | SystemEvent::Lock => {
			// turn off the accelerometer
			accelerometer.disable();
		},

		// Ignore any other events, just for this minimalistic example
		_ => {},
	}

	EventLoopCtrl::Continue
}
