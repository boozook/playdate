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
use gfx::color::*;

use gfx::bitmap;
use gfx::bitmap::{BitmapDrawMode, BitmapFlip, BitmapFlipExt};

use gfx::text;
use gfx::text::StringEncodingExt;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;
const TEXT_HEIGHT: u32 = 16;
const FONT_PATH: &'static str = "/System/Fonts/Asheville-Sans-14-Bold.pft";


/// App state
struct State {
	rotation: c_float,
	image: bitmap::Bitmap,
	font: text::Font,
}

impl State {
	fn new() -> Self {
		let font = text::load_font(FONT_PATH).unwrap();
		let bitmap = bitmap::Bitmap::new(100, 100, color::Color::BLACK).unwrap();

		// Indexes of symbols in system representation:
		// Note, UTF-codes is also should be acceptable.
		const RUST: [u32; 4] = [82, 117, 115, 116];

		let page = text::get_font_page(&font, RUST[0]).unwrap();

		// Create cached api end-point
		let gfx = Graphics::Cached();

		// draw some glyphs to bitmap:
		const OFFSET: i32 = 16;
		gfx.push_context(&bitmap);
		for (i, code) in RUST.into_iter().enumerate() {
			let mut advance = 0;
			let (glyph, bitmap_ref) = text::get_page_glyph_with_bitmap(&page, code, &mut advance).unwrap();

			let kern = RUST.get(i + 1)
			               .map(|next| text::get_glyph_kerning(&glyph, code, *next))
			               .unwrap_or_default();

			let char = bitmap_ref.into_bitmap().unwrap();
			let w = char.size().0;
			let x = OFFSET + i as i32 * w;
			let y = OFFSET + kern;

			gfx.set_draw_mode(BitmapDrawMode::kDrawModeInverted);
			char.draw(x as _, y as _, BitmapFlip::Unflipped);
		}
		gfx.pop_context();

		Self { rotation: 0.,
		       image: bitmap,
		       font }
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
		const ENC: text::StringEncoding = text::StringEncoding::ASCII;
		let cstr = CStr::from_bytes_with_nul(LABEL_DEF.as_bytes()).unwrap();

		// Create cached api end-point
		let gfx = Graphics::Cached();

		gfx.clear(Color::WHITE);

		// get width (screen-size) of text
		let text_width = gfx.get_text_width_cstr(cstr, ENC, Some(&self.font), 0);

		// set font
		gfx.set_font(&self.font);

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
