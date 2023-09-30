use crate::*;
use erased_set::ErasedSendSet;


pub static mut STORE: Option<ErasedSendSet> = None;


pub fn init_store() {
	if unsafe { STORE.is_none() } {
		unsafe { STORE = Some(ErasedSendSet::new()) }
	}
}

pub fn clean_store() {
	if let Some(true) = unsafe { STORE.as_mut() }.map(|store| store.is_empty()) {
		unsafe { STORE = None }
		println!("store cleaned up");
	}
}


pub unsafe extern "C" fn proxy_boards<F: 'static + Send + FnMut(ScoresResult<Boards>)>(boards: *mut PDBoardsList,
                                                                                       error: *const c_char) {
	let res = if boards.is_null() {
		Err(Error::from_ptr(error).expect("unable read err"))
	} else {
		if !error.is_null() {
			let err = Error::from_ptr(error).expect("unable read err");
			sys::println!("Err: {err}");
		}

		Ok(Boards(boards))
	};


	let f = unsafe { STORE.as_mut() }.map(|store| store.remove::<F>())
	                                 .flatten();
	f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

	// cleanup the storage
	clean_store();
}


pub unsafe extern "C" fn proxy_scores<F: 'static + Send + FnMut(ScoresResult<Scores>)>(scores: *mut PDScoresList,
                                                                                       error_message: *const c_char)
{
	let res = if scores.is_null() {
		Err(Error::from_ptr(error_message).expect("unable read err"))
	} else {
		if !error_message.is_null() {
			let err = Error::from_ptr(error_message).expect("unable read err");
			sys::println!("Err: {err}");
		}

		Ok(Scores(scores))
	};

	let f = unsafe { STORE.as_mut() }.map(|store| store.remove::<F>())
	                                 .flatten();
	f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

	// cleanup the storage
	clean_store();
}

pub unsafe extern "C" fn proxy_score<F: 'static + Send + FnMut(ScoresResult<ScoreRef>)>(score: *mut PDScore,
                                                                                        error: *const c_char) {
	let res = if score.is_null() {
		Err(Error::from_ptr(error).expect("unable read err"))
	} else {
		if !error.is_null() {
			let err = Error::from_ptr(error).expect("unable read err");
			sys::println!("Err: {err}");
		}

		Ok(ScoreRef(score))
	};

	let f = unsafe { STORE.as_mut() }.map(|store| store.remove::<F>())
	                                 .flatten();
	f.map(|mut f| f(res)).or_else(|| panic!("missed callback"));

	// cleanup the storage
	clean_store();
}
