#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;


use sys::ffi::{Playdate, SystemEvent};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};
use system::System;


#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	// Just for this example, ignore all events except init:
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	// Api-endpoint:
	let system = System::new(api.system);


	// Examples of various callbacks:

	fn on_update() -> UpdateDisplayCtrl {
		println!("call on_update");

		let api = System::default();
		api.update().set_with(Scene::on_update, Scene(0, api));

		UpdateDisplayCtrl::Nope
	}
	system.update().set(on_update);


	struct Scene(/* some state data: */ u8, System);
	impl Drop for Scene {
		fn drop(&mut self) { println!("Scene::drop()") }
	}
	impl Scene {
		fn on_update(&mut self) -> UpdateDisplayCtrl {
			println!("call Scene::on_update");

			self.0 += 1;
			if self.0 > 2 {
				let api = self.1;
				api.update().set(move || {
					            println!("call Scene::on_update::{{closure}}");

					            let f = |(v, api): &mut (u32, System)| {
						            println!("call Scene::on_update::{{closure}}::{{closure}}");
						            *v += 1;

						            if *v > 2 {
							            api.update().unset();
							            println!("done");
						            }

						            UpdateDisplayCtrl::from(*v)
					            };
					            api.update().set_with(f, (0, api));
					            0
				            });
			}

			UpdateDisplayCtrl::Nope
		}
	}


	EventLoopCtrl::Continue
}
