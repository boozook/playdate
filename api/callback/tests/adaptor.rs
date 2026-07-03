#![feature(tuple_trait)]


use std::ffi::*;
use std::marker::PhantomData;
use std::marker::Tuple;

extern crate playdate_callback as cb;
use cb::arg::Adapter;
use cb::into::FnOnceIntoCallback;
use cb::{proxy, scope};


mod basic {
	use super::*;


	struct Custom<In, Out>(PhantomData<(In, Out)>);

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


	#[test]
	fn convert() {
		let s = c"test";

		let (res,) = Custom::<_, (CString,)>::convert((s.as_ptr(),));
		assert_eq!(s, res.as_c_str());

		let (res,) = Custom::<_, (&CStr,)>::convert((s.as_ptr(),));
		assert_eq!(s, res);
	}


	#[test]
	fn usage() {
		struct Exec;

		impl scope::Scope for Exec {
			type Adapter<In, Out> = Custom<In, Out>;
			type Proxy<C, CIn, COut, R, RIn, ROut> =
				proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
			type Storage<Key> = <scope::Deferred as scope::Scope>::Storage<Key>;
		}


		type CFn = unsafe extern "C" fn(*const c_char);
		fn accept_once<Args: Tuple, R>(f: impl FnOnceIntoCallback<Exec, CFn, Args, R>) {
			let c = f.into_callback_once();
			unsafe { c(c"test".as_ptr()) };
		}


		accept_once(|s: &CStr| assert_eq!(c"test", s));
		accept_once(|s: CString| assert_eq!(c"test", s.as_c_str()));
	}
}


mod inject {
	use super::*;


	struct Injector<In, Out>(PhantomData<(In, Out)>);

	impl<In: Tuple> Adapter for Injector<In, (Foo,)> {
		type Params = In;
		type Args = (Foo,);

		fn convert(_: Self::Params) -> Self::Args { (Foo(V),) }
	}

	#[derive(Debug, PartialEq, Eq)]
	struct Foo(usize);

	const V: usize = 42;


	#[test]
	fn convert() {
		let s = c"test";

		assert_eq!((Foo(V),), Injector::convert((s.as_ptr(),)));
		assert_eq!((Foo(V),), Injector::convert((s, s.as_ptr())));
		assert_eq!((Foo(V),), Injector::convert((s, s.as_ptr(), V)));
	}


	#[test]
	fn usage() {
		struct Exec;

		impl scope::Scope for Exec {
			type Adapter<In, Out> = Injector<In, Out>;
			type Proxy<C, CIn, COut, R, RIn, ROut> =
				proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
			type Storage<Key> = <scope::Deferred as scope::Scope>::Storage<Key>;
		}


		type CFn = unsafe extern "C" fn(*const c_char);
		fn accept_fn<Args: Tuple, R>(f: impl FnOnceIntoCallback<Exec, CFn, Args, R>) {
			let c = f.into_callback_once();
			unsafe { c(c"test".as_ptr()) };
		}

		accept_fn(|foo| assert_eq!(Foo(V), foo));
		accept_fn(|foo| assert_eq!(Foo(V), foo));
	}
}
