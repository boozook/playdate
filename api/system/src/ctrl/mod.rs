/*!
### Controls API for PlayDate

High-level controls API built on-top of [playdate-sys][].

Covered components: buttons, accelerometer and crank.

### Usage

__Buttons:__
```no_run
# use playdate_system as system;
# use system::prelude::*;

// Get buttons state
let buttons = system::System::default().input().buttons().state();

if buttons.current.a() {
	println!("button A currently is down")
}
if buttons.pushed.b() {
	println!("button B was pushed")
}

buttons.released
			.into_iter_btns()
			.for_each(|btn| println!("button {btn:?} was released"));
```

__Accelerometer:__
```no_run
# use playdate_system as system;

let accel = system::System::default().input().accelerometer();

// Turn on the accelerometer
accel.enable();

// Get accelerometer data
let (x, y, z) = accel.get();
println!("[{x:.2},{y:.2},{z:.2}]");
```
*/


pub mod buttons;
pub mod api {
	pub mod crank;
	pub mod buttons;
	pub mod accelerometer;


	use accelerometer::Accelerometer;
	use buttons::Buttons;
	use crank::Crank;
	use crate::Api;


	/// Input control peripherals
	#[derive(Clone, Copy)]
	pub struct Ctrl(Api);

	impl Default for Ctrl {
		fn default() -> Self { Self(api!(system)) }
	}

	impl Ctrl {
		pub const fn new(api: Api) -> Self { Self(api) }
	}

	impl Ctrl {
		pub const fn crank(&self) -> Crank { Crank(self.0) }
		pub const fn buttons(&self) -> Buttons { Buttons(self.0) }
		pub const fn accelerometer(&self) -> Accelerometer { Accelerometer(self.0) }
	}
}
