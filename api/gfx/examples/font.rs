#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use core::ffi::*;
use alloc::boxed::Box;

use sys::ffi::*;
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
	image: Option<bitmap::Bitmap>,
	font: Option<text::Font>,
}

impl State {
	const fn new() -> Self {
		Self { rotation: 0.,
		       image: None,
		       font: None }
	}


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		const LABEL_DEF: &str = "Just rotating bitmap:\0";
		const ENC: text::StringEncoding = text::StringEncoding::ASCII;

		let cstr = CStr::from_bytes_with_nul(LABEL_DEF.as_bytes()).unwrap();

		gfx::clear(Color::WHITE);

		// get width (screen-size) of text
		let text_width = gfx::text::get_text_width_cstr(cstr, ENC, self.font.as_ref(), 0);

		// set font
		gfx::text::set_font(self.font.as_ref().unwrap());

		// render text
		gfx::text::draw_text_cstr(
		                          cstr,
		                          ENC,
		                          CENTER_X as c_int - text_width / 2,
		                          TEXT_HEIGHT.try_into().unwrap(),
		);

		// draw bitmap
		if let Some(image) = self.image.as_ref() {
			image.draw_rotated(CENTER_X as _, CENTER_Y as _, self.rotation, 0.5, 0.5, 1.0, 1.0);
		}

		self.rotation += 1.0;
		if self.rotation > 360.0 {
			self.rotation = 0.0;
		}

		Some(())
	}


	/// Event handler
	fn event(&'static mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => {
				unsafe { (*(*sys::API).display).setRefreshRate?(60.0) };

				let font = text::load_font(FONT_PATH).unwrap();
				let bitmap = bitmap::Bitmap::new(100, 100, color::Color::BLACK).unwrap();

				// Indexes of symbols in system representation:
				// Note, UTF-codes is also should be acceptable.
				const RUST: [u32; 4] = [82, 117, 115, 116];

				let page = text::get_font_page(&font, RUST[0]).unwrap();

				// draw some glyphs to bitmap:
				const OFFSET: i32 = 16;
				gfx::push_context(&bitmap);
				for (i, code) in RUST.into_iter().enumerate() {
					let mut advance = 0;
					let (glyph, bitmap_ref) = text::get_page_glyph_with_bitmap(&page, code, &mut advance).unwrap();

					let kern = RUST.get(i + 1)
					               .map(|next| text::get_glyph_kerning(&glyph, code, *next))
					               .unwrap_or_default();

					let char = bitmap_ref.into_bitmap();
					let w = char.size().map(|(w, h)| w).unwrap();
					let x = OFFSET + i as i32 * w;
					let y = OFFSET + kern;

					gfx::set_draw_mode(BitmapDrawMode::kDrawModeInverted);
					char.draw(x as _, y as _, BitmapFlip::Unflipped);
				}
				gfx::pop_context();


				self.font = font.into();
				self.image = Some(bitmap);
			},
			_ => {},
		}
		Some(())
	}
}


#[no_mangle]
/// Proxy event handler, calls `State::event`
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	static mut STATE: Option<Box<State>> = None;

	match event {
		PDSystemEvent::kEventInit => unsafe {
			// register the API entry point
			sys::API = api;

			// create game state
			if STATE.is_none() {
				STATE = Some(Box::new(State::new()));
			}
			let state = STATE.as_mut().unwrap().as_mut() as *mut State;

			// get `setUpdateCallback` fn
			let f = (*(*api).system).setUpdateCallback.expect("setUpdateCallback");
			// register update callback with user-data = our state
			f(Some(on_update), state.cast());
		},
		_ => {},
	}

	if let Some(state) = unsafe { STATE.as_mut() } {
		state.event(event).and(Some(0)).unwrap_or(1)
	} else {
		1
	}
}


/// Proxy update callback, calls `State::update`
unsafe extern "C" fn on_update(state: *mut c_void) -> i32 {
	let ptr: *mut State = state.cast();
	let state = ptr.as_mut().expect("missed state");
	state.update().and(Some(1)).unwrap_or_default()
}


// Needed for debug build
ll_symbols!();
