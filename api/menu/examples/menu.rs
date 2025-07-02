#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate gfx;
extern crate system;

use core::ffi::CStr;
use alloc::format;
use playdate_menu::{Menu, SimpleMenuItem, CheckMenuItem, OptionsMenuItem};
use sys::ffi::{Playdate, SystemEvent};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};
use system::System;


#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	let system = System::new(api.system);
	let menu = Menu::new(api.system);

	let opts = [c"one", c"two", c"three", c"four"];

	let _ = SimpleMenuItem::new(&system, c"a", Userdata).unwrap();

	let mut count = ClosureCtx(0);
	let _ = SimpleMenuItem::new_with(&system, c"", move || {
		        count.0 += 1;
	        }).unwrap();


	let _ = OptionsMenuItem::new(&system, c"", &opts, Userdata).unwrap();
	let _ = OptionsMenuItem::new_exact(&system, c"", &opts, Userdata).unwrap();

	let mut count = ClosureCtx(0);
	let _ = OptionsMenuItem::new_with(&system, c"b", &opts, move || {
		        count.0 += 1;
	        }).unwrap();


	let mut count = ClosureCtx(0);
	let a = SimpleMenuItem::new_with_ctx(&system, c"a", move |this| {
		        count.0 += 1;
		        let title = this.title(&system);
		        let value = this.value(&system);
		        println!("A called {count:?} times, ({title:?}, {value:?})");
	        }).unwrap();


	let mut count = ClosureCtx(0);
	let b = CheckMenuItem::new_with_ctx(&system, c"b", false, move |this: &_| {
		        count.0 += 1;
		        let title = this.title(&system);
		        let value = this.is_checked(&system);
		        println!("B called {count:?} times, ({title:?}, {value:?})");
	        }).unwrap();


	let mut count = ClosureCtx(0);
	let c = OptionsMenuItem::new_with_ctx(&system, c"c", &opts, move |this: &_| {
		        count.0 += 1;
		        let title = this.title(&system);
		        let value = this.value(&system);
		        println!("C called {count:?} times, ({title:?}, {value:?})");

		        // change itself title:
		        {
			        let b = format!("C {}\0", count.0).into_bytes();
			        let s = unsafe { CStr::from_bytes_with_nul_unchecked(&b) };
			        this.set_title(&system, s);
		        }

		        if value == 3 {
			        menu.remove_all_menu_items();
		        }
	        }).unwrap();


	let mut menu = Some((a, b, c));
	let mut count = 0;
	println!("use system menu");

	system.update().set(move || {
		               count += 1;
		               if count % 20 == 0 {
			               println!("count: {count}/200");
		               }

		               if count == 200 {
			               println!("dropping all menu items");
			               let _ = menu.take();

			               system.update().unset();
		               }
		               UpdateDisplayCtrl::Nope
	               });
	EventLoopCtrl::Continue
}


#[derive(Debug)]
struct ClosureCtx(usize);
impl Drop for ClosureCtx {
	fn drop(&mut self) {
		println!("dropping `ClosureCtx({})`", self.0);
	}
}


#[derive(Debug)]
struct Userdata;
impl Drop for Userdata {
	fn drop(&mut self) {
		println!("dropping `Userdata`");
	}
}
