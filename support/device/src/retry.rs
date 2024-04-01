//! Retry utils

use std::time::Duration;


#[derive(Clone)]
pub struct Retries<Iters: IterTime = DefaultIterTime> {
	/// How many iterations to perform before giving up.
	pub iters: Iters,
	/// Total awaiting time
	pub total: Duration,
}

impl<Iters: IterTime> Retries<Iters> {
	pub fn new(iters: Iters, total: Duration) -> Self { Self { iters, total } }
}

impl<T> Default for Retries<T> where T: Default + IterTime {
	fn default() -> Self {
		Self { iters: Default::default(),
		       total: Duration::from_secs(10) }
	}
}

impl<T> std::fmt::Display for Retries<T> where T: std::fmt::Display + IterTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({} => {:?})", self.iters, self.total)
	}
}
impl<T> std::fmt::Debug for Retries<T> where T: std::fmt::Debug + IterTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:?} => {:?})", self.iters, self.total)
	}
}


pub trait IterTime {
	fn preferred_iter_time(&self) -> Duration;

	#[inline(always)]
	fn interval(&self, total_wait: &Duration) -> Duration
		where for<'t> &'t Self: IterTime {
		calc_interval(total_wait, self)
	}
}


impl<T: IterTime> IterTime for &'_ T {
	#[inline(always)]
	fn preferred_iter_time(&self) -> Duration { T::preferred_iter_time(*self) }

	#[inline(always)]
	fn interval(&self, total_wait: &Duration) -> Duration
		where for<'t> &'t Self: IterTime {
		T::interval(*self, total_wait)
	}
}

pub fn calc_interval<T: IterTime>(wait: &Duration, cfg: T) -> Duration {
	let iters = wait.as_millis() / cfg.preferred_iter_time().as_millis() as u128;
	Duration::from_millis((wait.as_millis() / iters) as _)
}


#[derive(Clone, Default)]
pub struct DefaultIterTime;
const MIN_ITER_TIME: u64 = 100;

impl IterTime for DefaultIterTime {
	fn preferred_iter_time(&self) -> Duration { Duration::from_millis(MIN_ITER_TIME) }
}

impl std::fmt::Display for DefaultIterTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}ms", MIN_ITER_TIME) }
}
impl std::fmt::Debug for DefaultIterTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Duration::from_millis(MIN_ITER_TIME).fmt(f)
	}
}


impl IterTime for Duration {
	fn preferred_iter_time(&self) -> Duration { self.clone() }

	fn interval(&self, total_wait: &Duration) -> Duration
		where for<'t> &'t Self: IterTime {
		calc_interval(total_wait, self)
	}
}
