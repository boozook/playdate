#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]

#[macro_use]
extern crate alloc;
extern crate sys;
pub mod buttons;
pub mod peripherals;
