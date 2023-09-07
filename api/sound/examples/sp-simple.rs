#![no_std]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_sound as sound;

use core::ffi::*;
use alloc::borrow::Cow;
use alloc::boxed::Box;

use sys::ffi::*;
use fs::Path;
use sound::player;
use sound::sample::Sample;
use player::sp::*;
use player::Repeat;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;


/// App state
struct State {
	player: Option<Player>,
}

impl State {
	const fn new() -> Self { Self { player: None } }


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		let text: Cow<str> = if let Some(player) = self.player.as_ref() {
			let offset = player.try_get_offset().ok()?;
			let length = player.try_get_length().ok()?;
			format!("{:.2} / {:.2}", offset, length).into()
		} else {
			"no player".into()
		};
		let cstr = CString::new(text.as_ref()).ok()?;


		unsafe {
			let graphics = (*sys::API).graphics;
			(*graphics).clear?(LCDSolidColor::kColorWhite as LCDColor);

			// get width (screen-size) of text
			let text_width = (*graphics).getTextWidth?(
			                                           core::ptr::null_mut(),
			                                           cstr.as_ptr() as *const _,
			                                           text.len(),
			                                           PDStringEncoding::kUTF8Encoding,
			                                           0,
			);
			// render text
			(*graphics).drawText?(
			                      cstr.as_ptr() as *const _,
			                      text.len(),
			                      PDStringEncoding::kUTF8Encoding,
			                      INITIAL_X as c_int - text_width / 2,
			                      INITIAL_Y.try_into().unwrap(),
			);
		}
		Some(())
	}


	/// Event handler
	fn event(&mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => unsafe {
				(*(*sys::API).display).setRefreshRate?(60.0);

				// create player
				self.player = Player::try_new().ok()?.into();
				let player = self.player.as_ref()?;

				// load sound
				const SOUND_PATH: &Path = "sfx/main_theme.pda";
				let sample = Sample::new_from_file(SOUND_PATH);
				player.try_set_sample(&sample).ok()?;

				// start playback
				player.try_play(Repeat::LoopsEndlessly, 1.0).ok()?;
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
