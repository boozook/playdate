//! Unsafe low-level Hello-World example.
#![no_std]
#![no_main]

extern crate alloc;
use core::ffi::*;
use core::ptr::null_mut;
use alloc::boxed::Box;
use alloc::borrow::ToOwned;

extern crate playdate_sys as pd;
use pd::ffi::*;
use pd::macros::*;
use pd::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;
const TEXT: &CStr = c"Hello, Rust World";


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

	// cached text width
	text_width: Option<i32>,
}

impl State {
	fn new() -> Self {
		Self { location: Point::new(INITIAL_X as _, INITIAL_Y as _),
		       velocity: Point::new(1, 2),
		       text_width: None }
	}


	fn update(&mut self, mut next_scene: impl FnMut()) -> UpdateDisplayCtrl {
		let api = pd::api().expect("api");
		let graphics = api.graphics;

		// is btn A pressed:
		let pressed = {
			let mut btns = Buttons(0);
			unsafe { (api.system.getButtonState)(&mut btns, null_mut(), null_mut()) }
			(btns & Buttons::A) != Buttons(0)
		};

		if pressed {
			next_scene();
			unsafe { (graphics.clear)(SolidColor::White as Color) }
			return UpdateDisplayCtrl::Needed;
		}


		// get or calc width of text:
		let calc_text_width = || unsafe {
			let f = graphics.getTextWidth;
			f(
			  null_mut(),
			  TEXT.as_ptr().cast(),
			  TEXT.count_bytes(),
			  StringEncoding::ASCII,
			  0,
			)
		};
		let text_width = self.text_width.get_or_insert_with(calc_text_width).to_owned();


		// clean previous frame:
		unsafe {
			let prev_draw_mode = (graphics.setDrawMode)(BitmapDrawMode::FillWhite);
			(graphics.fillRect)(
			                    self.location.x,
			                    self.location.y,
			                    text_width,
			                    (TEXT_HEIGHT + 2) as _, // +2 for comma
			                    SolidColor::White as Color,
			);
			let _ = (graphics.setDrawMode)(prev_draw_mode);
		}

		// calc new position:
		self.location.x += self.velocity.x;
		self.location.y += self.velocity.y;

		if self.location.x < 0 || self.location.x > LCD_COLUMNS as i32 - text_width {
			self.velocity.x = -self.velocity.x;
		}

		if self.location.y < 0 || self.location.y > LCD_ROWS as i32 - TEXT_HEIGHT as i32 {
			self.velocity.y = -self.velocity.y;
		}

		// draw finally:
		unsafe {
			(graphics.drawText)(
			                    TEXT.as_ptr().cast(),
			                    TEXT.count_bytes(),
			                    StringEncoding::ASCII,
			                    self.location.x,
			                    self.location.y,
			);

			(api.system.drawFPS)(0, 0);
		}

		UpdateDisplayCtrl::Nope
	}


	/// Proxy update callback, calls `State::update`
	unsafe extern "C" fn on_update(ptr: *mut c_void) -> c_int {
		// previous state to drop:
		let mut prev = None;

		// set callback & state to the new:
		let reset_update = {
			// borrowed state to move into closure:
			let prev = &mut prev;
			move || unsafe {
				// here could be a new callback & new state instead of null, but this is just example:
				api!(system.setUpdateCallback)(None, null_mut());
				println!("switched to the next scene (none), kinda...");
				*prev = Some(ptr)
			}
		};

		// get self, call update:
		let updated = {
			let state = unsafe {
				let ptr: *mut Self = ptr.cast();
				ptr.as_mut().expect("missed state")
			};
			state.update(reset_update).into()
		};

		// drop old state:
		prev.inspect(|_| println!("dropping prev state..."))
		    .map(|p| Box::from_raw(p as *mut Self))
		    .map(|b| drop(b))
		    .inspect(|_| println!("prev state dropped."));

		updated
	}
}


#[no_mangle]
/// System Event Handler
pub extern "C" fn eventHandlerShim(api: &'static Playdate, event: SystemEvent, key: c_uint) -> c_int {
	let (event, _) = dbg!(event, key);

	match event {
		SystemEvent::Init => {
			// register the API entry point:
			pd::set_api(api);

			// set fps:
			unsafe { (api.display.setRefreshRate)(30.0) }

			let state = Box::into_raw(Box::new(State::new()));

			// register update callback with our state as user-data:
			unsafe {
				(api.system.setUpdateCallback)(Some(State::on_update), state.cast());
			}
		},
		_ => {},
	}

	EventLoopCtrl::Continue.into()
}


#[cfg(miri)]
#[no_mangle]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize { pd::mock::executor::minimal() }


// Needed for device target when building with arm-gcc and linking with its stdlib.
// ll_symbols!();
