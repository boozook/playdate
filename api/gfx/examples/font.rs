#![no_std]
#![no_main]
#![allow(unused_must_use)]

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use sys::ffi::{Playdate, StringEncoding, SystemEvent};
use sys::ctrl::EventLoopCtrl;
use system::System;
use fs::path::Path;
use gfx::text::Font;
use gfx::Graphics;


const FONT_SYSTEM_ASHEVILLE_LIGHT: &'static Path = c"/System/Fonts/Asheville-Sans-14-Light.pft";
const FONT_SYSTEM_ASHEVILLE_BOLD: &'static Path = c"/System/Fonts/Asheville-Sans-14-Bold.pft";
const FONT_SYSTEM_ROOBERT: &'static Path = c"/System/Fonts/Roobert-10-Bold.pft";
const FONT_SYSTEM_CUBERICK: &'static Path = c"/System/Settings.pdx/fonts/font-Cuberick-Bold.pft";
const FONT_SYSTEM_NONTENDO: &'static Path = c"/System/InputTest.pdx/images/Nontendo-Bold-Mono.pft";
const FONT_WRONG: &'static Path = c"wrong-path.pft";

const FONTS: &[&Path] = &[FONT_SYSTEM_ASHEVILLE_LIGHT,
                          FONT_SYSTEM_ASHEVILLE_BOLD,
                          FONT_SYSTEM_ROOBERT,
                          FONT_SYSTEM_CUBERICK,
                          FONT_SYSTEM_NONTENDO,
                          FONT_WRONG];


const TEXT: &[&str] = &["Hello", "こんにちは", "ⒶⒷ🟨⊙🔒🎣✛⬆➡⬇⬅"];


#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	let gfx = Graphics::new(api.graphics);

	// load fonts
	let fonts = FONTS.into_iter().map_while(move |path| {
		                             Font::load(&gfx, path).inspect_err(|err| println!("font load error: {err}"))
		                                                   .ok()
	                             });

	// left offset
	let x = 20;
	// acc y
	let mut y = 10;

	for font in fonts {
		gfx.set_font(&font);

		for s in TEXT {
			let slen = s.chars().count();
			let w = gfx.draw_text(*s, slen, StringEncoding::UTF8, x, y);
			if w > 0 {
				y += font.height(&gfx) as i32;
			}
		}
		y += 4;
	}

	System::new(api.system).update().unset();
	EventLoopCtrl::Continue
}
