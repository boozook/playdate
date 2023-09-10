#![no_std]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_menu as menu;

use core::ffi::*;
use alloc::boxed::Box;

use sys::ffi::*;
use menu::*;
use menu::api::*;


const INITIAL_X: u32 = LCD_COLUMNS / 2;
const INITIAL_Y: u32 = (LCD_ROWS - TEXT_HEIGHT) / 2;
const TEXT_HEIGHT: u32 = 16;


const MAX_CLICKS: u32 = 3;


#[derive(Debug, Clone, Copy)]
pub struct CustomApi(&'static playdate_sys);

impl CustomApi {
	fn new() -> Self { Self(sys::api!(system)) }
}

impl<'t> Api for CustomApi {
	fn get_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> c_int {
		self.0.getMenuItemValue.unwrap()
	}

	fn set_menu_item_value(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: c_int) {
		self.0.setMenuItemValue.unwrap()
	}

	fn get_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char {
		self.0.getMenuItemTitle.unwrap()
	}

	fn set_menu_item_title(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char) {
		self.0.setMenuItemTitle.unwrap()
	}

	fn get_menu_item_userdata(&self) -> unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void {
		self.0.getMenuItemUserdata.unwrap()
	}
}


/// App state
struct State {
	first: Option<SimpleMenuItem<u32, CustomApi>>,
	second: Option<CheckMenuItem<Option<&'static Self>, CustomApi>>,
	third: Option<OptionsMenuItem<(), CustomApi>>,
}

impl State {
	const fn new() -> Self {
		Self { first: None,
		       second: None,
		       third: None }
	}


	/// Updates the state
	fn update(&mut self) -> Option<()> {
		// remove first menu item if limit reached
		if let Some((_item, value)) = self.first
		                                  .as_ref()
		                                  .map(|item| item.get_userdata().map(|val| (item, val)))
		                                  .flatten()
		{
			if *value >= MAX_CLICKS {
				let item = self.first.take().unwrap();
				let value = item.remove();
				println!("First item removed on click {value:?}");
			}
		}

		// remove third menu item if requested
		if let Some((_item, value)) = self.third.as_ref().map(|item| (item, item.selected_option())) {
			if value != 0 {
				self.third.take();
			}
		}


		const LABEL_DEF: &str = "Use System Menu\0";

		let cstr = CStr::from_bytes_with_nul(LABEL_DEF.as_bytes()).unwrap();

		unsafe {
			let graphics = (*sys::API).graphics;
			(*graphics).clear?(LCDSolidColor::kColorWhite as LCDColor);

			// get width (screen-size) of text
			let text_width = (*graphics).getTextWidth?(
			                                           core::ptr::null_mut(),
			                                           cstr.as_ptr() as *const _,
			                                           LABEL_DEF.len(),
			                                           PDStringEncoding::kUTF8Encoding,
			                                           0,
			);
			// render text
			(*graphics).drawText?(
			                      cstr.as_ptr() as *const _,
			                      LABEL_DEF.len(),
			                      PDStringEncoding::kUTF8Encoding,
			                      INITIAL_X as c_int - text_width / 2,
			                      INITIAL_Y.try_into().unwrap(),
			);
		}
		Some(())
	}


	/// Event handler
	fn event(&'static mut self, event: PDSystemEvent) -> Option<()> {
		match event {
			// initial setup
			PDSystemEvent::kEventInit => unsafe {
				(*(*sys::API).display).setRefreshRate?(20.0);


				let api = CustomApi::new();


				fn callback(userdata: &mut u32) {
					println!("Check menu item clicked {userdata} times.");
					*userdata += 1;
				}
				self.first = SimpleMenuItem::new_with(api.clone(), "Check Me", Some(callback), 0).ok();


				fn change_first(state: &mut Option<&'static State>) {
					if let Some(state) = state {
						if let Some(item) = state.first.as_ref() {
							if let Some(value) = item.get_userdata() {
								item.set_title(format!("Clicked: {value}/{MAX_CLICKS}")).unwrap();
							} else {
								println!("No user-data")
							}
						} else {
							println!("No menu item")
						}
					} else {
						println!("No state")
					}
				}
				self.second =
					CheckMenuItem::new_with(api.clone(), "Change^", false, Some(change_first), None).unwrap()
					                                                                                .into();
				let second = self.second.as_ref().unwrap();
				second.set_userdata(Some(self));


				self.third = OptionsMenuItem::new_with(api.clone(), "Remove?", ["No", "Yes"], None, ()).unwrap()
				                                                                                       .into();
			},
			_ => {},
		}
		Some(())
	}
}


#[no_mangle]
/// Proxy event handler, calls `State::event`
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	static mut STATE: Option<Box<State>> = None;

	match event {
		PDSystemEvent::kEventInit => unsafe {
			// register the API entry point
			sys::API = api;

			// create game state
			if STATE.is_none() {
				STATE = Some(Box::new(State::new()));
			}
			let state = STATE.as_mut().unwrap().as_mut() as *mut State;

			// get `setUpdateCallback` fn
			let f = (*(*api).system).setUpdateCallback.expect("setUpdateCallback");
			// register update callback with user-data = our state
			f(Some(on_update), state.cast());
		},
		_ => {},
	}

	if let Some(state) = unsafe { STATE.as_mut() } {
		state.event(event).and(Some(0)).unwrap_or(1)
	} else {
		1
	}
}


/// Proxy update callback, calls `State::update`
unsafe extern "C" fn on_update(state: *mut c_void) -> i32 {
	let ptr: *mut State = state.cast();
	let state = ptr.as_mut().expect("missed state");
	state.update().and(Some(1)).unwrap_or_default()
}


// Needed for debug build
ll_symbols!();
