#![no_std]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use core::ffi::*;
use core::ptr::NonNull;
use pd::ext::PlaydateAPIExt;
use pd::sys::EventLoopCtrl;
use pd::sys::ffi::PlaydateAPI;
use pd::graphics::video::VideoPlayer;
use pd::system::prelude::*;
use pd::graphics::*;
use pd::fs::Path;


const VIDEO_PATH: &Path = "examples/video.pdv";


/// Game state
struct State {
	player: VideoPlayer<video::api::Cache, true>,

	// Current frame
	current: c_int,
	// Number of frames
	length: c_int,
}


/// Entry point
#[no_mangle]
fn event_handler(api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}

	// Set FPS
	api.display().set_refresh_rate(20.0);

	// Create video player
	let player = api.graphics().video().load(VIDEO_PATH).unwrap();
	// Set draw-target to the screen
	player.use_screen_context();

	// Register update handler
	let system = api.system();
	system.set_update_callback_boxed(
	                                 move |state| {
		                                 // Draw current frame of the player
		                                 state.player.render_frame(state.current).unwrap();

		                                 // Advance to the next frame
		                                 state.current += 1;
		                                 if state.current >= state.length {
			                                 state.current = 0;
		                                 }

		                                 // Draw FPS on-top of the player's render
		                                 system.draw_fps(0, 0);

		                                 UpdateCtrl::Continue
	                                 },
	                                 State { length: player.info().frame_count,
	                                         current: 0,
	                                         player, },
	);

	EventLoopCtrl::Continue
}


// Needed for debug build, absolutely optional
ll_symbols!();
