#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
#![cfg_attr(test, allow(deref_nullptr))]


/// Preferred `CString` to use.
pub use alloc::ffi::CString;
/// Preferred `CStr` to use.
pub use core::ffi::CStr;

mod dev;
pub use dev::*;
