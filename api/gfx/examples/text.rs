#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;

use alloc::vec::Vec;
use core::ffi::CStr;

use sys::ffi::{Playdate, StringEncoding, SystemEvent, LCD_COLUMNS};
use sys::ctrl::EventLoopCtrl;
use system::System;
use gfx::Graphics;


#[unsafe(no_mangle)]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	const C: &[(&CStr, StringEncoding)] = &[
	                                        (c"ASCII", StringEncoding::ASCII),
	                                        // valid for ASCII:
	                                        (c"Hello \xF0\x90\x80World", StringEncoding::ASCII),
	                                        // invalid UTF8:
	                                        (c"Hello \xF0\x90\x80World", StringEncoding::UTF8),
	];

	const UTF8: &[&str] = &[
	                        "Ⓐ Ⓑ 🟨 ⊙ 🔒 🎣 ✛ ⬆ ➡ ⬇ ⬅",
	                        "السلام عليكم",
	                        "Dobrý den",
	                        "Hello",
	                        "שלום",
	                        "नमस्ते",
	                        "こんにちは",
	                        "안녕하세요",
	                        "你好",
	                        "Olá",
	                        "Здравствуйте",
	                        "Hola",
	];


	let gfx = Graphics::new(api.graphics);
	let mut x = 0;
	let mut y = 0;

	let x_bound = |x: &mut i32, y: &mut i32| {
		if *x as u32 >= LCD_COLUMNS {
			*x = 0;
			*y += 20;
		}
	};

	for _ in 0..3 {
		// draw c-strings
		for (s, enc) in C {
			x += gfx.draw_text(*s, s.count_bytes(), *enc, x, y) + 2;
			x_bound(&mut x, &mut y);
		}

		// draw rust' utf8-strings
		for s in UTF8 {
			x += gfx.draw_text(*s, s.chars().count(), StringEncoding::UTF8, x, y) + 2;
			x_bound(&mut x, &mut y);
		}

		// draw rust' utf16-string
		let s = "{utf16-str}".encode_utf16().collect::<Vec<_>>();
		x += gfx.draw_text(&s[..], s.len(), StringEncoding::UTF16, x, y) + 2;
		x_bound(&mut x, &mut y);
	}

	System::new(api.system).update().unset();
	EventLoopCtrl::Continue
}
