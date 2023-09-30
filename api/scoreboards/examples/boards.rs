#![no_std]
extern crate alloc;
use core::ptr::NonNull;

#[macro_use]
extern crate sys;
extern crate playdate_scoreboards as scoreboards;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::prelude::*;

use scoreboards::ScoresResult;
use scoreboards::Scoreboards;


/// Entry point
#[no_mangle]
fn event_handler(_: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if !matches!(event, SystemEvent::Init) {
		return EventLoopCtrl::Continue;
	}

	const BOARD_ID: &str = "ID101";

	let scoreboards = Scoreboards::Cached();

	let res = scoreboards.add_score(BOARD_ID, 42, |res| {
		                     println!("Add score callback");
		                     match res {
			                     Ok(_) => println!("scores added"),
		                        Err(err) => println!("{err}"),
		                     }
	                     });
	match res {
		Ok(_) => println!("add_score res: F"),
		Err(err) => println!("add_score res: ERR: {err}"),
	}


	scoreboards.get_scoreboards(|boards| {
		           println!("1: Get boards callback");
		           println!("{boards:?}");
	           });
	scoreboards.get_scoreboards(|boards| {
		           println!("2: Get boards callback");
		           println!("{boards:?}");
	           });


	fn get_scores(scores: ScoresResult<scoreboards::Scores>) {
		println!("1: Get scores callback");
		println!("{scores:?}");
	}

	scoreboards.get_scores(BOARD_ID, get_scores).ok();
	scoreboards.get_scores(BOARD_ID, |res| {
		           println!("2: Get scores callback");
		           println!("{res:?}");
	           })
	           .ok();


	scoreboards.get_personal_best(BOARD_ID, |res| {
		           println!("Get personal best callback");
		           match res {
			           Ok(_) => todo!("scores received"),
		              Err(err) => println!("{err}"),
		           }
	           })
	           .ok();


	// Set no-op update callback
	system::System::Default().set_update_callback_boxed(|_| UpdateCtrl::Continue, ());

	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
