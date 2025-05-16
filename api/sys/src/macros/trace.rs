#[macro_export]
macro_rules! trace_stack_size {
	() => {{
		$crate::macros::trace::trace_stack_size!(@do: "")
	}};

	($fn:ident) => {{
		use $crate::macros::trace::*;
		trace_stack_size!(@do: "{}", type_name_of(&$fn))
	}};

	(@do: $($arg:tt)+) => {
		#[cfg(not(miri))]
		{
			static mut LOCAL_STACK_SIZE_TRACED: bool = false;
			if !unsafe { LOCAL_STACK_SIZE_TRACED } {
				$crate::macros::trace::trace!("stack": "{} bytes for {}",
														  $crate::macros::trace::stack_size(),
														  format_args!($($arg)+)
														  );
				unsafe { LOCAL_STACK_SIZE_TRACED = true };
			}
		}
	};
}
pub use trace_stack_size;


macro_rules! trace_alloc {
	// This should be heap/allocation-free, so here could be used on-stack fmt:
	(@fmt $num:expr) => {{
			use crate::print::fmt::n2ascii;
			let mut buf = [0_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0];
			let mut index = buf.len() - 3;
			n2ascii($num, &mut index, &mut buf);
			buf.rotate_left(index);
			buf[0] = 32;
			buf
	}};

	(@print $pat:literal $(($(len:$len:expr, s:$s:ident)? $(o:$o:ident)? $(p:$p:expr)?)),+) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "stack"))]
		if let Some(f) = $crate::macros::api_opt!(system.logToConsole) {
			unsafe {
				f(
					$pat.as_ptr().cast(),
					$(
						$($len, $s.as_ptr(),)?
						$($o,)?
						$($p,)?
					)+
				)
			}
		}
	}};


	// api:
	(realloc ptr=$ptr:ident, size=$size:ident) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"⎣alloc⎦ api.realloc";
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob\t\t%#x" (len:pre.count_bytes(), s:pre), (o:$size), (p:$ptr));
	}};

	//
	// global:
	//
	(global::alloc size=$size:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"⎡alloc⎤ global.alloc"; // ⎟
		let size = $size;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob" (len:pre.count_bytes(), s:pre), (o:size));
	}};
	(global::dealloc ptr=$ptr:ident, size=$size:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| global.dealloc";
		let size = $size;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob\t\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (p:$ptr));
	}};
	(global::realloc ptr=$ptr:ident, size=$size:expr, size=$aim:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| global.realloc";
		let size = $size;
		let aim = $aim;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob -> %ob\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (o:aim), (p:$ptr));
	}};

	//
	// local:
	//
	(local::allocate size=$size:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"⎡alloc⎤ local.allocate";
		let size = $size;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob" (len:pre.count_bytes(), s:pre), (o:size));
	}};
	(local::deallocate ptr=$ptr:expr, size=$size:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| local.deallocate";
		let size = $size;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob\t\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (p:$ptr));
	}};
	(local::grow ptr=$ptr:expr, size=$size:expr, size=$aim:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| local.grow";
		let size = $size;
		let aim = $aim;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob -> %ob\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (o:aim), (p:$ptr));
	}};
	(local::grow_zeroed ptr=$ptr:expr, size=$size:expr, size=$aim:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| local.grow_zeroed";
		let size = $size;
		let aim = $aim;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob -> %ob\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (o:aim), (p:$ptr));
	}};
	(local::shrink ptr=$ptr:expr, size=$size:expr, size=$aim:expr) => {{
		#![cfg(any(pdtrace = "all", pdtrace = "alloc"))]
		let pre = c"|alloc| local::shrink";
		let size = $size;
		let aim = $aim;
		$crate::macros::trace::trace_alloc!(@print c"%.*s\t%ob -> %ob\t%#x" (len:pre.count_bytes(), s:pre), (o:size), (o:aim), (p:$ptr));
	}};
}
pub(crate) use trace_alloc;


#[macro_export]
macro_rules! trace {
	// shorthands:
	(stack $(: $($arg:tt)*)?) => {{
		$crate::macros::trace::trace_stack_size!($($($arg)*)?)
	}};
	(alloc $(: $($arg:tt)*)?) => {{ compile_error!("use `trace_alloc` instead"); }};

	// fmt unification:
	($kind:literal: [t=$t:ident]) => {{
		use $crate::macros::trace::{type_name_of, trace};
		trace!($kind: "{}", type_name_of(&$t))
	}};
	// ($kind:literal: [t=$t:ident] $arg:tt) => {{
	// 	use $crate::macros::trace::{type_name_of, trace};
	// 	trace!($kind: "{} {}", type_name_of(&$t), format_args!($arg))
	// }};
	($kind:literal: [t=$t:ident] $($arg:tt)+) => {{
		use $crate::macros::trace::{type_name_of, trace};
		trace!($kind: "{} {}", type_name_of(&$t), format_args!($($arg)+))
	}};


	($kind:literal: [t:$t:ty]) => {{
		$crate::macros::trace::trace!($kind: "{}", core::any::type_name::<$t>())
	}};
	// ($kind:literal: [t:$t:ty] $arg:tt) => {{
	// 	$crate::macros::trace::trace!($kind: "{} {}", core::any::type_name::<$t>(), format_args!($arg))
	// }};
	($kind:literal: [t:$t:ty] $($arg:tt)+) => {{
		$crate::macros::trace::trace!($kind: "{} {}", core::any::type_name::<$t>(), format_args!($($arg)+))
	}};


	($kind:literal: $($arg:tt)+) => {{
		#![deny(unexpected_cfgs)] // XXX, TODO: allow unexpected cfgs here <--- before publish!
		#[cfg(any(pdtrace="all", pdtrace=$kind))] {
			use $crate::macros::{println, trace};
			#[cfg(debug_assertions)]
			println!("[{}] {} at {}", $kind, format_args!($($arg)*), trace::caller_location());
			#[cfg(not(debug_assertions))]
			println!("[{}] {}", $kind, format_args!($($arg)*));
		}
	}};
}
pub use trace;


#[doc(hidden)]
#[inline(always)]
pub const fn type_name_of<T>(_: &T) -> &'static str { core::any::type_name::<T>() }

#[doc(hidden)]
#[track_caller]
pub const fn caller_location() -> &'static core::panic::Location<'static> {
	::core::intrinsics::caller_location()
}

#[doc(hidden)]
#[inline(never)]
#[cfg(not(miri))]
pub fn stack_size() -> isize {
	if cfg!(any(pdtrace = "all", pdtrace = "stack")) {
		unsafe extern "Rust" {
			#[link_name = "pdtrace_stack_bottom"]
			static mut BOTTOM: *const ();
		}
		let v = ();
		(core::ptr::addr_of!(v).addr() as isize) - (unsafe { BOTTOM.addr() } as isize)
	} else {
		-1
	}
}
