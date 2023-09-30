#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_menu as menu;

use core::ffi::*;
use core::ptr::NonNull;

use sys::ffi::*;
use sys::EventLoopCtrl;
use system::prelude::*;
use gfx::color::Color;

use menu::*;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;
const LABEL: &str = "Use System Menu";


/// App state
struct State {
	first: Option<SimpleMenuItem<u32>>,
	second: Option<CheckMenuItem<u32>>,
	third: Option<OptionsMenuItem>,
}


fn update(state: &mut State) -> UpdateCtrl {
	// remove third menu item if requested
	if let Some((_, value)) = state.third.as_ref().map(|item| (item, item.selected_option())) {
		if value != 0 {
			state.third.take();
			println!("Third menu item removed.")
		}
	}


	let graphics = gfx::Graphics::Cached();
	graphics.clear(Color::WHITE);

	// Get width (screen-size) of text
	let font = Default::default();
	let text_width = graphics.get_text_width(LABEL, font, 0)?;
	// render text
	graphics.draw_text(
	                   LABEL,
	                   INITIAL_X as c_int - text_width / 2,
	                   INITIAL_Y.try_into().unwrap(),
	)?;

	UpdateCtrl::Continue
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}


	fn on_change(userdata: &mut u32) {
		println!("Menu item changed {userdata} times.");
		*userdata += 1;
	}

	let first = SimpleMenuItem::new("Check Me", Some(on_change), 0)?.into();
	let second = CheckMenuItem::new("Check", false, Some(on_change), 0)?.into();
	let third = OptionsMenuItem::new("Del me?", ["No", "Yes"], None, ())?.into();

	let state = State { first, second, third };

	// Set no-op update callback
	system::System::Default().set_update_callback_boxed(update, state);

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
