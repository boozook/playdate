#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use core::ffi::*;
use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::prelude::*;

use gfx::Graphics;
use gfx::bitmap::Bitmap;
use gfx::text::StringEncoding;
use gfx::color::*;
use gfx::text::StringEncodingExt;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;
const TEXT_HEIGHT: u32 = 16;


/// App state
struct State {
	rotation: c_float,
	image: Bitmap,
}

impl State {
	fn new() -> Self {
		let image = Bitmap::new(100, 100, Color::BLACK).unwrap();
		Self { rotation: 0., image }
	}

	/// Event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			// initial setup
			SystemEvent::Init => {
				display::Display::Default().set_refresh_rate(0.);

				// Register our update handler that defined below
				self.set_update_handler();
			},
			_ => {},
		}

		EventLoopCtrl::Continue
	}
}

impl Update for State {
	/// Updates the state
	fn update(&mut self) -> UpdateCtrl {
		const LABEL_DEF: &str = "Just rotating bitmap:\0";
		const ENC: StringEncoding = StringEncoding::ASCII;

		let cstr = CStr::from_bytes_with_nul(LABEL_DEF.as_bytes()).unwrap();

		// Create cached api end-point
		let gfx = Graphics::Cached();

		gfx.clear(Color::WHITE);

		// get width (screen-size) of text
		let font = Default::default();
		let text_width = gfx.get_text_width_cstr(cstr, ENC, font, 0);

		// render text
		gfx.draw_text_cstr(
		                   cstr,
		                   ENC,
		                   CENTER_X as c_int - text_width / 2,
		                   TEXT_HEIGHT.try_into().unwrap(),
		);

		// draw bitmap
		self.image
		    .draw_rotated(CENTER_X as _, CENTER_Y as _, self.rotation, 0.5, 0.5, 1.0, 1.0);

		self.rotation += 1.0;
		if self.rotation > 360.0 {
			self.rotation = 0.0;
		}

		UpdateCtrl::Continue
	}
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Unsafe static storage for our state.
	// Usually it's safe because there's only one thread.
	pub static mut STATE: Option<State> = None;
	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }
	}

	// Call state.event
	unsafe { STATE.as_mut() }.expect("impossible").event(event)
}


// Needed for debug build
ll_symbols!();
