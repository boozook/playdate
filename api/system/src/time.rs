use sys::ffi::PDDateTime;

use crate::api::Api;


pub trait PDDateTimeExt {
	fn to_epoch(&self) -> u32;
	fn from_epoch(epoch: u32) -> Self;

	fn from_epoch_to(dt: &mut Self, epoch: u32);
}


impl PDDateTimeExt for PDDateTime {
	fn to_epoch(&self) -> u32 {
		let f = super::api::Default::default().convert_date_time_to_epoch();
		unsafe { f(self as *const _ as *mut _) }
	}

	fn from_epoch(epoch: u32) -> Self {
		let mut dt = PDDateTime { year: 0,
		                          month: 0,
		                          day: 0,
		                          weekday: 0,
		                          hour: 0,
		                          minute: 0,
		                          second: 0 };
		let f = super::api::Default::default().convert_epoch_to_date_time();
		unsafe { f(epoch, &mut dt) };
		dt
	}

	fn from_epoch_to(dt: &mut Self, epoch: u32) {
		let f = super::api::Default::default().convert_epoch_to_date_time();
		unsafe { f(epoch, dt) }
	}
}
