use core::ffi::*;
use core::marker::{PhantomData, Tuple};
use core::ptr::null_mut;
use callback::util::marker::{Ud, UdFn, UdPtr};
use callback::{arg, proxy, scope};
use callback::into::{FnMutIntoCallback, FnMutIntoCallbackWith};
use sys::ffi::Buttons as FfiButtons;

use crate::ctrl::api::buttons::Buttons;
use crate::time::Milliseconds;

use super::DropUserdata;


/// Represents the result of a button callback specified using [`Buttons::set_callback`].
/// The value represents and signals to the system what to do with the event after it has been processed by the callback -
/// should it be removed from the queue or not.
#[repr(u8)]
pub enum ButtonQueueResult {
	/// Means that the callback was successfully processed a button event and that event should be removed from the queue.
	Captured = 0,
	/// Means that the callback was not processed (or with error) a button event and that event should be kept in the queue.
	Nope = 1,
}

impl From<ButtonQueueResult> for c_int {
	fn from(v: ButtonQueueResult) -> Self { v as c_int }
}

impl From<ButtonQueueResult> for c_uint {
	fn from(v: ButtonQueueResult) -> Self { v as c_uint }
}


impl Buttons {
	/// Sets `callback` to [`sys::ffi::PlaydateSys::setButtonCallback`].
	///
	/// See also [`Buttons::set_callback_with`] and [`Buttons::unset_callback`].
	#[doc(alias = "sys::ffi::PlaydateSys::setButtonCallback")]
	#[allow(private_bounds)]
	pub fn set_callback<Args: Tuple, F>(&self, callback: F, queue_size: c_int)
		where F: FnMutIntoCallback<Scope, CCb<F>, Args, ButtonQueueResult> {
		let c = self.0.setButtonCallback;
		let UdFn(f, ud) = callback.into_callback_mut();
		// set to api:
		unsafe { c(Some(f), ud.as_udptr(), queue_size) };
		// set dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.replace(DropUserdata::new(ud)) }.map(DropUserdata::drop);
	}

	/// Sets `callback` to [`sys::ffi::PlaydateSys::setButtonCallback`]
	///                                    with given extra `userdata`.
	///
	/// See also [`Buttons::set_callback`] and [`Buttons::unset_callback`].
	#[doc(alias = "sys::ffi::PlaydateSys::setButtonCallback")]
	#[allow(private_bounds)]
	pub fn set_callback_with<Args: Tuple, F, Ud>(&self, callback: F, userdata: Ud, queue_size: c_int)
		where F: FnMutIntoCallbackWith<Scope, CCbExt<F, Ud>, Args, ButtonQueueResult, Ud> {
		let c = self.0.setButtonCallback;
		let UdFn(f, ud) = callback.into_callback_mut_with(userdata);
		// set to api:
		unsafe { c(Some(f), ud.as_udptr(), queue_size) };
		// set dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.replace(DropUserdata::new(ud)) }.map(DropUserdata::drop);
	}


	/// Sets `None` to [`sys::ffi::PlaydateSys::setButtonCallback`], drops previously set callback.
	pub fn unset_callback(&self) {
		let c = self.0.setButtonCallback;
		unsafe { c(None, null_mut(), 0) };
		Self::clean_callback();
	}


	fn clean_callback() -> bool {
		// take dropper:
		#[allow(static_mut_refs)]
		unsafe { CB_DROP.take() }.map(DropUserdata::drop)
		                         .unwrap_or_default()
	}
}


static mut CB_DROP: Option<DropUserdata> = None;


struct Scope;

impl scope::Scope for Scope {
	type Adapter<In, Out> = Adapter<In, Out>;
	type Proxy<C, Params, Res, R, Args, Out> =
		proxy::default::Default<R, Self::Storage<R>, Self::Adapter<Params, Args>>;
	type Storage<Key> = ();
}


type Cfn = unsafe extern "C" fn(btn: FfiButtons, down: c_int, when: u32, ud: UdPtr) -> c_int;

type CCb<F> = UdFn<Cfn, F, 3>;
type CCbExt<F, Ud> = UdFn<Cfn, (F, Ud), 3>;


pub(crate) struct Adapter<In, Out>(PhantomData<(In, Out)>);


// base:
type InBase = (FfiButtons, c_int, u32);
impl arg::Adapter for Adapter<InBase, (FfiButtons, bool, Milliseconds)> {
	type Params = InBase;
	type Args = (FfiButtons, bool, Milliseconds);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let (btns, down, when) = src;
		(btns, down != 0, Milliseconds::new(when))
	}
}

// without Ud:
impl<T, Out: Tuple> arg::Adapter for Adapter<(FfiButtons, c_int, u32, Ud<T>), Out>
	where Adapter<InBase, Out>: arg::Adapter<Params = InBase, Args = Out>
{
	type Params = (FfiButtons, c_int, u32, Ud<T>);
	type Args = Out;

	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let (btn, down, when, ..) = src;
		Adapter::<InBase, Out>::convert((btn, down, when))
	}
}

// with Ud:
impl<'t, A, B, C, UD> arg::Adapter for Adapter<(FfiButtons, c_int, u32, Ud<UD>), (A, B, C, &'t mut UD)>
	where Adapter<InBase, (A, B, C)>: arg::Adapter<Params = InBase, Args = (A, B, C)>
{
	type Params = (FfiButtons, c_int, u32, Ud<UD>);
	type Args = (A, B, C, &'t mut UD);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let (btn, down, when, ud) = src;
		let (btn, down, when) = Adapter::<InBase, _>::convert((btn, down, when));
		(btn, down, when, unsafe { ud.into_ptr().as_mut().unwrap() })
	}
}
impl<'t, A, B, C, UD> arg::Adapter for Adapter<(FfiButtons, c_int, u32, Ud<UD>), (A, B, C, &'t UD)>
	where Adapter<InBase, (A, B, C)>: arg::Adapter<Params = InBase, Args = (A, B, C)>
{
	type Params = (FfiButtons, c_int, u32, Ud<UD>);
	type Args = (A, B, C, &'t UD);
	#[cold]
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let (btn, down, when, ud) = src;
		let (btn, down, when) = Adapter::<InBase, _>::convert((btn, down, when));
		(btn, down, when, unsafe { ud.into_ptr().as_ref().unwrap() })
	}
}


// special with Ud<F+Ext>:
impl<'t, F: 't, Ac, Bc, Dc, UD, A, B, C> arg::Adapter
	for Adapter<(Ac, Bc, Dc, Ud<(F, UD)>), (A, B, C, &'t mut UD)>
	where Adapter<(Ac, Bc, Dc, Ud<UD>), (A, B, C, &'t mut UD)>:
		      arg::Adapter<Params = (Ac, Bc, Dc, Ud<UD>), Args = (A, B, C, &'t mut UD)>
{
	type Params = (Ac, Bc, Dc, Ud<UD>);
	type Args = (A, B, C, &'t mut UD);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		Adapter::<(Ac, Bc, Dc, Ud<UD>), (A, B, C, &'t mut UD)>::convert(src)
	}
}
impl<'t, F: 't, Ac, Bc, Dc, UD, A, B, C> arg::Adapter for Adapter<(Ac, Bc, Dc, Ud<(F, UD)>), (A, B, C, &'t UD)>
	where Adapter<(Ac, Bc, Dc, Ud<UD>), (A, B, C, &'t UD)>:
		      arg::Adapter<Params = (Ac, Bc, Dc, Ud<UD>), Args = (A, B, C, &'t UD)>
{
	type Params = (Ac, Bc, Dc, Ud<UD>);
	type Args = (A, B, C, &'t UD);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		Adapter::<(Ac, Bc, Dc, Ud<UD>), (A, B, C, &'t UD)>::convert(src)
	}
}
