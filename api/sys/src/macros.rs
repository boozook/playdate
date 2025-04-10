/// Print line simulator's console or device's output channel.
///
/// Woks like [`std::println!`](https://doc.rust-lang.org/std/macro.println.html).
#[macro_export]
macro_rules! println {
	() => {{
		if let Some(api) = $crate::api() {
			$crate::print::print_cstr(api, c"")
		}
	}};

	($($arg:tt)*) => {{
		if let Some(api) = $crate::api() {
			$crate::print::print(api, format_args!($($arg)*))
		}
	}};
}


/// Adapted copy from [std-lib][std-dbg].
///
/// Woks like [`std::dbg!`][std-dbg].
///
/// [std-dbg]: https://doc.rust-lang.org/std/macro.dbg.html
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `println!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `println!`
    // will be malformed.
    () => {
        $crate::println!("[{}:{}:{}]", ::core::file!(), ::core::line!(), ::core::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::column!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}


/// Get ref to struct of fn from API.
///
/// Built in debug profile:
/// panics if meets uninitialized API, unwraps it with `expect(ctx)`.
///
/// Built in release profile:
/// uses `unwrap_unchecked` to get a reference to the API.
#[macro_export]
macro_rules! api {
	() => {
		if ::core::cfg!(debug_assertions) {
			$crate::api().expect("api")
		} else {
			unsafe { $crate::api().unwrap_unchecked() }
		}
	};

	($($path:ident).+) => {
		if ::core::cfg!(debug_assertions) {
			$crate::api().expect(::core::concat!("api", $(::core::concat!(".", ::core::stringify!($path))),+))
		} else {
			#[allow(unused_unsafe)]
			unsafe { $crate::api().unwrap_unchecked() }
		} $( .$path )+
	};

	($($path:ident).+!) => {{
		let r = $crate::macros::api!($($path).+);
		if ::core::cfg!(debug_assertions) {
			r.expect(::core::concat!("api", $(::core::concat!(".", ::core::stringify!($path))),+))
		} else {
			r.unwrap_unchecked()
		}
	}};
}


#[macro_export]
/// Try get ref to struct or fn from API, returns `Option`.
macro_rules! api_opt {
	() => { $crate::api() };

	($($path:ident).+) => {
		$crate::api().map(|o|o$(.$path)+ )
	};
}


/// Try get ref to struct or fn from API,
/// returns `Result` with [`crate::error::ApiError`].
#[macro_export]
macro_rules! try_api {
	($($path:ident).*) => {
		$crate::macros::api_opt!($($path).*).ok_or($crate::error::ApiError)
	};
}


pub use crate::{println, dbg};
pub use crate::{api, api_opt, try_api};


#[cfg(test)]
mod tests {
	mod panicing {
		#![cfg_attr(
		            not(debug_assertions),
		            ignore = "we wouldn't want SIGSEGV, it's just must build."
		)]

		#[test]
		#[should_panic(expected = "api.system")]
		fn a() { let _ = api!(system); }

		#[test]
		#[should_panic(expected = "api.graphics.clear")]
		fn b() { let _ = api!(graphics.clear); }
	}


	mod try_opt {
		#[test]
		fn api_opt() {
			fn test() -> Option<()> {
				unsafe {
					// api_opt!(sound.channel.newChannel)?();
					api_opt!(display.getWidth)?();
				}
				Some(())
			}

			assert!(test().is_none());
		}


		#[test]
		fn try_api() {
			fn test() -> Result<(), crate::error::ApiError> {
				unsafe {
					// try_api!(sound.channel.newChannel)?();
					try_api!(display.getWidth)?();
				}
				Ok(())
			}

			let res = test();
			assert!(res.is_err());
		}
	}
}
