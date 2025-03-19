#![feature(io_error_more)]
#![feature(slice_concat_trait)]
#![cfg_attr(test, feature(assert_matches))]

#[macro_use]
extern crate log;

pub use utils::*;


pub mod fs;
pub mod layout;
pub mod config;
pub mod assets;
pub mod metadata;
pub mod manifest;
