use core::ffi::*;
use core::ptr::null_mut;
use core::marker::{PhantomData, Tuple};
use callback::{arg, proxy, scope};
use callback::into::FnMutIntoCallback;
use callback::into::FnMutIntoCallbackWith;
use callback::util::marker::{Ud, UdFn};
use sys::ctrl::UpdateDisplayCtrl;

use crate::{System, Api};

use super::DropUserdata;


impl System {
	pub const fn update(&self) -> Update { Update(self.0) }
}


/// Ctrl system update loop / callback-subscription.
pub struct Update(Api);


struct Scope;

impl scope::Scope for Scope {
	type Adapter<In, Out> = Adapter<In, Out>;
	type Proxy<C, Params, Res, R, Args, Out> =
		proxy::default::Default<R, Self::Storage<R>, Self::Adapter<Params, Args>>;
	type Storage<Key> = ();
}


struct Adapter<In, Out>(PhantomData<(In, Out)>);

impl<T> arg::Adapter for Adapter<(Ud<T>,), ()> {
	type Params = (Ud<T>,);
	type Args = ();
	#[inline(always)]
	fn convert(_: Self::Params) -> Self::Args {}
}


impl<'t, F: 't, UD> arg::Adapter for Adapter<(Ud<(F, UD)>,), (&'t mut UD,)> {
	type Params = (Ud<UD>,);
	type Args = (&'t mut UD,);
	#[inline]
	fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_mut().unwrap(),) } }
}
impl<'t, F: 't, UD> arg::Adapter for Adapter<(Ud<(F, UD)>,), (&'t UD,)> {
	type Params = (Ud<UD>,);
	type Args = (&'t UD,);
	#[inline]
	fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_ref().unwrap(),) } }
}


impl<'t, UD> arg::Adapter for Adapter<(Ud<UD>,), (&'t mut UD,)> {
	type Params = (Ud<UD>,);
	type Args = (&'t mut UD,);
	#[cold]
	#[inline]
	fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_mut().unwrap(),) } }
}
impl<'t, UD> arg::Adapter for Adapter<(Ud<UD>,), (&'t UD,)> {
	type Params = (Ud<UD>,);
	type Args = (&'t UD,);
	#[cold]
	#[inline]
	fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<UD>().as_ref().unwrap(),) } }
}


type Cfn = unsafe extern "C" fn(ud: *mut c_void) -> c_int;

type CCb<F> = UdFn<Cfn, F, 0>;
type CCbExt<F, Ud> = UdFn<Cfn, (F, Ud), 0>;


static mut CB_DROP: Option<DropUserdata> = None;


impl Update {
	/// Sets `callback` to [`sys::ffi::PlaydateSys::setUpdateCallback`].
	///
	/// Acceptable options of signature of the `callback` can be
	/// `() -> Output`
	/// where `Output` is anything that impls `Into<UpdateDisplayCtrl>` such as any `uint` or `int`.
	#[doc(alias = "sys::ffi::PlaydateSys::setUpdateCallback")]
	#[allow(private_bounds)]
	pub fn set<Args: Tuple, R, F>(&self, callback: F)
		where F: FnMutIntoCallback<Scope, CCb<F>, Args, R>,
		      R: Into<UpdateDisplayCtrl> {
		let c = self.0.setUpdateCallback;
		let UdFn(f, ud) = callback.into_callback_mut();
		// set to api:
		unsafe { c(Some(f), ud.as_udptr()) };
		// set dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.replace(DropUserdata::new(ud)) }.map(DropUserdata::drop);
	}


	/// Sets `callback` to [`sys::ffi::PlaydateSys::setUpdateCallback`]
	///                                    with given extra `userdata`.
	///
	/// Acceptable options of signature of the `callback` can be
	/// - `(&userdata) -> Output`
	/// - `(&mut userdata) -> Output`\
	///   where `Output` is anything that impls `Into<UpdateDisplayCtrl>` such as any `uint` or `int`.
	#[doc(alias = "sys::ffi::PlaydateSys::setUpdateCallback")]
	#[allow(private_bounds)]
	pub fn set_with<Args: Tuple, R, F, Ud>(&self, callback: F, userdata: Ud)
		where F: FnMutIntoCallbackWith<Scope, CCbExt<F, Ud>, Args, R, Ud>,
		      R: Into<UpdateDisplayCtrl> {
		let c = self.0.setUpdateCallback;
		let UdFn(f, ud) = callback.into_callback_mut_with(userdata);
		// set to api:
		unsafe { c(Some(f), ud.as_udptr()) };
		// set dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.replace(DropUserdata::new(ud)) }.map(DropUserdata::drop);
	}


	/// Sets `None` to [`sys::ffi::PlaydateSys::setUpdateCallback`], drops previously set callback.
	pub fn unset(&self) {
		let c = self.0.setUpdateCallback;
		unsafe { c(None, null_mut()) };

		// take dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.take() }.map(DropUserdata::drop);
	}
}
