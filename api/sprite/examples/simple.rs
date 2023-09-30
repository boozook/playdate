#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate gfx;
extern crate playdate_sprite as sprite;

use core::ffi::*;
use core::ptr::NonNull;

use sys::ffi::*;
use sys::EventLoopCtrl;
use display::Display;
use system::event::*;
use system::update::*;
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


/// Game state
struct Game {
	rotation: c_float,
	bitmap: Option<Bitmap>,
	sprite: Option<Sprite>,
	velocity: Point<c_float>,
}

impl Game {
	const fn new() -> Self {
		Self { rotation: 0.,
		       bitmap: None,
		       sprite: None,
		       velocity: Point::new(1., 2.) }
	}


	/// Event handler
	fn event(&'static mut self, event: PDSystemEvent) -> EventLoopCtrl {
		match event {
			// Initial setup
			SystemEvent::Init => {
				// Set FPS to maximum possible
				Display::Default().set_refresh_rate(0.);

				let bitmap = Bitmap::new(50, 50, Color::BLACK).expect("bitmap");
				let sprite = Sprite::new();

				sprite.set_draw_mode(BitmapDrawMode::Copy);
				sprite.set_image(&bitmap, BitmapFlip::Unflipped);
				sprite.move_to(CENTER_X as _, CENTER_Y as _);
				sprite.add();

				self.sprite = Some(sprite);
				self.bitmap = Some(bitmap);

				// Register our update handler that defined below
				self.set_update_handler();
			},
			_ => {},
		}

		EventLoopCtrl::Continue
	}
}


impl Update for Game {
	/// Updates the state
	fn update(&mut self) -> UpdateCtrl {
		sprite::update_and_draw_sprites();

		// Reuse bitmap:
		if let Some(bitmap) = self.bitmap.as_ref() {
			self.rotation += 1.0;
			if self.rotation > 360.0 {
				self.rotation = 0.0;
			}

			// Draw bitmap rotated
			bitmap.draw_rotated(CENTER_X as _, CENTER_Y as _, self.rotation, 0.5, 0.5, 1.0, 1.0);
		}

		// Move sprite
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

		UpdateCtrl::Continue
	}
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Unsafe static storage for our state.
	// Usually it's safe because there's only one thread.
	pub static mut GAME: Option<Game> = None;
	if unsafe { GAME.is_none() } {
		let state = Game::new();
		unsafe { GAME = Some(state) }
	}

	// Call state.event
	unsafe { GAME.as_mut() }.expect("impossible").event(event)
}


// Needed for debug build
ll_symbols!();
