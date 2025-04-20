#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]


#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;


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


type Api = &'static sys::ffi::PlaydateSys;

// #[const_trait]
// pub trait SystemExt {
// 	fn peripherals(&self) -> Peripherals;
// 	fn accelerometer(&self) -> Accelerometer;
// 	fn buttons(&self) -> Buttons;
// 	fn crank(&self) -> Crank;
// }

// impl const SystemExt for system::System {
// 	fn peripherals(&self) -> Peripherals { Peripherals::new(self.inner()) }
// 	fn accelerometer(&self) -> Accelerometer { Accelerometer::new(self.inner()) }
// 	fn buttons(&self) -> Buttons { Buttons::new(self.inner()) }
// 	fn crank(&self) -> Crank { Crank::new(self.inner()) }
// }
