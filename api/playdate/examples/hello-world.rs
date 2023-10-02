#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate as pd;

use core::ffi::*;
use core::ptr::NonNull;

use pd::sys::ffi::PlaydateAPI;
use display::Display;
use gfx::*;
use gfx::text::*;
use gfx::bitmap::Bitmap;
use gfx::bitmap::Color;
use fs::Path;

use sys::EventLoopCtrl;
use system::prelude::*;
use sound::prelude::*;
use player::*;
use sample::Sample;


const CENTER_X: u32 = Display::COLUMNS / 2;
const CENTER_Y: u32 = Display::ROWS / 2;

const TEXT_HEIGHT: u32 = 20;
const FONT_PATH: &Path = "/System/Fonts/Asheville-Sans-14-Bold.pft";

const BOX_WIDTH: c_int = 100;
const BOX_HEIGHT: c_int = 100;

const URL: &str = concat!(env!("CARGO_PKG_HOMEPAGE"), "\0");
const ENC: StringEncoding = StringEncoding::ASCII;

const IMG_PATH: &Path = "examples/ferris";
const SOUND_PATH: &Path = "examples/main_theme.pda";


/// 2D point
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	const fn new(x: T, y: T) -> Point<T> { Point { x, y } }
}


/// Game state
struct State {
	location: Point<i32>,
	velocity: Point<i32>,
	rotation: c_float,
	image: Bitmap,
	#[allow(dead_code)]
	player: SamplePlayer,
}


impl State {
	fn new() -> Self {
		// Create bitmap
		let image = Bitmap::new(BOX_WIDTH, BOX_HEIGHT, Color::BLACK).unwrap();

		// Push bitmap into context to draw onto it
		gfx::push_context(&image);

		// Load ferris the crab and draw
		let ferris = Bitmap::<bitmap::api::Default>::load(IMG_PATH).expect("missed bitmap");
		gfx::set_draw_mode(BitmapDrawMode::Inverted);
		let (ferris_width, ferris_height) = ferris.size();
		ferris.draw(BOX_WIDTH / 2 - ferris_width / 2, 0, BitmapFlip::Unflipped);

		// Load system font
		let font = text::load_font(FONT_PATH).expect("failed to load font");

		// Draw text lines
		let lines = [("Ferris", None), ("loves", None), ("Playdate!", Some(font))];
		let mut acc_text_height = 0;
		for (line, font) in lines {
			// Get width (screen-size) of the line
			let text_width = text::get_text_width(line, font.as_ref(), 0).expect("get text width");

			// Set font for future drawing
			font.as_ref().map(text::set_font);

			// Draw line
			text::draw_text(
			                line,
			                BOX_WIDTH / 2 - text_width / 2,
			                ferris_height + acc_text_height,
			).expect("invalid string");

			acc_text_height += TEXT_HEIGHT as c_int;
		}

		// Remove our bitmap from the context
		gfx::pop_context();


		// Background music

		// Create player
		let player = SamplePlayer::new().unwrap();

		// load sound
		let sample = Sample::new_from_file(SOUND_PATH).unwrap();
		player.set_sample(&sample);

		// start playback
		player.play(Repeat::LoopsEndlessly, 1.0);

		// Finally store it all in the state
		Self { location: Point::new(CENTER_X as _, CENTER_Y as _),
		       velocity: Point::new(1, 2),
		       rotation: 0.,
		       image,
		       player }
	}


	/// System event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			// Initial setup
			SystemEvent::Init => {
				// Set FPS to 30
				Display::Default().set_refresh_rate(30.0);

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
		gfx::clear(Color::WHITE);


		self.location.x += self.velocity.x;
		self.location.y += self.velocity.y;

		if self.location.x < BOX_WIDTH / 2 || self.location.x > Display::COLUMNS as i32 - BOX_WIDTH / 2 {
			self.velocity.x = -self.velocity.x;
		}

		if self.location.y < BOX_HEIGHT / 2 || self.location.y > Display::ROWS as i32 - BOX_HEIGHT / 2 {
			self.velocity.y = -self.velocity.y;
		}


		let url = CStr::from_bytes_with_nul(URL.as_bytes()).expect("invalid string");

		// Get width (screen-size) of text
		let text_width = text::get_text_width_cstr(url, ENC, None, 0);

		// Draw bottom text
		text::draw_text_cstr(
		                     url,
		                     ENC,
		                     CENTER_X as c_int - text_width / 2,
		                     (Display::ROWS - TEXT_HEIGHT).try_into().unwrap(),
		);

		// Draw bitmap
		self.image.draw_rotated(
		                        self.location.x as _,
		                        self.location.y as _,
		                        self.rotation,
		                        0.5,
		                        0.5,
		                        1.0,
		                        1.0,
		);

		self.rotation += 0.5 * self.velocity.x as c_float;
		if self.rotation > 360.0 {
			self.rotation = 0.0;
		}


		System::Default().draw_fps(0, 0);

		UpdateCtrl::Continue
	}
}


/// Entry point
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	// Unsafe static storage for our state.
	// Usually it's safe because there's only one thread.
	pub static mut STATE: Option<State> = None;
	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }
	}

	// Call state.event
	unsafe { STATE.as_mut().expect("impossible") }.event(event)
}


// Needed for debug build, absolutely optional
ll_symbols!();
