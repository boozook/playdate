#![no_std]
#![no_main]
#![allow(unused_must_use)]

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use display::Display;
use system::System;
use sys::ffi::{Playdate, SystemEvent};
use sys::ffi::{LCD_COLUMNS, LCD_ROWSIZE, LCD_ROWS};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};

use gfx::Graphics;
use gfx::bitmap::Bitmap;
use gfx::color::*;


const CENTER_X: u32 = LCD_COLUMNS / 2;
const CENTER_Y: u32 = LCD_ROWS / 2;
const TILE_WH: i32 = 33; // 8*4 +1px


struct State {
	rotation: f32,
	image: Bitmap,

	gfx: Graphics,
}

impl State {
	fn new(api: &'static Playdate) -> Self {
		let gfx = Graphics::new(api.graphics);
		let pattern = color::pattern::opaque(color::pattern::gfxp::DOT12);
		let mut image = Bitmap::new(&gfx, TILE_WH, TILE_WH, &pattern).unwrap();

		println!("w,h: {:?}", image.size(&gfx));

		// just for example, trigger error
		{
			match image.load_into(&gfx, c"wrong-path") {
				Err(e) => println!("Err: {e:?}"),
				Ok(_) => unreachable!(),
			}
		}

		Self { rotation: 0.,
		       image,
		       gfx }
	}


	/// Updates the state
	fn update(&mut self) -> UpdateDisplayCtrl {
		let gfx = self.gfx;

		gfx.clear(Color::WHITE);


		// draw bitmap
		self.image.draw_rotated(
		                        &gfx,
		                        CENTER_X as _,
		                        CENTER_Y as _,
		                        self.rotation,
		                        0.5,
		                        0.5,
		                        1.0,
		                        1.0,
		);

		// update state
		self.rotation += 1.0;
		if self.rotation > 360.0 {
			self.rotation = 0.0;
		}


		// also draw someshit to debug-framebuffer
		if let Ok(mut dbt) = gfx.debug_frame_buffer() {
			gfx.push_context(&dbt);
			self.image.draw_rotated(
			                        &gfx,
			                        (CENTER_X / 2) as _,
			                        (CENTER_Y / 2) as _,
			                        self.rotation,
			                        0.5,
			                        0.5,
			                        1.0,
			                        1.0,
			);
			gfx.pop_context();

			let mut data = dbt.bitmap_data(&gfx);
			data.data_mut()
			    .chunks_exact_mut(LCD_ROWSIZE as _)
			    .step_by(2)
			    .map(|row| &mut row[(LCD_ROWSIZE - 10) as usize..])
			    .for_each(|right_side| right_side.fill(0xFF));
		}

		UpdateDisplayCtrl::Needed
	}
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	Display::new(api.display).set_fps(0.);
	System::new(api.system).update()
	                       .set_with(State::update, State::new(api));

	EventLoopCtrl::Continue
}
