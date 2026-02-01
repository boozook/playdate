#![no_std]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_ui_crank_indicator as ui;

use core::ffi::*;
use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::PlaydateAPI;
use ctrl::buttons::ButtonsExt;
use display::DisplayScale;
use menu::OptionsMenuItem;
use ui::CrankIndicator;
use system::prelude::*;


/// App state
struct State {
	/// system draw offset
	offset: (c_int, c_int),
	/// system scale mode
	scale: DisplayScale,
	/// custom system menu
	fps: OptionsMenuItem<bool, menu::api::Cache, true>,

	/// our neat indicator
	crank: Option<CrankIndicator>,

	// cached endpoints
	system: system::System<system::api::Cache>,
	display: display::Display<display::api::Cache>,
	gfx: gfx::Graphics<gfx::api::Cache>,
	btns: ctrl::peripherals::Buttons<ctrl::api::Cache>,
}

impl State {
	fn new() -> Result<Self, gfx::error::ApiError> {
		fn fps_changed(value: &mut bool) { *value = true }

		let fps = OptionsMenuItem::new("FPS", ["20", "50", "Inf"], Some(fps_changed), false).unwrap();

		let crank = CrankIndicator::new(DisplayScale::Normal)?;
		sprite::add_sprite(&crank);

		Ok(Self { offset: (0, 0),
		          scale: DisplayScale::Normal,
		          crank: crank.into(),
		          fps,
		          gfx: gfx::Graphics::new(),
		          system: system::System::new(),
		          display: display::Display::new(),
		          btns: ctrl::peripherals::Buttons::new() })
	}


	/// Updates the state
	fn update(&mut self) -> UpdateCtrl {
		sprite::update_and_draw_sprites();

		// save current scale to compare
		let old_scale = self.scale;

		// react to input
		let buttons = self.btns.get();
		if buttons.released.right() {
			self.offset.0 += 6;
			self.offset.1 += 2;
			self.gfx.set_draw_offset(self.offset.0, self.offset.1);
		} else if buttons.released.left() {
			self.offset.0 -= 6;
			self.offset.1 -= 2;
			self.gfx.set_draw_offset(self.offset.0, self.offset.1);
		} else if buttons.released.up() {
			self.scale = match self.scale {
				DisplayScale::Normal => DisplayScale::Double,
				DisplayScale::Double => DisplayScale::Quad,
				DisplayScale::Quad => DisplayScale::Eight,
				DisplayScale::Eight => DisplayScale::Eight,
			};
			self.display.set_scale(self.scale);
		} else if buttons.released.down() {
			self.scale = match self.scale {
				DisplayScale::Normal => DisplayScale::Normal,
				DisplayScale::Double => DisplayScale::Normal,
				DisplayScale::Quad => DisplayScale::Double,
				DisplayScale::Eight => DisplayScale::Quad,
			};
			self.display.set_scale(self.scale);
		}

		// create or remove indicator
		if buttons.released.a() {
			self.crank.take();
			let crank = CrankIndicator::new(self.scale)?;
			sprite::add_sprite(&crank);
			self.crank = crank.into();
		} else if buttons.released.b() {
			self.crank.take();
		}

		// update changed scale for indicator
		if old_scale != self.scale {
			self.crank.as_mut().map(|crank| crank.set_scale(self.scale));
		}

		// update frame rate if changed
		self.fps.get_userdata().filter(|v| **v).map(|value| {
			                                       let fps = match self.fps.selected_option() {
				                                       0 => 20.,
			                                          1 => 50.,
			                                          _ => 0.,
			                                       };
			                                       self.display.set_fps(fps);
			                                       *value = false;
		                                       });

		// draw state (offset, scale, fps)
		self.gfx
		    .draw_text_utf8(format!("{}x, {:?}", self.scale, self.offset), 18, 0)?;
		self.system.draw_fps(0, 0);

		UpdateCtrl::Continue
	}
}


#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}

	let state = State::new()?;
	state.display.set_refresh_rate(20.);

	let system = state.system;
	system.set_update_callback_boxed(State::update, state);

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
