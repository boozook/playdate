pub(crate) mod update;
pub(crate) mod serial;
pub(crate) mod ntp;
pub(crate) mod btn;


use callback::util::marker::Ud;


/// Type-agnostic callback drop.
struct DropUserdata(*mut (), unsafe fn(*mut ()) -> bool);

impl DropUserdata {
	fn new<T>(ud: Ud<T>) -> Self {
		// coercion, erasing T:
		Self(ud.into_ptr().cast(), Ud::<T>::drop_ptr)
	}

	fn drop(self) -> bool {
		let Self(loc, drop) = self;
		unsafe { drop(loc) }
	}
}


#[cfg(debug_assertions)]
mod __ {
	use core::ffi::c_void;
	use callback::util::marker::UdPtr;

	const _: () = const {
		use core::intrinsics::type_id;
		const _: [(); (type_id::<UdPtr>() - type_id::<*mut c_void>() != 0) as usize] = [(); 0];
	};
}
