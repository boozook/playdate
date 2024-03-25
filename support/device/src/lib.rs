#![allow(trivial_bounds)]
#![feature(trivial_bounds)]
#![feature(error_generic_member_access)]
#![feature(exit_status_error)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate log;

pub extern crate serialport;
pub extern crate utils;

pub mod error;
pub mod serial;
pub mod usb;
pub mod device;
pub mod mount;

pub mod send;
pub mod install;
pub mod run;

pub mod interface;


pub const VENDOR_ID: u16 = 0x1331;
pub const PRODUCT_ID_DATA: u16 = 0x5740;
pub const PRODUCT_ID_STORAGE: u16 = 0x5741;
