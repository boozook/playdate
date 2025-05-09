macro_rules! trace {
	() => {
		#[cfg(debug_assertions)]
		{
			#[cfg(all(test, not(miri)))]
			::std::print!("\n");
			#[cfg(all(miri, not(test)))]
			{
				#[cfg(miri)]
				extern "Rust" {
					pub fn miri_write_to_stdout(bytes: &[u8]);
				}
				unsafe { miri_write_to_stdout("\n".as_bytes()) };
			}
		}
	};
	($($arg:tt)*) => {
		#[cfg(debug_assertions)]
		{
			#[cfg(all(test, not(miri)))]
			::std::println!($($arg)*);
			#[cfg(all(miri, not(test)))]
			{
				#[cfg(miri)]
				extern "Rust" {
					pub fn miri_write_to_stdout(bytes: &[u8]);
				}
				let s = alloc::format!($($arg)*);
				unsafe { miri_write_to_stdout(s.as_bytes()) };
			}
		}
	};
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
