#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;
extern crate sys;

use core::ffi::c_uint;
use core::marker::PhantomData;

use display::DisplayScale;
use gfx::BitmapFlip;
use gfx::BitmapFlipExt;
use gfx::bitmap;
use gfx::bitmap::Bitmap;
use gfx::bitmap::table::BitmapTable;
use sprite::Sprite;
use sprite::SpriteType;
use sys::ffi::PDRect;
use sys::traits::AsRaw;

use sprite::AnySprite;
use sprite::prelude::*;
use sprite::callback::update::SpriteUpdate;
use sprite::callback::update;
use sprite::callback::draw::SpriteDraw;
use sprite::callback::draw;


const CRANK_FRAME_COUNT: u8 = 12;
const TEXT_FRAME_COUNT: u8 = 14;

type MySprite = Sprite<State, sprite::api::Default>;
type UpdHandle = update::Handle<true, MySprite, UpdateDraw>;
type DrwHandle = draw::l2::Handle<true, MySprite, UpdHandle, UpdateDraw>;


pub struct CrankIndicator {
	sprite: DrwHandle,
}

impl CrankIndicator {
	pub fn new(scale: DisplayScale) -> Result<Self, gfx::error::ApiError> {
		let state = State::new(scale)?;

		let sprite = Sprite::<_, sprite::api::Default>::new().into_update_handler::<UpdateDraw>();
		sprite.set_ignores_draw_offset(true);
		sprite.set_bounds(state.bounds());

		sprite.set_userdata(state);
		Ok(Self { sprite: sprite.into_draw_handler::<UpdateDraw>() })
	}


	pub fn set_scale(&self, scale: DisplayScale) { self.sprite.userdata().map(|state| state.set_scale(scale)); }

	pub fn set_offset(&self, x: i8, y: i8) {
		self.sprite
		    .userdata()
		    .map(|state| state.set_offset(Point::new(x, y)));
	}
}


impl AsRaw for CrankIndicator {
	type Type = sys::ffi::LCDSprite;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.sprite.as_raw() }
}
impl SpriteApi for CrankIndicator {
	type Api = sprite::api::Default;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		self.sprite.api()
	}

	fn api_ref(&self) -> &Self::Api { self.sprite.api_ref() }
}
impl AnySprite for CrankIndicator {}

impl SpriteType for CrankIndicator {
	type Api = <Self as SpriteApi>::Api;
	type Userdata = <UpdateDraw as SpriteType>::Userdata;
}


pub struct UpdateDraw<T: AnySprite = SpriteRef>(PhantomData<T>);

impl<T: AnySprite> SpriteType for UpdateDraw<T> {
	type Api = <T as SpriteApi>::Api;
	type Userdata = State;
	const FREE_ON_DROP: bool = false;
}

impl<T: AnySprite> SpriteUpdate for UpdateDraw<T> {
	#[inline(always)]
	fn on_update(s: &update::Handle<false, SharedSprite<Self::Userdata, Self::Api>, Self>) {
		if let Some(state) = s.userdata() {
			if state.update() {
				s.set_bounds(state.bounds());
				s.mark_dirty();
			} else {
				// skip draw, not dirty
			}
		}
	}
}

impl<T: AnySprite> SpriteDraw for UpdateDraw<T> {
	#[inline(always)]
	fn on_draw(s: &draw::Handle<false, SharedSprite<Self::Userdata, Self::Api>, Self>, bounds: PDRect, _: PDRect) {
		if let Some(state) = s.userdata() {
			let gfx = state.gfx;
			gfx.draw(&state.bubble, bounds.x as _, bounds.y as _, state.bubble_flip);

			const NORM: BitmapFlip = BitmapFlip::Unflipped;

			if let Some(crank) = state.crank_current.as_ref() {
				gfx.draw(&crank, state.crank_pos.x as _, state.crank_pos.y as _, NORM);
			} else if let Some(text) = state.text.as_ref() {
				gfx.draw(
				         &text,
				         state.text_position.x as _,
				         state.text_position.y as _,
				         NORM,
				);
			}
		}
	}
}


pub struct State {
	// background
	bubble: Bitmap<gfx::api::Default>,
	bubble_pos: Point<i16>,
	bubble_size: Size<u8>,
	bubble_flip: BitmapFlip,

	/// Crank animation frames
	crank: BitmapTable<gfx::api::Default>,
	/// Position of current frame of the crank for render
	crank_pos: Point<i16>,
	/// Current frame of the crank animation
	crank_current: Option<Bitmap>,

	// frames of sequence
	frame: u8,
	frame_count: u8,

	// text
	text: Option<Bitmap<gfx::api::Default>>,
	text_frame_count: u8,
	text_offset: i16,
	text_position: Point<i16>,

	/// User set option clockwise
	clockwise: bool,
	/// User set option offset
	offset: Point<i8>,
	/// User set option scale
	scale: DisplayScale,

	/// Last draw moment
	last_time: c_uint,
	/// Need to reload bitmaps
	dirty: bool,

	// cached endpoints
	system: system::System<system::api::Cache>,
	display: display::Display<display::api::Cache>,
	gfx: gfx::Graphics<gfx::api::Cache>,
}

impl State {
	fn new(scale: DisplayScale) -> Result<Self, gfx::error::ApiError> {
		let bubble = load_bubble_for_scale(scale)?;
		let crank = load_crank_for_scale(scale)?;

		let bubble_size = bubble.size();
		let bubble_size = Size::new(bubble_size.0 as _, bubble_size.1 as _);

		let mut this = Self { bubble,
		                      bubble_pos: Point::new(0, 0),
		                      bubble_size,
		                      bubble_flip: BitmapFlip::Unflipped,
		                      crank,
		                      crank_current: None,
		                      crank_pos: Point::new(0, 0),
		                      frame: 1,
		                      frame_count: CRANK_FRAME_COUNT * 3,
		                      text: None,
		                      text_frame_count: 0,
		                      text_position: Point::new(0, 0),
		                      text_offset: 0,
		                      offset: Point::new(0, 0),
		                      clockwise: true,
		                      scale,
		                      last_time: 0,
		                      dirty: false,
		                      system: system::System::new(),
		                      display: display::Display::new(),
		                      gfx: gfx::Graphics::new() };

		this.load_text_if_needed()?;
		this.calc_positions();

		Ok(this)
	}


	fn calc_positions(&mut self) {
		let crank_indicator_y = 210 / self.scale.as_u8();

		if self.system.flipped() {
			let y = self.display.height() as i16 - (crank_indicator_y - self.bubble_size.h / 2) as i16;
			self.bubble_pos = Point::new(0, y);
			self.bubble_flip = BitmapFlip::FlippedXY;
			self.text_offset = 100 / self.scale.as_u8() as i16;
		} else {
			self.bubble_pos.x = self.display.width() as i16 - self.bubble_size.w as i16;
			self.bubble_pos.y = crank_indicator_y as i16 - self.bubble_size.h as i16 / 2;
			self.bubble_flip = BitmapFlip::Unflipped;
			self.text_offset = 76 / self.scale.as_u8() as i16;
		}

		self.frame = 1;
		self.frame_count = CRANK_FRAME_COUNT;

		if let Some(text_frame_image) = &self.text {
			self.text_frame_count = TEXT_FRAME_COUNT;
			self.frame_count = CRANK_FRAME_COUNT + TEXT_FRAME_COUNT;

			let x_offset = self.offset_correction_x();

			let (tw, th) = text_frame_image.size();
			let x = self.bubble_pos.x + x_offset + (self.text_offset - tw as i16) / 2;
			let y = self.bubble_pos.y + self.offset.y as i16 + (self.bubble_size.h as i16 - th as i16) / 2;
			self.text_position.x = x;
			self.text_position.y = y;
		} else {
			self.text_frame_count = 0;
			self.frame_count = CRANK_FRAME_COUNT;
		}
	}


	fn load_text_if_needed(&mut self) -> Result<(), gfx::error::ApiError> {
		if matches!(self.scale, DisplayScale::Normal | DisplayScale::Double) {
			self.text = load_text_for_scale(self.scale)?.into();
		} else {
			self.text.take();
		}
		Ok(())
	}


	fn reload_bitmaps(&mut self) -> Result<(), gfx::error::ApiError> {
		let bubble = load_bubble_for_scale(self.scale)?;
		self.crank = load_crank_for_scale(self.scale)?;

		let bubble_size = bubble.size();
		self.bubble_size = Size::new(bubble_size.0 as _, bubble_size.1 as _);

		self.bubble = bubble;

		self.load_text_if_needed()?;

		self.calc_positions();
		self.dirty = false;

		Ok(())
	}

	fn offset_correction_x(&self) -> i16 {
		// if matches!(self.scale, DisplayScale::Double) {
		// 	self.offset.x - 1
		// } else {
		// 	self.offset.x
		// }

		// this is better:
		self.offset.x as i16
	}

	fn offset_correction_y(&self) -> i16 {
		if matches!(self.scale, DisplayScale::Double | DisplayScale::Quad) {
			self.offset.y as i16 + 1
		} else {
			self.offset.y as i16
		}
	}


	fn set_scale(&mut self, scale: DisplayScale) {
		self.scale = scale;
		self.dirty = true;
	}

	fn set_offset(&mut self, offset: Point<i8>) {
		self.offset = offset;
		self.calc_positions();
	}

	fn update(&mut self) -> bool {
		let mut dirty = self.dirty;
		let last_frame = self.frame;
		let crank_drawn = self.crank_current.is_some();


		if self.dirty {
			self.reload_bitmaps().ok();
		}


		let current_time = self.system.current_time_ms();
		let mut delta = current_time - self.last_time;


		// reset to start frame if `draw` hasn't been called in more than a second
		if delta > 1000 {
			self.frame = 1;
		}

		// normalized steps by delta
		while delta >= 50 {
			self.last_time += 50;
			delta -= 50;
			self.frame += 1;
			if self.frame > self.frame_count {
				self.frame = 1;
			}
		}

		// prepare next frame of the crank
		if self.scale.as_u8() > 2 || self.frame > self.text_frame_count {
			let index = if self.clockwise {
				((self.frame - self.text_frame_count - 1) % CRANK_FRAME_COUNT) + 1
			} else {
				((CRANK_FRAME_COUNT - (self.frame - self.text_frame_count - 1)) % CRANK_FRAME_COUNT) + 1
			} - 1;

			if dirty || self.frame != last_frame {
				dirty = true;

				let frame = self.crank
				                .get::<bitmap::api::Default>(index as _)
				                .expect("missed frame");
				let (fw, fh) = frame.size();

				let x = self.bubble_pos.x + self.offset.x as i16 + (self.text_offset - fw as i16) / 2;
				let y = self.bubble_pos.y + self.offset_correction_y() + (self.bubble_size.h as i16 - fh as i16) / 2;
				self.crank_pos = Point::new(x, y);
				self.crank_current = frame.into();
			}
		} else {
			self.crank_current = None;
		}

		// is dirty:
		// 0. if bitmaps just reloaded,
		// 1. if frame changed,
		// 2. if self.crank_current was None, but now is Some, and otherwise.
		dirty || (crank_drawn != self.crank_current.is_some())
	}


	fn bounds(&self) -> PDRect {
		PDRect { x: (self.bubble_pos.x + self.offset.x as i16) as _,
		         y: (self.bubble_pos.y + self.offset.y as i16) as _,
		         width: self.bubble_size.w as _,
		         height: self.bubble_size.h as _ }
	}
}


fn load_bubble_for_scale(scale: DisplayScale) -> Result<Bitmap<gfx::api::Default>, gfx::error::ApiError> {
	let path = format!("ui/crank-ind/crank-notice-bubble-{}x", scale.as_u8());
	Bitmap::load(path)
}

fn load_text_for_scale(scale: DisplayScale) -> Result<Bitmap<gfx::api::Default>, gfx::error::ApiError> {
	let path = format!("ui/crank-ind/crank-notice-text-{}x", scale.as_u8());
	Bitmap::load(path)
}

fn load_crank_for_scale(scale: DisplayScale)
                        -> Result<BitmapTable<gfx::bitmap::table::api::Default>, gfx::error::ApiError> {
	let path = format!("ui/crank-ind/crank-frames-{}x", scale.as_u8());
	BitmapTable::load(path)
}


/// 2D point
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	const fn new(x: T, y: T) -> Point<T> { Self { x, y } }
}

/// 2D size
struct Size<T> {
	w: T,
	h: T,
}

impl<T> Size<T> {
	const fn new(w: T, h: T) -> Size<T> { Self { w, h } }
}
