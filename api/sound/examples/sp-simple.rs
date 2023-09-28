#![no_std]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_sound as sound;

use core::ffi::*;
use core::ptr::NonNull;

use sys::ffi::*;
use sys::EventLoopCtrl;
use system::prelude::*;
use gfx::color::Color;
use fs::Path;

use sound::player;
use sound::sample::Sample;
use player::sp::*;
use player::Repeat;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: PDSystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}

	// create player
	let player = Player::<api::Cache>::new()?;

	// load sound
	const SOUND_PATH: &Path = "sfx/main_theme.pda";
	let sample = Sample::new_from_file(SOUND_PATH)?;
	player.set_sample(&sample);

	// start playback
	player.play(Repeat::LoopsEndlessly, 1.0);


	// Register update handler
	// just to draw current playback position
	let system = system::System::Default();
	system.set_update_callback_boxed(
	                                 move |player| {
		                                 let text = {
			                                 let offset = player.offset();
			                                 let length = player.length();
			                                 format!("> {:.2} / {:.2}", offset, length)
		                                 };

		                                 gfx::clear(Color::WHITE);

		                                 // get width (screen-size) of text
		                                 let text_width = gfx::text::get_text_width(&text, None, 0)?;

		                                 // render text with current player position
		                                 let x = INITIAL_X as c_int - text_width / 2;
		                                 let y = INITIAL_Y as _;
		                                 gfx::text::draw_text(text, x, y)?;

		                                 UpdateCtrl::Continue
	                                 },
	                                 player,
	);

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
