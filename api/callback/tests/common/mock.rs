#![allow(dead_code, unused_variables)]
#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_int;

use self::ty::*;

pub mod ty {
	pub use core::ffi::*;


	#[repr(C)]
	pub(crate) struct PDButtons(usize);
	#[repr(C)]
	pub(crate) struct PDRect(usize);
	pub(crate) struct LCDSprite {}
	pub(crate) enum SpriteCollisionResponseType {}
	pub(crate) struct FilePlayer {}
	pub(crate) struct SoundSource {}
	pub(crate) type MIDINote = core::ffi::c_float;
	pub(crate) struct PDScore {}
	pub(crate) struct PDBoardsList {}
	pub(crate) struct PDScoresList {}

	// Statefull
	pub type PDCallbackFunction = unsafe extern "C" fn(userdata: *mut c_void) -> c_int;
	pub type PDMenuItemCallbackFunction = unsafe extern "C" fn(userdata: *mut c_void);
	pub type PDButtonCallbackFunction =
		unsafe extern "C" fn(button: PDButtons, down: c_int, when: u32, userdata: *mut c_void) -> c_int;
	pub type ListFilesCallback = unsafe extern "C" fn(path: *const c_char, userdata: *mut c_void);
	pub type LCDSpriteDrawFunction =
		unsafe extern "C" fn(sprite: *mut LCDSprite, bounds: PDRect, drawrect: PDRect);
	pub type LCDSpriteUpdateFunction = unsafe extern "C" fn(sprite: *mut LCDSprite);
	pub type LCDSpriteCollisionFilterProc =
		unsafe extern "C" fn(sprite: *mut LCDSprite, other: *mut LCDSprite) -> SpriteCollisionResponseType;
	pub type sndCallbackProc = unsafe extern "C" fn(c: *mut SoundSource, userdata: *mut c_void);
	pub type signalStepFunc =
		unsafe extern "C" fn(userdata: *mut c_void, ioframes: *mut c_int, ifval: *mut c_float) -> c_float;
	pub type signalNoteOnFunc =
		unsafe extern "C" fn(userdata: *mut c_void, note: MIDINote, vel: c_float, len: c_float);
	pub type signalNoteOffFunc = unsafe extern "C" fn(userdata: *mut c_void, stop: c_int, offset: c_int);
	pub type signalDeallocFunc = unsafe extern "C" fn(userdata: *mut c_void);

	// Stateless
	pub type SerialMessageCallback = unsafe extern "C" fn(data: *const c_char);


	// Once, Stateless
	pub type AddScoreCallback = unsafe extern "C" fn(score: *mut PDScore, errorMessage: *const c_char);
	pub type PersonalBestCallback = unsafe extern "C" fn(score: *mut PDScore, errorMessage: *const c_char);
	pub type BoardsListCallback = unsafe extern "C" fn(boards: *mut PDBoardsList, errorMessage: *const c_char);
	pub type ScoresCallback = unsafe extern "C" fn(scores: *mut PDScoresList, errorMessage: *const c_char);
}


// Statefull
pub extern "C" fn setUpdateCallback(cb: Option<PDCallbackFunction>, ud: *mut c_void) {
	cb.map(|f| unsafe { f(ud) });
}
pub extern "C" fn addMenuItem(cb: Option<PDMenuItemCallbackFunction>, ud: *mut c_void) {}
pub extern "C" fn setButtonCallback(cb: Option<PDButtonCallbackFunction>, ud: *mut c_void) {}

// Stateless
pub extern "C" fn setSerialMessageCallback(cb: Option<SerialMessageCallback>) {}

// Statefull
pub extern "C" fn listfiles(cb: ListFilesCallback, ud: *mut c_void) {}


// Stateless (for sprite)
pub extern "C" fn setUpdateFunction(sprite: *mut LCDSprite, func: Option<LCDSpriteUpdateFunction>) {}
pub extern "C" fn setDrawFunction(sprite: *mut LCDSprite, func: Option<LCDSpriteDrawFunction>) {}

// Stateless (for sprite) 🤷🏻‍♂️
pub extern "C" fn setCollisionResponseFunction(sprite: *mut LCDSprite,
                                               func: Option<LCDSpriteCollisionFilterProc>) {
}


// Statefull
pub extern "C" fn setFinishCallback(player: *mut FilePlayer, callback: sndCallbackProc, userdata: *mut c_void) {}
pub extern "C" fn setLoopCallback(player: *mut FilePlayer, callback: sndCallbackProc, userdata: *mut c_void) {}

pub extern "C" fn newSignal(step: Option<signalStepFunc>,
                            noteOn: Option<signalNoteOnFunc>,
                            noteOff: Option<signalNoteOffFunc>,
                            dealloc: Option<signalDeallocFunc>,
                            userdata: *mut c_void) {
}


// scoreboards-?
pub extern "C" fn addScore(boardId: *const c_char, value: u32, callback: Option<AddScoreCallback>) -> c_int {
	todo!()
}
pub extern "C" fn getPersonalBest(boardId: *const c_char, callback: Option<PersonalBestCallback>) -> c_int {
	todo!()
}
pub extern "C" fn getScoreboards(callback: Option<BoardsListCallback>) -> c_int { todo!() }
pub extern "C" fn getScores(boardId: *const c_char, callback: Option<ScoresCallback>) -> c_int { todo!() }
