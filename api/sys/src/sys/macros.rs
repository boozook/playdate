#[macro_export]
/// Print line to stdout, simulator's console or device's output.
///
/// Woks like [`std::println!`](https://doc.rust-lang.org/std/macro.println.html).
macro_rules! println {
	() => {{
		$crate::log::println("")
	}};

	($($arg:tt)*) => {{
		$crate::log::println($crate::alloc::format!("{}", format_args!($($arg)*)));
	}};
}


#[macro_export]
/// Get ref to struct of fn from API.
///
/// Panics if meets null-ptr or `None`, unwrap it with `expect(ctx)`.
macro_rules! api {
	(/$($path:ident).*) => {
		core::concat!("api", $(core::concat!(".", core::stringify!($path))),*)
	};

	($($path:ident).*) => {{
		unsafe {
			$crate::api().expect("api")
				$( .$path.as_ref().expect(core::stringify!($path)) )*
		}
	}};
}

#[macro_export]
/// Try get ref to struct of fn from API, returns `Option`.
macro_rules! api_opt {
	($($path:ident).*) => {{
		unsafe {
			#![allow(unused_unsafe)]
			$crate::api()
				$( ?.$path.as_ref() )*
		}
	}};
}

#[macro_export]
/// Try get ref to struct of fn from API,
/// returns `Result` with [`crate::error::NullPtrError`].
macro_rules! api_ok {
	($($path:ident).*) => {{
		unsafe {
			#![allow(unused_unsafe)]
			#![allow(unused_imports)]
			use $crate::error::OkOrNullErr as _;
			use $crate::error::OkOrNullFnErr as _;

			$crate::api().ok_or_null()
				$( ?.$path.ok_or_null() )*
		}
	}};
}

#[cfg(feature = "error-ctx")]
#[macro_export]
/// Try get ref to struct of fn from API,
/// returns `Result` with [`crate::error::ctx::NullPtrError`].
macro_rules! api_ok_ctx {
	($($path:ident).*) => {{
		unsafe {
			#![allow(unused_unsafe)]
			#![allow(unused_imports)]
			use $crate::error::ctx::OkOrNullCtx as _;
			use $crate::error::ctx::OkOrNullFnCtxErr as _;

			// core::concat!("api", $(core::concat!(".", core::stringify!($path))),*)
			$crate::api().ok_or_null_ctx("api")
				$( ?.$path.ok_or_null_ctx( core::stringify!($path) ) )*
		}
	}};
}


#[macro_export]
/// Get raw ptr to struct or fn of API,
/// __using unsafe pointer dereferencing.__
///
/// Call with trailing `!` to unwrap the `Option<fn>`.
///
/// SEGFAULT or SIGSEGV if meets null-ptr or panics if function is `None`.
macro_rules! api_unchecked {
	(/$($path:ident).*) => {
		core::concat!("api", $(core::concat!(".", core::stringify!($path))),*)
	};

	($($path:ident).*!) => {{
		api_unchecked!($($path).*) .expect(api_unchecked!(/$($path).*))
	}};

	($($path:ident).* $(!)?) => {{
		$crate::API $( .read().$path )*
	}};
}


#[cfg(test)]
mod tests {
	use core::ptr::null_mut;


	// This is also test. It must not run because we don't want SIGSEGV, it's just must build.
	#[allow(dead_code)]
	fn api_unchecked() {
		unsafe {
			let _ = api_unchecked!(file);
			let p = api_unchecked!(sound.channel);
			(*p).newChannel.unwrap()();

			let f = api_unchecked!(file.read!);
			f(null_mut(), null_mut(), 0);

			let f = api_unchecked!(file.read).unwrap();
			f(null_mut(), null_mut(), 0);

			let f = api_unchecked!(sound.channel.newChannel).unwrap();
			f();
		}
	}

	#[test]
	#[should_panic(expected = "api")]
	fn api() {
		let f = api!(file.read);
		unsafe { f(null_mut(), null_mut(), 0) };

		let f = api!(sound.channel.newChannel);
		unsafe { f() };
	}


	#[test]
	fn try_api_opt() {
		fn test() -> Option<()> {
			unsafe {
				api_opt!(file.read)?(null_mut(), null_mut(), 0);
				api_opt!(sound.channel.newChannel)?();
			}
			Some(())
		}

		assert!(test().is_none());
	}

	#[test]
	fn try_api_ok() {
		fn test() -> Result<(), crate::error::NullPtrError> {
			unsafe {
				api_ok!(file.read)?(null_mut(), null_mut(), 0);
				api_ok!(sound.channel.newChannel)?();
			}
			Ok(())
		}

		let res = test();
		assert!(res.is_err());
	}

	#[test]
	#[cfg(feature = "error-ctx")]
	fn try_api_ok_ctx() {
		fn test() -> Result<(), crate::error::ctx::NullPtrError> {
			unsafe {
				api_ok_ctx!(file.read)?(null_mut(), null_mut(), 0);
				api_ok_ctx!(sound.channel.newChannel)?();
			}

			Ok(())
		}
		let res = test();
		assert!(res.is_err());
		assert_eq!("api", res.unwrap_err().ctx);
	}
}
