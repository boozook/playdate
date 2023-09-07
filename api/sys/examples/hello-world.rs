//! Unsafe low-level Hello-World example.

#![no_std]
extern crate alloc;
use core::ffi::*;
use core::ptr::null_mut;
use alloc::boxed::Box;

#[macro_use]
extern crate playdate_sys as pd;
use pd::ffi::*;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (pd::ffi::LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;
const TEXT: &str = "Hello, Rust World";


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
	location: Point<i32>,
	velocity: Point<i32>,
}

impl State {
	const fn new() -> Self {
		Self { location: Point::new(INITIAL_X as _, INITIAL_Y as _),
		       velocity: Point::new(1, 2) }
	}


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		unsafe {
			let graphics = (*pd::API).graphics;
			(*graphics).clear?(LCDSolidColor::kColorWhite as LCDColor);

			let c_text = CString::new(TEXT).ok()?;
			let text_width = (*graphics).getTextWidth?(
			                                           null_mut(),
			                                           c_text.as_ptr() as *const _,
			                                           TEXT.len(),
			                                           PDStringEncoding::kUTF8Encoding,
			                                           0,
			);
			(*graphics).drawText?(
			                      c_text.as_ptr() as *const _,
			                      TEXT.len(),
			                      PDStringEncoding::kUTF8Encoding,
			                      self.location.x,
			                      self.location.y,
			);

			self.location.x += self.velocity.x;
			self.location.y += self.velocity.y;

			if self.location.x < 0 || self.location.x > LCD_COLUMNS as i32 - text_width {
				self.velocity.x = -self.velocity.x;
			}

			if self.location.y < 0 || self.location.y > LCD_ROWS as i32 - TEXT_HEIGHT as i32 {
				self.velocity.y = -self.velocity.y;
			}

			(*(*pd::API).system).drawFPS?(0, 0);
			Some(())
		}
	}


	/// Event handler
	fn event(&mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => unsafe {
				(*(*pd::API).display).setRefreshRate?(20.0);
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
			pd::API = api;

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
