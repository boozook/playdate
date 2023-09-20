#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate gfx;
extern crate playdate_sprite as sprite;

use core::ffi::*;
use alloc::boxed::Box;

use sys::ffi::*;
use gfx::color::*;
use gfx::bitmap::*;
use sprite::*;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;


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
	rotation: c_float,
	bitmap: Option<Bitmap>,
	sprite: Option<Sprite>,
	velocity: Point<c_float>,
}

impl State {
	const fn new() -> Self {
		Self { rotation: 0.,
		       bitmap: None,
		       sprite: None,
		       velocity: Point::new(1., 2.) }
	}


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		sprite::update_and_draw_sprites();

		// reuse bitmap:
		if let Some(bitmap) = self.bitmap.as_ref() {
			self.rotation += 1.0;
			if self.rotation > 360.0 {
				self.rotation = 0.0;
			}

			bitmap.draw_rotated(CENTER_X as _, CENTER_Y as _, self.rotation, 0.5, 0.5, 1.0, 1.0);
		}

		// move sprite
		if let Some(sprite) = self.sprite.as_ref() {
			let bounds = sprite.bounds();

			if bounds.x < 0. || bounds.x + bounds.width > LCD_COLUMNS as _ {
				self.velocity.x = -self.velocity.x;
			}

			if bounds.y < 0. || bounds.y + bounds.height > LCD_ROWS as _ {
				self.velocity.y = -self.velocity.y;
			}

			sprite.move_by(self.velocity.x, self.velocity.y);
		}

		Some(())
	}


	/// Event handler
	fn event(&'static mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => {
				unsafe { (*(*sys::API).display).setRefreshRate?(60.0) };

				let bitmap = Bitmap::new(50, 50, Color::BLACK).expect("bitmap");
				let sprite = Sprite::new();

				sprite.set_draw_mode(BitmapDrawMode::Copy);
				sprite.set_image(&bitmap, BitmapFlip::Unflipped);
				sprite.move_to(CENTER_X as _, CENTER_Y as _);
				sprite.add();

				self.sprite = Some(sprite);
				self.bitmap = Some(bitmap);
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
