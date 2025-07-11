#![no_std]
#![no_main]
#![allow(unused_must_use)]

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use core::ffi::CStr;

use display::Display;
use system::System;
use sys::ffi::{Playdate, SystemEvent};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};
use gfx::Graphics;
use gfx::video::VideoPlayer;


const FILENAME: &CStr = c"video.pdv";


#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};


	let gfx = Graphics::new(api.graphics);
	let vid = gfx.video();


	let mut player = VideoPlayer::load(&vid, FILENAME).unwrap();
	player.use_screen_context(&vid);

	let info = player.info(&vid);
	Display::new(api.display).set_fps(info.frame_rate);

	let mut frame = 0;

	System::new(api.system).update().set(move || {
		                                player.render_frame(&vid, frame).unwrap();

		                                frame += 1;
		                                if frame == info.frame_count {
			                                frame = 0;
		                                }

		                                UpdateDisplayCtrl::Needed
	                                });


	EventLoopCtrl::Continue
}
