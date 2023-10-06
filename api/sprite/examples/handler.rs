#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate gfx;
extern crate playdate_sprite as sprite;

use core::ffi::*;
use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use gfx::*;
use gfx::color::*;
use gfx::bitmap::Bitmap;
use display::Display;
use sprite::prelude::*;
use sprite::callback::update::*;
use system::event::*;
use system::update::*;

use self::updater::*;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;


/// Game state
struct Game {
	rotation: c_float,
	bitmap: Bitmap,
	sprite: Handle<true, Sprite<UpdState>, Upd>,
}

impl Game {
	fn new() -> Self {
		let bitmap = Bitmap::new(50, 50, Color::BLACK).expect("bitmap");
		let sprite = Sprite::new().into_update_handler::<Upd>();

		sprite.set_draw_mode(BitmapDrawMode::Copy);
		sprite.set_image(&bitmap, BitmapFlip::Unflipped);
		sprite.move_to(CENTER_X as _, CENTER_Y as _);
		sprite.add();

		sprite.set_userdata(UpdState::new());

		Self { rotation: 0.,
		       bitmap,
		       sprite }
	}


	/// Event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			// Initial setup
			SystemEvent::Init => {
				// Set FPS to maximum possible
				Display::Default().set_refresh_rate(0.);

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
		self.rotation += 1.0;
		if self.rotation > 360.0 {
			self.rotation = 0.0;
		}

		// Draw bitmap rotated
		self.bitmap
		    .draw_rotated(CENTER_X as _, CENTER_Y as _, self.rotation, 0.5, 0.5, 1.0, 1.0);

		UpdateCtrl::Continue
	}
}


/// Here is our "update sprite" behavior.
mod updater {
	use core::ffi::c_float;
	use core::marker::PhantomData;

	use sys::ffi::{LCD_COLUMNS, LCD_ROWS};

	use super::sprite;
	use sprite::{AnySprite, SpriteType};
	use sprite::prelude::*;
	use sprite::callback::update::{Handle, SpriteUpdate};


	pub struct Upd<UD = UpdState, T: AnySprite = SpriteRef>(PhantomData<(UD, T::Api)>);

	impl<T: AnySprite, UD> SpriteType for Upd<UD, T> {
		type Api = <T as SpriteApi>::Api;
		type Userdata = UD;
	}

	impl<T, UD> SpriteUpdate for Upd<UD, T>
		where T: AnySprite,
		      Self::Userdata: AsMut<UpdState>
	{
		fn on_update(sprite: &Handle<false, SharedSprite<Self::Userdata, Self::Api>, Self>) {
			if let Some(UpdState { velocity }) = sprite.userdata().map(|ud| ud.as_mut()) {
				let bounds = sprite.bounds();

				if bounds.x < 0. || bounds.x + bounds.width > LCD_COLUMNS as _ {
					velocity.x = -velocity.x;
				}

				if bounds.y < 0. || bounds.y + bounds.height > LCD_ROWS as _ {
					velocity.y = -velocity.y;
				}
				// Move sprite
				sprite.move_by(velocity.x, velocity.y);
			}
		}
	}


	#[derive(Default)]
	pub struct UpdState {
		velocity: Point<c_float>,
	}

	impl UpdState {
		pub fn new() -> Self { Self { velocity: Point::new(1., 2.) } }
	}

	impl AsMut<Self> for UpdState {
		fn as_mut(&mut self) -> &mut Self { self }
	}

	#[derive(Default)]
	struct Point<T> {
		pub x: T,
		pub y: T,
	}

	impl<T> Point<T> {
		const fn new(x: T, y: T) -> Point<T> { Point { x, y } }
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
