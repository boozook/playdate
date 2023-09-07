pub mod allocator;
pub mod panic;
pub mod proc;
pub mod macros;
pub mod error;


/// Main unsafe API endpoint.
pub static mut API: *const crate::ffi::PlaydateAPI = core::ptr::null_mut();

/// Reference to main (root) API endpoint.
pub type ApiRef = Option<&'static crate::ffi::PlaydateAPI>;

/// Returns reference to main API endpoint ([`ApiRef`]).
// TODO: make this `const fn` when rustc feature `const_ptr_as_ref` is well-tested.
#[inline(always)]
pub fn api() -> ApiRef { unsafe { API.as_ref() } }
