#![no_std]
#![feature(repr_simd)]
#![feature(core_intrinsics)]
#![allow(internal_features)]

extern crate alloc;

#[macro_use]
extern crate function_name;

#[macro_use]
extern crate sys;

use display::Display;
use sys::ctrl::EventLoopCtrl;
use sys::ctrl::UpdateDisplayCtrl;
use sys::ffi::Playdate;
use sys::ffi::SystemEvent;
use system::prelude::*;


#[no_mangle]
fn event_handler(api: &'static Playdate, event: SystemEvent, _: u32) -> EventLoopCtrl {
	if matches!(event, SystemEvent::Init) {
		Display::new(api.display).set_fps(1.);
		System::new(api.system).update()
		                       .set_with(on_update, Default::default());
	}

	EventLoopCtrl::Continue
}


fn on_update(frame: &mut usize) -> UpdateDisplayCtrl {
	match *frame {
		0 => dummy_test(),

		1 => simd::i32(),
		2 => simd::f32(),

		3 => fp32::core(),
		4 => fp32::libm(),
		5 => fp32::num_traits(),

		6 => fp64::core(),
		7 => fp64::libm(),
		8 => fp64::num_traits(),

		9 => tests_complete(),
		_ => System::default().update().unset(),
	}

	*frame += 1;
	UpdateDisplayCtrl::Nope
}


fn dummy_test() { println!("init: OK") }
fn tests_complete() { println!("all tests: OK") }

macro_rules! print_test_name {
	() => {
		println!("test {}", concat!(module_path!(), "::", function_name!()));
	};
	($msg:literal) => {
		println!("test {}: {}",
		         concat!(module_path!(), "::", function_name!()),
		         $msg);
	};
}


pub mod fp32 {
	const F: f32 = 1.4;

	#[named]
	pub fn core() {
		print_test_name!();
		let sin = unsafe { core::intrinsics::sinf32(F) };
		let cos = unsafe { core::intrinsics::cosf32(F) };
		println!("cos: {cos}");
		println!("sin: {sin}");
		print_test_name!("OK");
	}

	#[named]
	pub fn libm() {
		print_test_name!();
		let (sin, cos) = libm::sincosf(F);
		println!("cos: {cos}");
		println!("sin: {sin}");
		print_test_name!("OK");
	}

	#[named]
	pub fn num_traits() {
		print_test_name!();
		let (sin, cos) = num_traits::Float::sin_cos(F);
		println!("cos: {cos}");
		println!("sin: {sin}");
		print_test_name!("OK");
	}
}

pub mod fp64 {
	const F: f64 = 1.4;

	#[named]
	pub fn core() {
		print_test_name!();
		println!("cos: {}", unsafe { core::intrinsics::cosf64(F) });
		println!("sin: {}", unsafe { core::intrinsics::sinf64(F) });
		print_test_name!("OK");
	}

	#[named]
	pub fn libm() {
		print_test_name!();
		let (sin, cos) = libm::sincos(F);
		println!("cos: {cos}");
		println!("sin: {sin}");
		print_test_name!("OK");
	}

	#[named]
	pub fn num_traits() {
		print_test_name!();
		let (sin, cos) = num_traits::real::Real::sin_cos(F);
		println!("cos: {cos}");
		println!("sin: {sin}");
		print_test_name!("OK");
	}
}


pub mod simd {
	use core::intrinsics::simd::simd_add;

	#[repr(simd)]
	#[derive(Clone, Copy, Debug)]
	struct Simd4<T>([T; 4]);


	#[named]
	pub fn i32() {
		print_test_name!();
		unsafe {
			let a = Simd4([10, 10, 10, 10]);
			let b = Simd4([1, 2, 3, 4]);
			let mut res = Simd4([0, 0, 0, 0]);

			for _ in 0..101 {
				res = simd_add(a, simd_add(b, res));
			}
			println!("res: {res:?}");
		}
		print_test_name!("OK");
	}

	#[named]
	pub fn f32() {
		print_test_name!();
		unsafe {
			let a = Simd4::<f32>([10.1, 10.1, 10.1, 10.1]);
			let b = Simd4([1.1, 2.1, 3.1, 4.1]);
			let mut res = Simd4([0.0, 0.0, 0.0, 0.0]);

			for _ in 0..101 {
				res = simd_add(a, simd_add(b, res));
			}
			println!("res: {res:?}");
		}
		print_test_name!("OK");
	}
}
