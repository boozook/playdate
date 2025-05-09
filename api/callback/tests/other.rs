#![feature(tuple_trait)]
#![feature(min_specialization)]

use core::marker::PhantomData;
use core::ffi::*;
use std::ffi::CString;

extern crate playdate_callback as cb;
use cb::arg::Adapter;
use cb::into::*;
use cb::proxy;
use cb::scope;

mod common;
use common::{mock, mock::ty};


mod set_serial_message_callback {
	use std::marker::Tuple;

	use super::*;


	struct Scope;

	impl scope::Scope for Scope {
		type Adapter<In, Out> = Custom<In, Out>;
		type Proxy<C, CIn, COut, R, RIn, ROut> =
			proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
		type Storage<Key> = <scope::Deferred as scope::Scope>::Storage<Key>;
	}


	struct Custom<In, Out>(PhantomData<(In, Out)>);

	// as-is:
	impl<T> Adapter for Custom<(T,), (T,)> {
		type Params = (T,);
		type Args = (T,);
		default fn convert(src: Self::Params) -> Self::Args { src }
	}

	impl<'t> Adapter for Custom<(*const c_char,), (&'t CStr,)> {
		type Params = (*const c_char,);
		type Args = (&'t CStr,);

		fn convert(src: Self::Params) -> Self::Args { (unsafe { CStr::from_ptr(src.0) },) }
	}

	impl Adapter for Custom<(*const c_char,), (CString,)> {
		type Params = (*const c_char,);
		type Args = (CString,);

		fn convert(src: Self::Params) -> Self::Args { (unsafe { CStr::from_ptr(src.0).to_owned() },) }
	}


	fn set_serial_message_callback_fn<Args: Tuple>(cb: Option<
	                                                      impl FnIntoCallback<
		Scope,
		ty::SerialMessageCallback,
		Args,
		(),
	>,
	>) {
		use mock::setSerialMessageCallback;
		setSerialMessageCallback(cb.map(FnIntoCallback::into_callback));
	}

	fn set_serial_message_callback_mut<Args: Tuple>(cb: Option<
	                                                       impl FnMutIntoCallback<
		Scope,
		ty::SerialMessageCallback,
		Args,
		(),
	>,
	>) {
		use mock::setSerialMessageCallback;
		setSerialMessageCallback(cb.map(FnMutIntoCallback::into_callback_mut));
	}


	#[test]
	fn test() {
		set_serial_message_callback_fn(Some(|_: *const c_char| {}));
		set_serial_message_callback_fn(Some(|_: &CStr| {}));
		set_serial_message_callback_fn(Some(|_: CString| {}));

		set_serial_message_callback_mut(Some(|_: *const c_char| {}));
		set_serial_message_callback_mut(Some(|_: &CStr| {}));
		set_serial_message_callback_mut(Some(|_: CString| {}));
	}
}


mod set_update_callback {
	use core::marker::Tuple;
	use cb::util::marker::Ud;
	use cb::util::marker::UdFn;
	use cb::util::marker::UdPtr;

	use super::*;


	struct Scope;

	impl scope::Scope for Scope {
		type Adapter<In, Out> = Custom<In, Out>;
		type Proxy<C, CIn, COut, R, RIn, ROut> =
			proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
		type Storage<Key> = <scope::Deferred as scope::Scope>::Storage<Key>;
	}


	struct Custom<In, Out>(PhantomData<(In, Out)>);

	impl<'t, UD> Adapter for Custom<(Ud<UD>,), ()> {
		type Params = (Ud<UD>,);
		type Args = ();

		default fn convert(_: Self::Params) -> Self::Args { () }
	}

	impl Adapter for Custom<(), ()> {
		type Params = ();
		type Args = ();

		default fn convert(_: Self::Params) -> Self::Args { () }
	}


	impl<'t, F: 't, UD> Adapter for Custom<(Ud<(F, UD)>,), (&'t mut UD,)> {
		type Params = (Ud<UD>,);
		type Args = (&'t mut UD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_mut().unwrap(),) } }
	}
	impl<'t, F: 't, UD> Adapter for Custom<(Ud<(F, UD)>,), (&'t UD,)> {
		type Params = (Ud<UD>,);
		type Args = (&'t UD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_ref().unwrap(),) } }
	}


	impl<'t, UD> Adapter for Custom<(Ud<UD>,), (&'t mut UD,)> {
		type Params = (Ud<UD>,);
		type Args = (&'t mut UD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_mut().unwrap(),) } }
	}
	impl<'t, UD> Adapter for Custom<(Ud<UD>,), (&'t UD,)> {
		type Params = (Ud<UD>,);
		type Args = (&'t UD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_ref().unwrap(),) } }
	}


	fn set_update_callback_fn_ud_wrap<F: 'static, Ud: 'static>(cb: F, mut ud: Ud)
		where F: Fn(&mut Ud) -> c_int {
		use mock::setUpdateCallback;

		let wrap = move || {
			let ud = &mut ud;
			cb(ud)
		};
		let UdFn(f, ud) =
		FnMutIntoCallback::<Scope, UdFn<unsafe extern "C" fn(UdPtr) -> c_int, _, 0>, (), c_int>::into_callback_mut(wrap);
		setUpdateCallback(Some(f), ud.as_udptr());
	}


	#[test]
	fn no_ud() {
		set_update_callback_fn(|| 0);

		let mut state = Vec::<u8>::new();
		set_update_callback_mut(move || {
			state.push(0);
			0
		});


		fn set_update_callback_fn<F, Rargs: Tuple, RR>(cb: F)
			where F: FnIntoCallback<Scope, UdFn<unsafe extern "C" fn(UdPtr) -> c_int, F, 0>, Rargs, RR> {
			use mock::setUpdateCallback;

			let UdFn(f, ud) = cb.into_callback();
			setUpdateCallback(Some(f), ud.as_udptr());
		}


		fn set_update_callback_mut<F, Rargs: Tuple, RR>(cb: F)
			where F: FnMutIntoCallback<Scope, UdFn<unsafe extern "C" fn(UdPtr) -> c_int, F, 0>, Rargs, RR> {
			use mock::setUpdateCallback;

			let UdFn(f, ud) = cb.into_callback_mut();
			setUpdateCallback(Some(f), ud.as_udptr());
		}
	}

	#[test]
	fn ud_wrap() {
		static UD: usize = 42;

		set_update_callback_fn_ud_wrap(|_| 0, UD);
	}

	#[test]
	fn ud_ext() {
		static UD: usize = 42;

		set_update_callback_mut_ud(|_: Option<&mut usize>| 0, UD);
		set_update_callback_mut_ud(|_: Option<&usize>| 0, UD);


		fn set_update_callback_mut_ud<F, Rargs: Tuple, RR, Ud>(cb: F, ud: Ud)
			where F: FnMutIntoCallbackWith<
			                               scope::Deferred,
			                               UdFn<unsafe extern "C" fn(UdPtr) -> c_int, (F, Ud), 0>,
			                               Rargs,
			                               RR,
			                               Ud,
			> {
			use mock::setUpdateCallback;

			let UdFn(f, ud) = cb.into_callback_mut_with(ud);
			setUpdateCallback(Some(f), ud.as_udptr());
		}
	}


	#[test]
	fn ud_ext_custom() {
		static UD: usize = 42;

		set_update_callback_fn_ud_ext(|_: &mut usize| 0, UD);
		set_update_callback_fn_ud_ext(|_: &usize| 0, UD);


		fn set_update_callback_fn_ud_ext<'t, F: 't, Rargs: Tuple, RR, Ud: 't>(cb: F, ud: Ud)
			where F: FnOnceIntoCallbackWith<Scope, UdFn<extern "C" fn(UdPtr) -> c_int, (F, Ud), 0>, Rargs, RR, Ud> {
			use mock::setUpdateCallback;
			let UdFn(f, ud) = cb.into_callback_once_with(ud);
			setUpdateCallback(Some(f), ud.as_udptr());
		}
	}
}
