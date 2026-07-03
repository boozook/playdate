macro_rules! trace {
	(add: ($F:ty as $Kind:ident => $Store:ty)) => {{
		use $crate::util::macros::trace;
		trace!("cb-add": "{f} as {k} into {s}" ($F as $Kind => $Store));
	}};
	(rem: ($F:ty as $Kind:ident => $Store:ty)) => {{
		use $crate::util::macros::trace;
		trace!("cb-rem": "{f}{} as {k} from {s}" ($F as $Kind => $Store) {
			if core::mem::needs_drop::<$F>() { " (drop)" } else { "" }
		});
	}};
	(call: ($F:ty as $Kind:ident => $Store:ty)) => {{
		use $crate::util::macros::trace;
		trace!("cb-call": "{f} as {k} in {s}" ($F as $Kind => $Store));
	}};
	(get: ($F:ty as $Kind:ident => $Store:ty)) => {{
		use $crate::util::macros::trace;
		trace!("cb-get": "{f} as {k} from {s}" ($F as $Kind => $Store));
	}};

	(re-add: ($F:ty as $Kind:ident => $Store:ty)) => {{
		use $crate::util::macros::trace;
		trace!("cb-add": "[OVERRIDE] {f} as {k} into {s}" ($F as $Kind => $Store));
	}};

	(into: $trait:ident::$fn:ident($c:ty, $res:ty => $r:ty, $ret:ty), $F:ty, $Adapter:ty) => {{
		#![allow(unused_imports)]
		use core::any::type_name;
		use $crate::util::macros::trace;
		trace!("cb-into": "{trait}::{fn}: C fn[{c}] <-- [{r}], size: {s}, ty: {ty}, using {adapter}"
				 trait = stringify!($trait),
				 fn = stringify!($fn),
				 c = format_args!("{} -> {}", type_name::<$c>(), type_name::<$res>()),
				 r = format_args!("{} -> {}", type_name::<$r>(), type_name::<$ret>()),
				 s = size_of::<$F>(),
				 ty = type_name::<$F>(),
				 adapter = type_name::<$Adapter>(),

		);
	}};


	// `tmt` is like "{f} as {k} into {s}"
	($kind:literal: $tmt:literal ($F:ty as $Kind:ident => $Store:ty) $(,)? $($arg:tt)*) => {{
		#![allow(unused_imports)]
		use ::core::any::type_name;
		use ::core::stringify;
		use $crate::util::macros::trace;
		trace!($kind: $tmt $($arg,)*
							f = type_name::<$F>(),
							k = stringify!($Kind),
							s = type_name::<$Store>());
	}};

	($kind:literal: $tmt:literal $($arg:tt)*) => {{
		#[cfg(any(pdtrace = "all", pdtrace = $kind))]
		cfg_select! {
			feature = "sys" => {
				::sys::macros::trace::trace!($kind: $tmt, $($arg)*);
			}
			all(test, not(miri)) => {
				::std::println!("[{}] {}", $kind, format_args!($tmt, $($arg)*));
			}
			all(miri, not(test)) => {{
				extern "Rust" {
					pub fn miri_write_to_stdout(bytes: &[u8]);
				}
				let s = alloc::format!("[{}] {}", $kind, format_args!($tmt, $($arg)*));
				unsafe { miri_write_to_stdout(s.as_bytes()) };
				drop(s);
			}}
			_ => {}
		}
	}};
}

pub(crate) use trace;


#[rustfmt::skip]
macro_rules! tlen {
	() => { 0 };
	($A:ident $(,)?) => { 1 };
	($A:ident $(,)? $B:ident $(,)?) => { 2 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)?) => { 3 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)?) => { 4 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)?) => { 5 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)?) => { 6 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)?) => { 7 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)? $H:ident $(,)?) => { 8 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)? $H:ident $(,)? $I:ident $(,)?) => { 9 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)? $H:ident $(,)? $I:ident $(,)? $J:ident $(,)?) => { 10 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)? $H:ident $(,)? $I:ident $(,)? $J:ident $(,)? $K:ident $(,)?) => { 11 };
	($A:ident $(,)? $B:ident $(,)? $C:ident $(,)? $D:ident $(,)? $E:ident $(,)? $F:ident $(,)? $G:ident $(,)? $H:ident $(,)? $I:ident $(,)? $J:ident $(,)? $K:ident $(,)? $L:ident $(,)?) => { 12 };
}

pub(crate) use tlen;
