//! Playdate Scoreboards API.
//!
//! Wraps C-API.
//! [Official documentation](https://help.play.date/catalog-developer/scoreboard-api/#c-api-reference).

#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate sys;
extern crate alloc;

use core::ffi::c_char;
use core::ffi::c_uint;
use alloc::borrow::Cow;

use sys::ffi::CStr;
use sys::ffi::CString;
use sys::ffi::PDBoard;
use sys::ffi::PDBoardsList;
use sys::ffi::PDScore;
use sys::ffi::PDScoresList;


pub mod error;
mod storage;

use error::*;
use storage::*;


pub type ScoresResult<T> = Result<T, Error>;


#[derive(Debug, Clone, Copy)]
pub struct Scoreboards<Api = api::Default>(Api);

impl Scoreboards<api::Default> {
	/// Creates default [`Scoreboards`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Scoreboards<api::Cache> {
	/// Creates [`Scoreboards`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Scoreboards<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Scoreboards<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Scoreboards<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


#[gen_api_shorthands::gen_shorthands]
impl<Api: api::Api> Scoreboards<Api> {
	/// Requests to add score `value` to the board with given `board_id`.
	///
	/// Safety: read description for [`Scoreboards::get_scoreboards`].
	///
	/// Equivalent to [`sys::ffi::playdate_scoreboards::addScore`].
	#[doc(alias = "sys::ffi::scoreboards::addScore")]
	pub fn add_score<S: AsRef<str>, F: FnMut(ScoresResult<ScoreRef>)>(&self,
	                                                                  board_id: S,
	                                                                  value: u32,
	                                                                  callback: F)
	                                                                  -> Result<Option<F>, ApiError>
		where F: 'static + Send
	{
		let id = CString::new(board_id.as_ref())?;

		init_store();
		let prev = unsafe { STORE.as_mut() }.expect("impossible")
		                                    .insert::<F>(callback);
		let f = self.0.add_score();

		let result = unsafe { f(id.as_ptr() as _, value, Some(proxy_score::<F>)) };

		if result != 0 {
			Err(Error::Unknown.into())
		} else {
			Ok(prev)
		}
	}


	/// Requests user's personal best scores for the given `board`.
	///
	/// Safety: read description for [`Scoreboards::get_scoreboards`].
	///
	/// Equivalent to [`sys::ffi::playdate_scoreboards::getPersonalBest`].
	#[doc(alias = "sys::ffi::scoreboards::getPersonalBest")]
	pub fn get_personal_best_for<F: FnMut(ScoresResult<ScoreRef>)>(&self,
	                                                               board: &Board,
	                                                               callback: F)
	                                                               -> Result<Option<F>, ApiError>
		where F: 'static + Send
	{
		self.get_personal_best(board.id().expect("board.id"), callback)
	}

	/// Requests user's personal best scores for the given `board_id`.
	///
	/// Safety: read description for [`Scoreboards::get_scoreboards`].
	///
	/// Equivalent to [`sys::ffi::playdate_scoreboards::getPersonalBest`].
	#[doc(alias = "sys::ffi::scoreboards::getPersonalBest")]
	pub fn get_personal_best<S: AsRef<str>, F: FnMut(ScoresResult<ScoreRef>)>(&self,
	                                                                          board_id: S,
	                                                                          callback: F)
	                                                                          -> Result<Option<F>, ApiError>
		where F: 'static + Send
	{
		let id = CString::new(board_id.as_ref())?;

		init_store();
		let prev = unsafe { STORE.as_mut() }.expect("impossible")
		                                    .insert::<F>(callback);
		let f = self.0.get_personal_best();

		let result = unsafe { f(id.as_ptr() as _, Some(proxy_score::<F>)) };

		if result != 0 {
			Err(Error::Unknown.into())
		} else {
			Ok(prev)
		}
	}


	/// Requests scores list [`Scores`] for the given `board_id`.
	///
	/// Safety: read description for [`Scoreboards::get_scoreboards`].
	///
	/// Equivalent to [`sys::ffi::playdate_scoreboards::getScores`].
	#[doc(alias = "sys::ffi::scoreboards::getScores")]
	pub fn get_scores<S: AsRef<str>, F: FnMut(ScoresResult<Scores>)>(&self,
	                                                                 board_id: S,
	                                                                 callback: F)
	                                                                 -> Result<Option<F>, ApiError>
		where F: 'static + Send
	{
		let id = CString::new(board_id.as_ref())?;

		init_store();
		let prev = unsafe { STORE.as_mut() }.expect("impossible")
		                                    .insert::<F>(callback);
		let f = self.0.get_scores();

		let result = unsafe { f(id.as_ptr() as _, Some(proxy_scores::<F>)) };

		if result != 0 {
			Err(Error::Unknown.into())
		} else {
			Ok(prev)
		}
	}


	/// Requests boards list [`Boards`] for the given `board_id`.
	///
	/// Returns previous callback `F` if it exists, so it was overwritten.
	/// Usually, it's not possible fo closures because until it's type is not erased.
	/// Anyway if it happened, we just override it with new one, given as `callback`,
	/// so responses will be passed to the new callback.
	///
	/// Equivalent to [`sys::ffi::playdate_scoreboards::getScoreboards`].
	#[doc(alias = "sys::ffi::scoreboards::getScoreboards")]
	pub fn get_scoreboards<F: FnMut(ScoresResult<Boards>)>(&self, callback: F) -> Option<F>
		where F: 'static + Send {
		init_store();
		let prev = unsafe { STORE.as_mut() }.expect("impossible")
		                                    .insert::<F>(callback);
		let f = self.0.get_scoreboards();
		unsafe { f(Some(proxy_boards::<F>)) };

		prev
	}
}


pub struct Boards(*mut PDBoardsList);

impl core::fmt::Debug for Boards {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut t = f.debug_tuple("Boards");
		self.boards().into_iter().for_each(|board| {
			                         t.field(board);
		                         });
		t.finish()
	}
}

impl Boards {
	pub fn last_updated(&self) -> u32 { unsafe { (*self.0).lastUpdated } }

	pub fn boards(&self) -> &[Board] {
		let count = unsafe { (*self.0).count };
		let ptr = unsafe { (*self.0).boards };
		let slice = unsafe { core::slice::from_raw_parts(ptr, count as _) };
		unsafe { core::mem::transmute(slice) }
	}

	pub fn boards_mut(&mut self) -> &mut [Board] {
		let count = unsafe { (*self.0).count };
		let ptr = unsafe { (*self.0).boards };
		let slice = unsafe { core::slice::from_raw_parts_mut(ptr, count as _) };
		unsafe { core::mem::transmute(slice) }
	}
}


#[repr(transparent)]
pub struct Board(PDBoard);

impl core::fmt::Debug for Board {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Board")
		 .field("id", &self.id())
		 .field("name", &self.name())
		 .finish()
	}
}

impl Board {
	pub fn id<'s>(&'s self) -> Option<Cow<'s, str>> {
		let ptr = self.0.boardID;
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_string_lossy().into()
		}
	}

	pub fn name<'s>(&'s self) -> Option<Cow<'s, str>> {
		let ptr = self.0.name;
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_string_lossy().into()
		}
	}
}

impl Drop for Boards {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let get_fn = || sys::api_opt!(scoreboards.freeBoardsList);
			if let Some(f) = get_fn() {
				unsafe { f(self.0) }
			}
		}
	}
}


pub struct Scores(*mut PDScoresList);

impl core::fmt::Debug for Scores {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Scores")
		 .field("id", &self.id())
		 .field("count", &self.len())
		 .field("capacity", &self.capacity())
		 .field("last_updated", &self.last_updated())
		 .field("playerIncluded", &self.player_included())
		 .finish()
	}
}

impl Drop for Scores {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let get_fn = || sys::api_opt!(scoreboards.freeScoresList);
			if let Some(f) = get_fn() {
				unsafe { f(self.0) }
			}
		}
	}
}

impl Scores {
	/// ID of associated board.
	pub fn id<'s>(&'s self) -> Option<Cow<'s, str>> {
		let ptr = unsafe { (*self.0).boardID };
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_string_lossy().into()
		}
	}

	pub fn last_updated(&self) -> u32 { unsafe { (*self.0).lastUpdated } }
	pub fn player_included(&self) -> bool { unsafe { (*self.0).playerIncluded == 1 } }

	pub fn len(&self) -> c_uint { unsafe { (*self.0).count } }
	pub fn capacity(&self) -> c_uint { unsafe { (*self.0).limit } }

	pub fn scores(&self) -> &[Score] {
		let count = self.len();
		let ptr = unsafe { (*self.0).scores };
		let slice = unsafe { core::slice::from_raw_parts(ptr, count as _) };
		unsafe { core::mem::transmute(slice) }
	}
}


#[repr(transparent)]
pub struct Score(PDScore);

impl core::fmt::Debug for Score {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Score")
		 .field("rank", &self.rank())
		 .field("value", &self.value())
		 .field("player", &self.player())
		 .finish()
	}
}

impl Score {
	pub fn rank(&self) -> u32 { self.0.rank }
	pub fn value(&self) -> u32 { self.0.value }

	pub fn player<'s>(&'s self) -> Option<Cow<'s, str>> {
		let ptr = self.0.player;
		if ptr.is_null() {
			None
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_string_lossy().into()
		}
	}
}

#[repr(transparent)]
pub struct ScoreRef(*mut PDScore);

impl Drop for ScoreRef {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let get_fn = || sys::api_opt!(scoreboards.freeScore);
			if let Some(f) = get_fn() {
				unsafe { f(self.0) }
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use core::mem::size_of;


	#[test]
	fn board_size() {
		assert_eq!(size_of::<Board>(), size_of::<PDBoard>());
	}

	#[test]
	fn score_size() {
		assert_eq!(size_of::<Score>(), size_of::<PDScore>());
	}

	#[test]
	fn score_ref_size() {
		assert_eq!(size_of::<ScoreRef>(), size_of::<*mut PDScore>());
	}
}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_int;
	use core::ptr::NonNull;

	use sys::ffi::AddScoreCallback;
	use sys::ffi::PDBoardsList;
	use sys::ffi::PDScore;
	use sys::ffi::PDScoresList;
	use sys::ffi::ScoresCallback;
	use sys::ffi::BoardsListCallback;
	use sys::ffi::PersonalBestCallback;
	use sys::ffi::playdate_scoreboards;


	/// Default scoreboards api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached scoreboards api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_scoreboards);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(scoreboards)) }
	}

	impl From<*const playdate_scoreboards> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_scoreboards) -> Self { Self(unsafe { ptr.as_ref() }.expect("scoreboards")) }
	}

	impl From<&'static playdate_scoreboards> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_scoreboards) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_scoreboards>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_scoreboards>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_scoreboards>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_scoreboards>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl Api for Cache {
		fn add_score(
			&self)
			-> unsafe extern "C" fn(boardId: *const c_char, value: u32, callback: AddScoreCallback) -> c_int {
			self.0.addScore.expect("addScore")
		}

		fn get_personal_best(
			&self)
			-> unsafe extern "C" fn(boardId: *const c_char, callback: PersonalBestCallback) -> c_int {
			self.0.getPersonalBest.expect("getPersonalBest")
		}

		fn free_score(&self) -> unsafe extern "C" fn(score: *mut PDScore) { self.0.freeScore.expect("freeScore") }

		fn get_scoreboards(&self) -> unsafe extern "C" fn(callback: BoardsListCallback) -> c_int {
			self.0.getScoreboards.expect("getScoreboards")
		}

		fn free_boards_list(&self) -> unsafe extern "C" fn(boardsList: *mut PDBoardsList) {
			self.0.freeBoardsList.expect("freeBoardsList")
		}

		fn get_scores(&self) -> unsafe extern "C" fn(board_id: *const c_char, callback: ScoresCallback) -> c_int {
			self.0.getScores.expect("getScores")
		}

		fn free_scores_list(&self) -> unsafe extern "C" fn(scores_list: *mut PDScoresList) {
			self.0.freeScoresList.expect("freeScoresList")
		}
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_scoreboards::addScore`]
		#[doc(alias = "sys::ffi::scoreboards::addScore")]
		#[inline(always)]
		fn add_score(
			&self)
			-> unsafe extern "C" fn(boardId: *const c_char, value: u32, callback: AddScoreCallback) -> c_int {
			*sys::api!(scoreboards.addScore)
		}

		/// Returns [`sys::ffi::playdate_scoreboards::getPersonalBest`]
		#[doc(alias = "sys::ffi::scoreboards::getPersonalBest")]
		#[inline(always)]
		fn get_personal_best(
			&self)
			-> unsafe extern "C" fn(boardId: *const c_char, callback: PersonalBestCallback) -> c_int {
			*sys::api!(scoreboards.getPersonalBest)
		}

		/// Returns [`sys::ffi::playdate_scoreboards::freeScore`]
		#[doc(alias = "sys::ffi::scoreboards::freeScore")]
		#[inline(always)]
		fn free_score(&self) -> unsafe extern "C" fn(score: *mut PDScore) { *sys::api!(scoreboards.freeScore) }

		/// Returns [`sys::ffi::playdate_scoreboards::getScoreboards`]
		#[doc(alias = "sys::ffi::scoreboards::getScoreboards")]
		#[inline(always)]
		fn get_scoreboards(&self) -> unsafe extern "C" fn(callback: BoardsListCallback) -> c_int {
			*sys::api!(scoreboards.getScoreboards)
		}

		/// Returns [`sys::ffi::playdate_scoreboards::freeBoardsList`]
		#[doc(alias = "sys::ffi::scoreboards::freeBoardsList")]
		#[inline(always)]
		fn free_boards_list(&self) -> unsafe extern "C" fn(boardsList: *mut PDBoardsList) {
			*sys::api!(scoreboards.freeBoardsList)
		}

		/// Returns [`sys::ffi::playdate_scoreboards::getScores`]
		#[doc(alias = "sys::ffi::scoreboards::getScores")]
		#[inline(always)]
		fn get_scores(&self) -> unsafe extern "C" fn(board_id: *const c_char, callback: ScoresCallback) -> c_int {
			*sys::api!(scoreboards.getScores)
		}

		/// Returns [`sys::ffi::playdate_scoreboards::freeScoresList`]
		#[doc(alias = "sys::ffi::scoreboards::freeScoresList")]
		#[inline(always)]
		fn free_scores_list(&self) -> unsafe extern "C" fn(scores_list: *mut PDScoresList) {
			*sys::api!(scoreboards.freeScoresList)
		}
	}
}
