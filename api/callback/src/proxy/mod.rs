use core::marker::Tuple;

/// Default [`Proxy`] implementation.
pub mod default;

/// safe -> unsafe
mod r#unsafe;

/// Spec for special marked argument `:Ud<F>`, on positions: __first, second or trailing__.
mod ud;


/// - `C`: C fn type
/// - `CArgs`: C-fn args
/// - `CRet`: C-fn return-type
/// - `R`: Rust fn type
/// - `RArgs`: Rust-fn args
/// - `RRet`: Rust-fn return-type
pub(crate) trait Proxy<C, CArgs: Tuple, CRet, R, RArgs: Tuple, RRet> {
	/// Wrapped / adapted `Fn`.
	fn fn_fn() -> C
		where R: Fn<RArgs, Output = RRet>;

	/// Wrapped / adapted `FnMut`.
	fn fn_mut() -> C
		where R: FnMut<RArgs, Output = RRet>;

	/// Wrapped / adapted `FnOnce`.
	///
	/// Must [`take`](crate::storage::Store::take) function before the call, then drop.
	fn fn_once() -> C
		where R: FnOnce<RArgs, Output = RRet>;
}
