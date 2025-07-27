#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(feature = "allocator-api", feature(allocator_api))]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(fn_ptr_trait, tuple_trait, unboxed_closures, rustc_attrs, const_type_id)]
#![feature(fn_traits)]
#![allow(internal_features)]
#![allow(uncommon_codepoints)]
#![cfg_attr(test, feature(thread_local))]
#![cfg_attr(any(debug_assertions, feature = "sys"), feature(cfg_select))]

#[cfg(test)]
#[macro_use]
extern crate std;
extern crate alloc;


pub mod scope;
pub mod util;
pub mod arg;
pub mod storage;

pub mod into;
pub mod proxy;
