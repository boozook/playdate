use core::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NullPtrError;

impl core::error::Error for NullPtrError {
	// Removed text from str to do not store this in output binary.
	/// `description()` is deprecated; use `Display`
	fn description(&self) -> &str { "" }
}

impl fmt::Display for NullPtrError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "NullPtr") }
}


pub trait OkOrNullErr<T> {
	fn ok_or_null<'a>(self) -> Result<&'a T, NullPtrError>;
}

impl<T> OkOrNullErr<T> for *const T {
	fn ok_or_null<'a>(self) -> Result<&'a T, NullPtrError> { unsafe { self.as_ref() }.ok_or(NullPtrError) }
}


pub trait OkMutOrNullErr<T> {
	fn ok_or_null<'a>(self) -> Result<&'a mut T, NullPtrError>;
}

impl<T> OkMutOrNullErr<T> for *mut T {
	fn ok_or_null<'a>(self) -> Result<&'a mut T, NullPtrError> { unsafe { self.as_mut() }.ok_or(NullPtrError) }
}


pub trait OkOrNullFnErr<T> {
	type Error: core::error::Error;
	fn ok_or_null(self) -> Result<T, Self::Error>;
}

impl OkOrNullFnErr<&'static crate::ffi::PlaydateAPI> for crate::ApiRef {
	type Error = NullPtrError;
	fn ok_or_null(self) -> Result<&'static crate::ffi::PlaydateAPI, NullPtrError> { self.ok_or(NullPtrError) }
}


macro_rules! impl_fn_def {
	($($t:ident),*) => {
		unsafe extern "C" fn($($t),*) -> R
	};

	($($t:ident),*, ...) => {
		unsafe extern "C" fn($($t),* ,...) -> R
	}
}

macro_rules! impl_ok_or_null_fn_err {
	($($t:ident),*) => {
		impl<R, $($t),*> OkOrNullFnErr<impl_fn_def!($($t),*)> for Option<impl_fn_def!($($t),*)> {
			type Error = NullPtrError;
			fn ok_or_null(self) -> Result<impl_fn_def!($($t),*), NullPtrError> {
				self.ok_or(NullPtrError)
			}
		}

		impl<'f, R, $($t),*> OkOrNullFnErr<&'f impl_fn_def!($($t),*)> for Option<&'f impl_fn_def!($($t),*)> {
			type Error = NullPtrError;
			fn ok_or_null(self) -> Result<&'f impl_fn_def!($($t),*), NullPtrError> {
				self.ok_or(NullPtrError)
			}
		}
	};

	($($t:ident),* ...) => {
		impl_ok_or_null_fn_err!($($t),*);

		impl<R, $($t),*> OkOrNullFnErr<impl_fn_def!($($t),* ,...)> for Option<impl_fn_def!($($t),* ,...)> {
			type Error = NullPtrError;
			fn ok_or_null(self) -> Result<impl_fn_def!($($t),* ,...), NullPtrError> {
				self.ok_or(NullPtrError)
			}
		}

		impl<'f, R, $($t),*> OkOrNullFnErr<&'f impl_fn_def!($($t),* ,...)> for Option<&'f impl_fn_def!($($t),* ,...)> {
			type Error = NullPtrError;
			fn ok_or_null(self) -> Result<&'f impl_fn_def!($($t),* ,...), NullPtrError> {
				self.ok_or(NullPtrError)
			}
		}
	};
}


impl_ok_or_null_fn_err!();
impl_ok_or_null_fn_err!(A ...);
impl_ok_or_null_fn_err!(A, B ...);
impl_ok_or_null_fn_err!(A, B, C ...);
impl_ok_or_null_fn_err!(A, B, C, D ...);
impl_ok_or_null_fn_err!(A, B, C, D, E ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M, N ...);
impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O ...);


#[cfg(test)]
mod tests {
	use super::*;


	// this just should compile
	fn api_access_ok_() -> Result<(), NullPtrError> {
		crate::sys::api().ok_or_null()?.file.ok_or_null()?;
		crate::sys::api().ok_or_null()?
		                 .file
		                 .ok_or_null()?
		                 .open
		                 .ok_or_null()?;
		Ok(())
	}

	// this just should compile
	fn api_access_raw_() -> Result<(), NullPtrError> {
		unsafe { crate::sys::API }.ok_or_null()?.file.ok_or_null()?;
		unsafe { crate::sys::API }.ok_or_null()?
		                          .file
		                          .ok_or_null()?
		                          .open
		                          .ok_or_null()?;
		Ok(())
	}

	#[test]
	fn api_access_ok() { let _ = api_access_ok_; }

	#[test]
	fn api_access_raw() { let _ = api_access_raw_; }
}


#[cfg(feature = "error-ctx")]
pub mod ctx {
	use core::fmt;


	#[derive(Debug)]
	pub struct NullPtrError {
		pub ctx: &'static str,
	}

	impl fmt::Display for NullPtrError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Ptr to {} is null", self.ctx) }
	}

	impl Into<super::NullPtrError> for NullPtrError {
		/// Convert to non-contextual error.
		/// Removes context of this error.
		fn into(self) -> super::NullPtrError { super::NullPtrError }
	}

	impl core::error::Error for NullPtrError {
		// Removed text from str to do not store this in output binary.
		/// `description()` is deprecated; use `Display`
		fn description(&self) -> &str { "" }
	}

	impl AsRef<str> for NullPtrError {
		fn as_ref(&self) -> &str { self.ctx }
	}


	pub trait OkOrNullCtx<T> {
		fn ok_or_null_ctx<'a>(self, ctx: &'static str) -> Result<&'a T, NullPtrError>;
	}

	impl<T> OkOrNullCtx<T> for *const T {
		fn ok_or_null_ctx<'a>(self, ctx: &'static str) -> Result<&'a T, NullPtrError> {
			unsafe { self.as_ref() }.ok_or(NullPtrError { ctx })
		}
	}


	pub trait OkOrNullAddCtx<T> {
		/// Convert result with non-contextual error to result with error with given `ctx`.
		fn ctx(self, ctx: &'static str) -> Result<T, NullPtrError>;
	}

	impl<T> OkOrNullAddCtx<T> for Result<T, super::NullPtrError> {
		fn ctx(self, ctx: &'static str) -> Result<T, NullPtrError> {
			match self {
				Ok(t) => Ok(t),
				Err(_) => Err(NullPtrError { ctx }),
			}
		}
	}


	pub trait OkOrNullFnCtxErr<T> {
		type Error: core::error::Error;
		fn ok_or_null_ctx(self, ctx: &'static str) -> Result<T, Self::Error>;
	}

	impl OkOrNullFnCtxErr<&'static crate::ffi::PlaydateAPI> for crate::ApiRef {
		type Error = NullPtrError;
		fn ok_or_null_ctx(self, ctx: &'static str) -> Result<&'static crate::ffi::PlaydateAPI, NullPtrError> {
			self.ok_or(NullPtrError { ctx })
		}
	}

	macro_rules! impl_ok_or_null_fn_err {
		($($t:ident),*) => {
			impl<R, $($t),*> OkOrNullFnCtxErr<impl_fn_def!($($t),*)> for Option<impl_fn_def!($($t),*)> {
				type Error = NullPtrError;
				fn ok_or_null_ctx(self, ctx: &'static str) -> Result<impl_fn_def!($($t),*), NullPtrError> {
					self.ok_or(NullPtrError { ctx })
				}
			}
		};
	}

	impl_ok_or_null_fn_err!();
	impl_ok_or_null_fn_err!(A);
	impl_ok_or_null_fn_err!(A, B);
	impl_ok_or_null_fn_err!(A, B, C);
	impl_ok_or_null_fn_err!(A, B, C, D);
	impl_ok_or_null_fn_err!(A, B, C, D, E);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
	impl_ok_or_null_fn_err!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);


	#[cfg(test)]
	mod tests {
		use super::*;


		// this just should compile
		fn api_access_ok_() -> Result<(), NullPtrError> {
			crate::sys::api().ok_or_null_ctx("api")?
			                 .file
			                 .ok_or_null_ctx("file")?;
			crate::sys::api().ok_or_null_ctx("api")?
			                 .file
			                 .ok_or_null_ctx("file")?
			                 .open
			                 .ok_or_null_ctx("open")?;
			Ok(())
		}

		// this just should compile
		fn api_access_raw_() -> Result<(), NullPtrError> {
			unsafe { crate::sys::API }.ok_or_null_ctx("api")?
			                          .file
			                          .ok_or_null_ctx("file")?;
			unsafe { crate::sys::API }.ok_or_null_ctx("api")?
			                          .file
			                          .ok_or_null_ctx("file")?
			                          .open
			                          .ok_or_null_ctx("open")?;
			Ok(())
		}

		#[test]
		fn api_access_ok() { let _ = api_access_ok_; }

		#[test]
		fn api_access_raw() { let _ = api_access_raw_; }
	}
}
