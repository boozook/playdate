#![no_std]
#[macro_use]
extern crate alloc;
use core::ffi::*;
use alloc::borrow::Cow;
use alloc::boxed::Box;

#[macro_use]
extern crate sys;
extern crate playdate_controls as controls;
use alloc::format;
use alloc::vec::Vec;
use controls::buttons::PDButtonsExt;
use controls::buttons::PDButtonsIter;
use crate::controls::buttons::IterSingleButtons;
use sys::ffi::*;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (sys::ffi::LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;


/// 2D point
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	const fn new(x: T, y: T) -> Point<T> { Point { x, y } }
}


/// App state
struct State {
	pos: Point<i32>,
}

impl State {
	const fn new() -> Self { Self { pos: Point::new(INITIAL_X as _, INITIAL_Y as _) } }


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		// get buttons state
		let buttons = controls::peripherals::Buttons::get()?;

		unsafe {
			let graphics = (*sys::API).graphics;
			(*graphics).clear?(LCDSolidColor::kColorWhite as LCDColor);

			// render buttons state to string
			let text: Cow<str> = if buttons.current.is_empty() {
				"[Press any button]".into()
			} else {
				format!("{:?}", buttons.current.iter().singles().collect::<Vec<_>>()).into()
			};

			let c_text = CString::new(text.as_ref()).ok()?;
			// get width (screen-size) of text
			let text_width = (*graphics).getTextWidth?(
			                                           core::ptr::null_mut(),
			                                           c_text.as_ptr() as *const _,
			                                           text.len(),
			                                           PDStringEncoding::kUTF8Encoding,
			                                           0,
			);
			// render text
			(*graphics).drawText?(
			                      c_text.as_ptr() as *const _,
			                      text.len(),
			                      PDStringEncoding::kUTF8Encoding,
			                      self.pos.x,
			                      self.pos.y,
			);

			// move the label
			const SPEED: i32 = 4;
			if buttons.current.right() {
				self.pos.x += SPEED;
			}
			if buttons.current.left() {
				self.pos.x -= SPEED;
			}
			if buttons.current.up() {
				self.pos.y -= SPEED;
			}
			if buttons.current.down() {
				self.pos.y += SPEED;
			}

			// check screen boundaries
			if self.pos.x < 0 {
				self.pos.x = 0
			} else if self.pos.x > LCD_COLUMNS as i32 - text_width {
				self.pos.x = LCD_COLUMNS as i32 - text_width
			}
			if self.pos.y < 0 {
				self.pos.y = 0
			} else if self.pos.y > LCD_ROWS as i32 - TEXT_HEIGHT as i32 {
				self.pos.y = LCD_ROWS as i32 - TEXT_HEIGHT as i32
			}

			(*(*sys::API).system).drawFPS?(0, 0);
			Some(())
		}
	}


	/// Event handler
	fn event(&mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => unsafe {
				(*(*sys::API).display).setRefreshRate?(20.0);
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
