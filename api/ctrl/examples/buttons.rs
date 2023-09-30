#![no_std]
extern crate alloc;
use alloc::format;
use alloc::vec::Vec;
use alloc::borrow::Cow;
use core::ptr::NonNull;

#[macro_use]
extern crate sys;
extern crate playdate_controls as controls;

use controls::buttons::PDButtonsExt;
use controls::buttons::PDButtonsIter;
use controls::buttons::IterSingleButtons;
use controls::peripherals::Buttons;

use display::Display;
use gfx::color::Color;
use sys::EventLoopCtrl;
use sys::ffi::PlaydateAPI;
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


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}

	Display::Default().set_refresh_rate(20.);

	// State of the example app:
	let pos = Point::new(INITIAL_X as i32, INITIAL_Y as i32);

	// Create cached end-points that we using every update
	let system = system::System::Cached();
	let graphics = gfx::Graphics::Cached();
	let buttons = Buttons::Cached();

	// Register update handler
	// Just to draw current playback position
	system.set_update_callback_boxed(
	                                 move |(pos, buttons)| {
		                                 graphics.clear(Color::WHITE);

		                                 // Get buttons state
		                                 let buttons = buttons.get();

		                                 // Render buttons state to string
		                                 let text: Cow<str> = if buttons.current.is_empty() {
			                                 "[Press any button]".into()
		                                 } else {
			                                 format!("{:?}", buttons.current.iter().singles().collect::<Vec<_>>()).into()
		                                 };

		                                 // Get width (screen-size) of text
		                                 let font = Default::default();
		                                 let text_width = graphics.get_text_width(&text, font, 0)?;
		                                 // Render text
		                                 graphics.draw_text(text, pos.x, pos.y)?;

		                                 // Move the label position
		                                 const SPEED: i32 = 4;
		                                 if buttons.current.right() {
			                                 pos.x += SPEED;
		                                 }
		                                 if buttons.current.left() {
			                                 pos.x -= SPEED;
		                                 }
		                                 if buttons.current.up() {
			                                 pos.y -= SPEED;
		                                 }
		                                 if buttons.current.down() {
			                                 pos.y += SPEED;
		                                 }

		                                 // Check screen boundaries
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
	                                 (pos, buttons),
	);

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
