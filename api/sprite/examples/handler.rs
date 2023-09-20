#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate gfx;
extern crate playdate_sprite as sprite;

use core::ffi::*;
use alloc::boxed::Box;

use sys::ffi::*;
use gfx::*;
use gfx::color::*;
use gfx::bitmap::Bitmap;
use sprite::prelude::*;
use sprite::callback::update::*;

use self::updater::*;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;


/// App state
struct State {
	rotation: c_float,
	bitmap: Option<Bitmap>,
	sprite: Option<Handle<true, Sprite<UpdState>, Upd>>,
}

impl State {
	const fn new() -> Self {
		Self { rotation: 0.,
		       bitmap: None,
		       sprite: None }
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

		Some(())
	}


	/// Event handler
	fn event(&'static mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => {
				unsafe { (*(*sys::API).display).setRefreshRate?(60.0) };

				let bitmap = Bitmap::new(50, 50, Color::BLACK).expect("bitmap");
				let sprite = Sprite::new().into_update_handler::<Upd>();

				sprite.set_draw_mode(BitmapDrawMode::Copy);
				sprite.set_image(&bitmap, BitmapFlip::Unflipped);
				sprite.move_to(CENTER_X as _, CENTER_Y as _);
				sprite.add();

				sprite.set_userdata(UpdState::new());

				self.sprite = Some(sprite);
				self.bitmap = Some(bitmap);
			},
			_ => {},
		}
		Some(())
	}
}


mod updater {
	use core::ffi::c_float;

	use super::sprite;
	use sys::ffi::{LCD_COLUMNS, LCD_ROWS};
	use sys::traits::AsRaw;
	use sprite::AnySprite;
	use sprite::prelude::*;
	use sprite::callback::update::{Handle, SpriteUpdate};


	pub struct Upd<UD = UpdState, T: AnySprite = SpriteRef>(Sprite<UD, T::Api, false>);

	impl<T: AnySprite, UD> AsRef<Sprite<UD, T::Api, false>> for Upd<UD, T> {
		fn as_ref(&self) -> &Sprite<UD, T::Api, false> { &self.0 }
	}

	impl<T: AnySprite, UD> From<T> for Upd<UD, T> where Sprite<UD, T::Api, false>: From<T> {
		fn from(ptr: T) -> Self { Self(Sprite::from(ptr)) }
	}

	impl<T: AnySprite, UD> TypedSprite for Upd<UD, T> {
		type Userdata = UD;
		const FREE_ON_DROP: bool = false;
	}
	impl<T: AnySprite, UD> AsRaw for Upd<UD, T> {
		type Type = <T as AsRaw>::Type;
		unsafe fn as_raw(&self) -> *mut Self::Type { self.0.as_raw() }
	}
	impl<T: AnySprite, UD> SpriteApi for Upd<UD, T> {
		type Api = <T as SpriteApi>::Api;

		fn api(&self) -> Self::Api
			where Self::Api: Copy {
			self.0.api()
		}

		fn api_ref(&self) -> &Self::Api { self.0.api_ref() }
	}


	impl<T, UD> SpriteUpdate for Upd<UD, T>
		where T: AnySprite,
		      Self: From<SpriteRef>,
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
